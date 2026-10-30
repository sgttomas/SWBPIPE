---
doc_id: OPS-BUILD-AND-RELEASE
doc_kind: engineering.release_build_guide
status: draft
created: 2026-05-04
deliverable_id: DEL-10-04
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-RELEASE-QUALITY-GATES
  - rel: implements
    to: SOW-032
---

# Build And Release Guide

## 1. Purpose

This guide defines the provider-neutral build, packaging, and release-evidence
skeleton for SWBPIPE. It gives maintainers a reproducible local path for
collecting software-quality evidence under the ruled CI and v0.1 release
postures. Publication and later signing decisions retain their own gates.

This guide is not a live CI workflow or a release publication authorization.

## 2. Current Authority Boundary

Development uses the selected hosted checks aggregated by the required Root
`harness` job. The blanket DEC-025 pre-push/fan-in sweep is retired. Do not
repeat a full local suite already covered by hosted checks. Broad checks run
on demand or for release-candidate assurance.

- the v0.1 release matrix, installer format, and signing posture are ruled
  (`DEC-057`, 2026-07-04, recorded in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; packet
  `execution/_Coordination/_DECISIONS/D-06_release_matrix_installers_publication.md`):
  macOS Apple Silicon (`aarch64-apple-darwin`) only; the Tauri `.app` bundle
  distributed as a zip archive with a published SHA-256 checksum. The owner
  re-decided signing at `D-06b` on 2026-10-23 (§9): v0.1 is signed with the
  owner's Developer ID Application identity, with the hardened runtime, and
  notarized by Apple with the ticket stapled. The checksum + the commit-bound
  `DEC-025` sweep artifact + the §8 release artifact record still bind the
  artifact to its source revision;
- no publication act occurs from this guide: packaging (§6) produces a local
  zip + checksum + §8 record only; the ruled publication target (GitHub
  Releases on the prospective public sanitized-export repository,
  `DEC-057`/`DEC-059`) does not exist yet, and any publication is a separate
  human release-authority act;
- DEC-027 records the sole human project authority as sole maintainer and
  release authority with quorum one; this does not approve a release;
- no final numerical tolerance, coverage, or performance threshold is selected.

Those numerical thresholds remain `TBD` until the applicable human decision.

## 3. Repository Baseline

The current repository has a root npm workspace manifest (`package.json` with
the `apps/desktop` workspace) and a Tauri 2 desktop shell under `apps/desktop/`
(`apps/desktop/package.json` and `apps/desktop/src-tauri/Cargo.toml`). There is
still no root Cargo workspace: Rust crates remain crate-local under `core/`,
`validation/benchmarks/`, and `apps/desktop/src-tauri/`. Python tests and
validation helpers live under `tests/` and `tools/`.

The provider-neutral readiness script therefore discovers existing manifests
instead of assuming a future workspace layout.

The browser-mode operation engine is the wasm32 build of
`core/model_operations/operation_applier` (`DEC-020` / ADR-0001): building the
desktop app for browser-mode use requires the `wasm32-unknown-unknown` Rust
target and the `wasm-bindgen` CLI at exactly the version pinned in that crate's
`Cargo.toml`, and the artifact is produced by
`npm run build:wasm --workspace apps/desktop` (generated under
`apps/desktop/src/services/wasmEngine/__generated__/`, never committed; the
script fails with explicit remediation commands when a prerequisite is
missing). The build writes the glue to a sibling temp directory and renames it
into place, so a concurrent reader never sees a half-written artifact set
(`DEC-025` F-4 rider).

```bash
python3 tools/release/check_release_readiness.py --profile skeleton
python3 tools/release/check_release_readiness.py --profile skeleton --execute
python3 tools/release/check_release_readiness.py --profile cargo
python3 tools/release/check_release_readiness.py --profile stack-witnesses
python3 tools/release/check_release_readiness.py --profile all
```

The `stack-witnesses` profile runs the six retained W1 stack witnesses at the
R/64 headroom leg, one process each; the release sweep runs it after the crate
sweep (DEL-04-08 Design `stack-headroom-repair.md`).

Without `--execute`, the script performs path checks and prints the local
commands it would run. With `--execute`, it runs only local commands from the
selected profile. It does not use network services, release signing,
publication credentials, or shell command evaluation.

## 4. Reproducibility Inputs

A release-evidence record should capture:

| Field | Required value |
|---|---|
| Source revision | Git commit hash or explicit working-tree state. |
| Working tree state | Clean, or list of changed files if evidence is pre-commit. |
| Runtime versions | Python, Cargo/Rust, Node/package tooling where applicable. |
| Commands run | Exact command, profile, host OS, and pass/fail result. |
| Artifacts reviewed | Docs, schemas, binaries, packages, manifests, or reports. |
| Validation status | Applicable release-quality gate outcome or waiver. |
| Data boundary status | Protected-content, private-data, and real-secret scan result. |
| Known limitations | Open risks and unresolved `TBD` decisions. |
| Human gate | Maintainer or project-authority acceptance record, if any. |

Working-tree evidence may support review, but release publication should bind
to a committed source revision unless a human release authority records an
exception.

## 5. Local Check Profiles

The local readiness script defines these provider-neutral profiles:

| Profile | Scope | Intended use |
|---|---|---|
| `skeleton` | Documentation path checks, dependency schema validation, and focused script tests. | Fast smoke check for the release skeleton. |
| `python` | Python contract, governance, and validation tests. | Local Python gate before review. |
| `security` | Security/privacy tests. | Local privacy and redaction gate. |
| `cargo` | `cargo test` for discovered crate manifests. | Local Rust crate gate without a root workspace assumption. |
| `all` | Union of available local profiles. | Maintainer pre-release dry run or local full run. |

These profiles are available for focused local diagnosis or release verification;
they are not a requirement to run every profile on every development change.

### 5.1 Release-candidate sweep

`python3 tools/release/run_evidence_sweep.py --execute` remains available for
macOS release candidates whose packaging authenticity chain uses its artifact
(§8). It is not a development merge gate. Do not commit routine sweep logs or
run an evidence-only closeout PR. Keep release artifacts where their consumers
need them. Avoid concurrent commands that rebuild the same wasm artifacts.

## 6. Packaging Skeleton

The packaging checklist for a release candidate:

1. Confirm source revision and working-tree state.
2. Run applicable local readiness profiles.
3. Confirm release quality gates in `docs/RELEASE_QUALITY_GATES.md`.
4. Confirm protected-content, private-data, and real-secret scan disposition.
5. Prepare release notes from `docs/RELEASE_NOTES_TEMPLATE.md`; release notes
   carry the §8.2 install note. The 0.1.0 draft is `docs/RELEASE_NOTES_v0.1.0_DRAFT.md`;
   end-user installation steps are in the user guide §3.
6. Record known limitations, unresolved `TBD` decisions, and human gate state.
7. If binaries or installers are produced, record package path, target, build
   command, checksum, signing/notarization state, and publication state via
   the §8 release-artifact record.

### 6.1 Ruled v0.1 Package Path (`DEC-057`)

- **Matrix:** macOS Apple Silicon (`aarch64-apple-darwin`) only. Windows and
  Linux enter only through the evidence-gated matrix-expansion rider
  (packaged build + recorded packaged-run smoke + §8 record per platform).
- **Bundle:** `apps/desktop/src-tauri/tauri.conf.json` enables the bundler
  (`bundle.active: true`, explicit `targets: ["app"]`) with the SWB mark
  (the website's mark; owner, RT-V8 P1) as a real multi-resolution `.icns`
  (`apps/desktop/src-tauri/icons/icon.icns`, with `icon.png`; regenerate
  deterministically from `apps/desktop/SWBPIPE_logo.png` with
  `python3 tools/release/generate_app_icon.py`, then list the new bytes in
  `tools/release/reviewed_binaries.json`). Build the signed `.app`
  with:

  ```bash
  python3 tools/release/build_signed_app.py --execute --supplier-archive <verified-stock-archive>
  ```

  It requires one explicit `--supplier-archive`, `--supplier-binary` or
  `--supplier-download` source. The latter is build-time acquisition only; no
  runtime or first-use download is selected. A private generated resource
  overlay carries the original pinned supplier, manifest and licence/NOTICE
  payloads. The wrapper runs `npm run tauri -- build --bundles app --target
  aarch64-apple-darwin --config <private-resource-overlay>` in `apps/desktop` with
  `APPLE_SIGNING_IDENTITY="Developer ID Application"`, which names the
  keychain's certificate by its kind. It first refuses unless the keychain
  holds exactly one valid Developer ID Application identity, so codesign never
  meets an ambiguous match, whose diagnostic would list identity names. It
  removes Tauri's notarization variables from the build environment, and
  masks identity names, team identifiers and certificate hashes in the output
  it forwards. No identity, team identifier or certificate enters the
  repository. `tauri.conf.json` sets `bundle.macOS.hardenedRuntime: true`. The
  wrapper retains its private preparation until the original build completes,
  then verifies actual `Contents/Resources` binary and licensing bytes before
  returning success. Ordinary `bundle.resources` preserve the supplier's
  original signature/stock digest; no `externalBin` re-signing or new entitlement
  is selected. See DEL-16-06 Design `agent-supplier-packaging.md` for this input
  contract and remaining compatibility/isolation/distribution duties. The
  app needs no entitlements: WebKit runs its web content, including the
  WebAssembly engine, in its own processes. The native smoke's results were
  unchanged against a Developer ID signed, hardened-runtime binary (#1536). Notarize the built bundle
  separately with a notarytool keychain profile:

  ```bash
  python3 tools/release/notarize_app.py --execute --keychain-profile swbpipe-notary
  ```

  It submits a `ditto` zip of the bundle with `xcrun notarytool submit
  --keychain-profile <name> --wait`, then staples and validates the ticket.
  It prints fixed outcome categories, exit codes and the submission id only;
  tool output, which can name the account or team, is never echoed.
  The owner creates the profile once, privately, with `xcrun notarytool
  store-credentials swbpipe-notary --apple-id <Apple ID> --team-id <team>`,
  which prompts for an app-specific password and keeps it in the login
  keychain. Only the profile name appears in commands.

- **Artifact shape:** the `.app` zipped with a published SHA-256 checksum,
  produced by the deterministic packaging entrypoint:

  ```bash
  python3 tools/release/package_release_artifact.py            # dry-run
  python3 tools/release/package_release_artifact.py --execute \
      --sweep-artifact validation/evidence/sweeps/SWEEP_<utc>_<commit12>.json
  ```

  It refuses a bundle that is not Developer ID signed, hardened, timestamped
  and notarized with a stapled ticket, unless `--allow-unsigned` is given for
  a rehearsal record.

  The zip is deterministic where feasible (sorted entries, commit-derived
  UTC timestamps, unix permissions and symlinks preserved, no extra fields);
  the zip and `.zip.sha256` land under `dist/release/` (untracked), and the
  §8 record lands under `validation/evidence/release_artifacts/`.
- **Signing:** Developer ID signed with the hardened runtime and notarized,
  with the ticket stapled (`D-06b`, owner, 2026-10-23; §9). Certificates,
  passwords and API keys stay in the local keychain (§7).
- **Publication:** no publication act from this path. The ruled target is
  GitHub Releases on the prospective public sanitized-export repository
  (`DEC-057`/`DEC-059`); until it exists, artifacts are recorded locally per
  §8 and distributed directly by the owner.

Producing a package under this path is packaging mechanics, not a release:
an actual release additionally requires the `D-20` scan record, gate
records, and the human release authority's acceptance.

### 6.2 Independent public repository and signed v1.0 scan (RT-V8)

The owner selected public `sgttomas/SWBPIPE` as SWBPIPE's home and created it
with an MIT license. Owner decisions of 2026-10-29: "Split repo replaces RT-V8"
and "Push now, sign before v1.0". The verified history import and migration PRs
may be published before the scan is signed. This replaces the sanitized-export
pipeline, its include/exclude lists and its pre-push signature prerequisite.
Repository settings and required-check changes remain owner-controlled.

Before v1.0 release, bind the complete tracked release-candidate tree to its
Git revision and run the D-20/DEC-058 scan. For example, from this repository,
using a new output directory outside every checkout:

```bash
# Set SCAN_OUT to an owner-selected new directory outside the checkout.
mkdir -p "$SCAN_OUT/tree" "$SCAN_OUT/records"
git rev-parse HEAD > "$SCAN_OUT/revision.txt"
git archive --format=tar HEAD | tar -xf - -C "$SCAN_OUT/tree"
python3 tools/release/run_release_candidate_scan.py --execute --candidate v1.0 \
  --artifact-class "AC-4=$SCAN_OUT/tree" \
  --not-applicable "AC-1=no separate package in this repository-tree candidate" \
  --not-applicable "AC-2=no separate report in this repository-tree candidate" \
  --not-applicable "AC-3=no installer in this repository-tree candidate" \
  --not-applicable "AC-5=tracked examples and fixtures included in AC-4" \
  --not-applicable "AC-6=tracked release documents included in AC-4" \
  --output-dir "$SCAN_OUT/records"
# Owner only: supply the complete private list without publishing its values.
python3 tools/validation/validate_private_terms.py \
  --base bda65d1adf27f23120ceaaac64a521dc7660f09a --head HEAD \
  --from-host --terms-file /owner/private/full-term-list --require-terms
```

Those not-applicable reasons describe only the repository-tree candidate.
Separately shipped packages, reports, installers and release notes need their
own class entries and bound artifacts; repository verification never covers
an unnamed release artifact. Inventory bytes/modes and source revision must
match the exact candidate being signed. Original-to-imported history and the
historical pricing/archive inputs are recorded in `reference/migration/`.

Review and disposition every finding. The earlier export-scope scan documented
62 blocking lint and 27,460 provenance findings in 97 files; those were findings,
not clearance. New full-repository dry runs name their exact candidate, count
and scope. The original reviewed binary hashes, trusted-schema manifest and
explicit P7 provenance bindings remain scan policy inputs. Their presence does
not waive a finding on changed or unlisted bytes; new exemptions require review.

Only the owner completes the emitted record's `owner_sign_off`: `signed`,
`signatory`, `signed_utc`, `per_finding_dispositions_recorded` and
`acceptance_statement`. Attach the full private-term scan evidence and retain
per-finding dispositions. Unresolved blocking findings halt v1.0 release. An
unsigned tooling record, CI pass or repository merge supplies no signature,
clearance, engineering acceptance or release authority (F-PIP-3). Packaging,
notarization and the remaining release gates still apply.

## 7. Hosted verification

The Root `.github/workflows/governance-harness.yml` calls selected product
workflows and produces the single required `harness` result. Piping selection
is defined by `tools/hosted-ci-routing.json` and the numerical/Python selectors.
Changed numerical crates and consumers get focused oracles; selected Python
contracts, desktop checks and a core browser journey cover their changed inputs.
Broad matrices are manual. Exact successful-result reuse is permitted only when
inputs and environment match; a reused pass identifies its original run.

See [CI_STRATEGY.md](CI_STRATEGY.md). Release publication and actual product
acceptance remain separate owner decisions.

### 7.1 Playwright Browser Provisioning Policy

A future authorized CI job must install dependencies from the committed
lockfile with `npm ci` at the repository root, then
provision the Playwright-managed Chromium that matches the installed
`@playwright/test` package:

```bash
npx playwright install --with-deps chromium
```

That command is the supported-Linux-image basis. A different authorized image
must declare its equivalent operating-system dependency preparation. The
Playwright configs launch that pinned Chromium by default, locally as in CI;
they do not fall back to an installed Google Chrome. An explicit
`PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` is an exceptional declared override, not
the provider-neutral default, and a missing override fails the launch.

Run both browser lanes with `CI=true`. Each live config then defaults to one
worker; a positive integer `PLAYWRIGHT_WORKERS` may override that posture only
when backed by recorded evidence. CI mode also prevents reuse of an existing
web server. Both lanes retain traces on failure. Those traces are diagnostic
artifacts and do not, by themselves, constitute release evidence or human
acceptance.

Hosted CI must not receive private project data, private rule packs, private
material/component libraries, protected standards content, signing secrets, or
publishing credentials unless a later security and release-governance decision
explicitly authorizes that handling.

## 8. Release Artifact Record

Every release candidate record should identify:

- source revision and evidence profile;
- changed packages and deliverables;
- checks and gate outcomes;
- package or artifact paths, if generated;
- checksums for generated artifacts, if any;
- validation status and known limitations;
- data-boundary and professional-boundary notices;
- human review or waiver record.

### 8.1 Record Emitter (`DEC-057` Mechanics)

`python3 tools/release/package_release_artifact.py --execute` emits the
machine-readable record
`validation/evidence/release_artifacts/RELEASE_ARTIFACT_<utc>_<commit12>[-dirty].json`
alongside the zip + checksum it produces. The record binds the fields above
to the current commit hash and runtime versions and adds the `DEC-057`
ruled-shape fields: release matrix (`aarch64-apple-darwin`), installer
format (Tauri `.app` zip), build command, signing state, publication state,
and the human-review placeholder (publication requires the human release
authority's acceptance; the emitter never fills that field). The signing
state is what `codesign --verify --deep --strict`, `codesign -d`, `xcrun
stapler validate` and `spctl --assess --type execute` report. They run on the
bundle and on a `ditto` extraction of the zip, as a user receives it. The
record keeps their exit codes, whether the signature satisfies the
Apple-anchored Developer ID Application requirement, the hardened runtime,
the secure timestamp, the stapled ticket and the Gatekeeper source as a fixed
label. It keeps no authority name, team identifier, origin line or local rule
label. If the extracted bundle does not verify, the zip and checksum are
removed and no record is written.

### 8.2 Authenticity Chain And Install Note (`DEC-057`, `D-06b`)

The Developer ID signature and stapled notarization identify the developer
to macOS. Three bound surfaces, recorded together in the emitted record's
`authenticity_chain`, bind the artifact to its source revision and evidence:

1. the artifact's SHA-256 checksum, published beside it as `<zip>.sha256`;
2. the commit-bound `DEC-025` sweep artifact passed via `--sweep-artifact`
   — the chain is `verified` only when the record's commit is clean and
   identical to a passing, clean-tree sweep summary's bound commit, or
   trails it only by evidence-only closeout commits (deltas confined to
   `validation/evidence/`, recorded path-by-path in the record);
3. this §8 release artifact record binding both to the source revision.

Release notes and any distribution surface carry the install note, which
the emitter records verbatim in the record of a signed, notarized bundle:

> This build is signed with a Developer ID Application certificate and
> notarized by Apple, with the notarization ticket stapled (D-06b owner
> decision of 2026-10-23). Before opening it, verify the artifact's SHA-256
> checksum against the published .sha256 file; the release artifact record
> binds that digest to the source commit and its commit-bound DEC-025
> evidence-sweep artifact. macOS Gatekeeper then opens it as an identified,
> notarized app.

A rehearsal record made with `--allow-unsigned` carries the former
unsigned-install caveat instead and is not a v0.1 distribution record.

Release labels describe software maturity and validation evidence.

## 9. Open Decisions

- Development assurance uses selected hosted checks under Root `AGENTS.md`;
  the old DEC-025 blanket merge sweep is superseded.
- Decided 2026-07-04 (`DEC-057`, D-06 Option O-A): v0.1 release matrix is
  macOS Apple Silicon (`aarch64-apple-darwin`) only; installer format is the
  Tauri `.app` bundle zipped with a published SHA-256 checksum (§6.1);
  signing/notarization was none for v0.1, re-decided at `D-06b` (next
  item); publication target is
  GitHub Releases on the prospective public sanitized-export repository,
  with local §8 recording and direct owner distribution until it exists.
  Windows/Linux matrix growth is evidence-gated (new register rows).
- Decided 2026-10-23 (`D-06b`, re-deciding `DEC-057`'s unsigned v0.1 posture
  and moving `DEC-089`'s policy-only Developer ID target into v0.1). The
  owner: "Oh I'm already an Apple Developer. There's already an App ID for
  this application. Proceed with signing and notarizing." v0.1 is Developer
  ID signed with the hardened runtime and notarized with a stapled ticket
  (§6.1, §8.2), under the App ID `DEC-106` records (`com.chirality.swbpipe`).
- TBD: coverage, performance, tolerance, and permitted-variance thresholds.
- Decided by DEC-027: sole human maintainer and release authority, quorum one.
  Artifact retention remains a separate delivery matter.
- Decided: desktop project container is the multi-member archive per the
  PKG-17 contracts (`DEC-028`, 2026-06-11), named `.opsproj` /
  "OpenPipeStress Project Package" by the `DEC-057` naming rider.
