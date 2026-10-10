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
  distributed as a zip archive with a published SHA-256 checksum; no signing
  or notarization for v0.1 (re-decided at `D-06b`), with authenticity carried
  by the checksum + the commit-bound `DEC-025` sweep artifact + the §8
  release artifact record and the unsigned-install caveat;
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
python3 tools/release/check_release_readiness.py --profile all
```

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
   must carry the §8 unsigned-install caveat while `DEC-057`'s unsigned
   posture stands.
6. Record known limitations, unresolved `TBD` decisions, and human gate state.
7. If binaries or installers are produced, record package path, target, build
   command, checksum, signing/notarization state, and publication state via
   the §8 release-artifact record.

### 6.1 Ruled v0.1 Package Path (`DEC-057`)

- **Matrix:** macOS Apple Silicon (`aarch64-apple-darwin`) only. Windows and
  Linux enter only through the evidence-gated matrix-expansion rider
  (packaged build + recorded packaged-run smoke + §8 record per platform).
- **Bundle:** `apps/desktop/src-tauri/tauri.conf.json` enables the bundler
  (`bundle.active: true`, explicit `targets: ["app"]`) with the invented
  SWBPIPE mark as a real multi-resolution `.icns`
  (`apps/desktop/src-tauri/icons/icon.icns`; regenerate deterministically
  with `python3 tools/release/generate_app_icon.py`). Build the `.app` with:

  ```bash
  cd apps/desktop && npm run tauri -- build --bundles app
  ```

- **Artifact shape:** the `.app` zipped with a published SHA-256 checksum,
  produced by the deterministic packaging entrypoint:

  ```bash
  python3 tools/release/package_release_artifact.py            # dry-run
  python3 tools/release/package_release_artifact.py --execute \
      --sweep-artifact validation/evidence/sweeps/SWEEP_<utc>_<commit12>.json
  ```

  The zip is deterministic where feasible (sorted entries, commit-derived
  UTC timestamps, unix permissions and symlinks preserved, no extra fields);
  the zip and `.zip.sha256` land under `dist/release/` (untracked), and the
  §8 record lands under `validation/evidence/release_artifacts/`.
- **Signing:** none for v0.1 (`DEC-057`); re-decided at `D-06b` before any
  R5 "Signed releases" deliverable claim. Signing secrets stay local (§7).
- **Publication:** no publication act from this path. The ruled target is
  GitHub Releases on the prospective public sanitized-export repository
  (`DEC-057`/`DEC-059`); until it exists, artifacts are recorded locally per
  §8 and distributed directly by the owner.

Producing a package under this path is packaging mechanics, not a release:
an actual release additionally requires the `D-20` scan record, gate
records, and the human release authority's acceptance.

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
lockfile with `npm ci` in the `projects/chirality-piping` workspace, then
provision the Playwright-managed Chromium that matches the installed
`@playwright/test` package:

```bash
npx playwright install --with-deps chromium
```

That command is the supported-Linux-image basis. A different authorized image
must declare its equivalent operating-system dependency preparation. CI must
not rely on the macOS Google Chrome fallback in the Playwright configs. An
explicit `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` is an exceptional declared
override, not the provider-neutral default.

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
format (Tauri `.app` zip), build command, signing state (`unsigned`,
re-decision `D-06b`), publication state, and the human-review placeholder
(publication requires the human release authority's acceptance; the emitter
never fills that field).

### 8.2 Authenticity Chain And Unsigned-Install Caveat (`DEC-057`)

v0.1 artifacts are unsigned, so authenticity is carried by three bound
surfaces, recorded together in the emitted record's `authenticity_chain`:

1. the artifact's SHA-256 checksum, published beside it as `<zip>.sha256`;
2. the commit-bound `DEC-025` sweep artifact passed via `--sweep-artifact`
   — the chain is `verified` only when the record's commit is clean and
   identical to a passing, clean-tree sweep summary's bound commit, or
   trails it only by evidence-only closeout commits (deltas confined to
   `validation/evidence/`, recorded path-by-path in the record);
3. this §8 release artifact record binding both to the source revision.

Release notes and any distribution surface must carry the unsigned-install
caveat (the emitter records it verbatim in every record):

> This build is not code-signed or notarized (DEC-057: v0.1 ships unsigned;
> signing/notarization is re-decided at D-06b). macOS Gatekeeper will
> quarantine the downloaded app. Before opening it, verify the artifact's
> SHA-256 checksum against the published .sha256 file and the commit-bound
> DEC-025 evidence-sweep artifact referenced in the release artifact record;
> then open the app explicitly (Control-click > Open, or remove the
> quarantine attribute). Authenticity is carried by the checksum + the
> commit-bound sweep artifact + the release artifact record, not by an OS
> code signature.

Release labels describe software maturity and validation evidence.

## 9. Open Decisions

- Development assurance uses selected hosted checks under Root `AGENTS.md`;
  the old DEC-025 blanket merge sweep is superseded.
- Decided 2026-07-04 (`DEC-057`, D-06 Option O-A): v0.1 release matrix is
  macOS Apple Silicon (`aarch64-apple-darwin`) only; installer format is the
  Tauri `.app` bundle zipped with a published SHA-256 checksum (§6.1);
  signing/notarization is none for v0.1 with checksum + commit-bound sweep +
  §8 record authenticity and the unsigned-install caveat (§8.2), re-decided
  at `D-06b` before any R5 "Signed releases" claim; publication target is
  GitHub Releases on the prospective public sanitized-export repository,
  with local §8 recording and direct owner distribution until it exists.
  Windows/Linux matrix growth is evidence-gated (new register rows).
- TBD: coverage, performance, tolerance, and permitted-variance thresholds.
- Decided by DEC-027: sole human maintainer and release authority, quorum one;
  future signing/notarization beyond the unsigned v0.1 posture is re-decided
  at D-06b. Artifact retention remains a separate delivery matter.
- Decided: desktop project container is the multi-member archive per the
  PKG-17 contracts (`DEC-028`, 2026-06-11), named `.opsproj` /
  "OpenPipeStress Project Package" by the `DEC-057` naming rider.
