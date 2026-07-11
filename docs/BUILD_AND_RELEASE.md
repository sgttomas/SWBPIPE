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
skeleton for OpenPipeStress. It gives maintainers a reproducible local path for
collecting software-quality evidence before a future CI provider, release
matrix, signing process, and publishing workflow are selected.

This guide is not a live CI workflow, release publication authorization,
professional engineering approval, legal opinion, certification, sealing,
standards-body endorsement, or code-compliance determination.

## 2. Current Authority Boundary

The current implementation lane is provider-neutral:

- hosted CI is deferred by human ruling (`DEC-025`, 2026-06-11, recorded in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12): the five-surface local
  evidence sweep (§5.1) is the commit-bound merge gate for parallel agent
  development branches; hosted CI is re-decided at the named follow-up
  `D-05b` (public sanitized-export CI, prepared with D-06). GitHub Actions on
  the private monorepo remains prohibited absent an explicitly recorded §7
  private-data-handling authorization;
- no `.github/` or other live workflow file is created by this deliverable;
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
- no final numerical tolerance, coverage, performance, or maintainer-quorum
  threshold is selected.

Those decisions remain `TBD` until a human project authority records them.

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

The final CI job names, matrix, required thresholds, and failure policy remain
`TBD`. Future provider-specific workflows should call the same local script or
an equivalent command plan so local and hosted evidence stay comparable.

### 5.1 Five-Surface Evidence Sweep (DEC-025 Merge Gate)

The deterministic local evidence entrypoint is:

```bash
python3 tools/release/run_evidence_sweep.py            # dry-run: print the plan
python3 tools/release/run_evidence_sweep.py --execute  # run the sweep
```

It runs the five evidence surfaces sequentially, in this fixed F-4-safe
order, failing fast and recording later surfaces as `not_run`:

| # | Surface | Command basis |
|---|---|---|
| 1 | Rust crate sweep | `python3 tools/release/check_release_readiness.py --profile cargo --execute` |
| 2 | Python tests | `python3 -m pytest -q tests` |
| 3 | Desktop Vitest (wasm engine built first) | `npm run build:wasm:desktop` then `npm run test:desktop` |
| 4 | Playwright e2e | `npm run test:e2e:desktop` |
| 5 | Desktop production build | `npm run build:desktop` |

Each execute run writes a machine-readable summary artifact to
`validation/evidence/sweeps/SWEEP_<utc>_<commit12>[-dirty].json` containing
the bound commit hash, branch, working-tree deltas, runtime versions,
per-command exit codes and durations, and the overall pass/fail status. The
exit code is `0` only when all five surfaces pass.

Merge-gate role (`DEC-025`): the sweep is the required pre-push/fan-in
evidence for every parallel agent development branch. The recommended gate
pattern is:

1. commit the completed tranche;
2. run `python3 tools/release/run_evidence_sweep.py --execute` at the clean
   committed HEAD, so the summary binds to that commit hash;
3. commit the summary artifact as an evidence-only closeout commit and push.

A dirty-tree sweep is recorded as working-tree evidence (the summary lists
the deltas and the filename carries a `-dirty` suffix); per §4 it may support
review, but the merge gate binds to a clean committed revision.

The surfaces must not run concurrently with each other or with a second
sweep on the same checkout: surfaces 3 and 4 rebuild the shared wasm engine
artifact, and the cargo sweep saturates the same cores. The atomic wasm-build
swap (§3) removes the half-written-artifact hazard, not the contention.

A green sweep is development evidence only. It is not a release claim,
release publication authorization, professional engineering approval,
certification, sealing, authentication, or code-compliance determination.

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
  OpenPipeStress mark as a real multi-resolution `.icns`
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

## 7. Future CI Mapping

Hosted CI is deferred (`DEC-025`; re-decided at `D-05b` with D-06). When a
hosted location is later selected, the provider workflow should map to these
stable phases, with the five-surface sweep (§5.1) as the local command basis
so local and hosted evidence stay comparable:

| Phase | Provider-neutral command basis |
|---|---|
| Repository sanity | `python3 tools/release/check_release_readiness.py --profile skeleton --execute` |
| Python/schema contracts | `python3 tools/release/check_release_readiness.py --profile python --execute` |
| Security/privacy checks | `python3 tools/release/check_release_readiness.py --profile security --execute` |
| Rust crates | `python3 tools/release/check_release_readiness.py --profile cargo --execute` |
| Release candidate review | Release notes, gate record, scan record, and human acceptance record. |

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

Release labels describe software maturity and validation evidence. They do not
approve a project-specific piping calculation, authenticate a user rule pack,
or replace competent professional review.

## 9. Open Decisions

- Decided 2026-06-11 (`DEC-025`): hosted CI deferred; the five-surface local
  sweep (§5.1) is the commit-bound merge gate; hosted workflow location is
  re-decided at `D-05b` (public sanitized-export CI, prepared with D-06).
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
- TBD: maintainer quorum and release authority.
- Decided: desktop project container is the multi-member archive per the
  PKG-17 contracts (`DEC-028`, 2026-06-11), named `.opsproj` /
  "OpenPipeStress Project Package" by the `DEC-057` naming rider.
