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

- no CI provider is selected;
- no `.github/` or other live workflow file is created by this deliverable;
- no installer, signed binary, notarized package, attestation, or publication
  target is generated;
- no final OS/architecture release matrix is selected;
- no final numerical tolerance, coverage, performance, or maintainer-quorum
  threshold is selected.

Those decisions remain `TBD` until a human project authority records them.

## 3. Repository Baseline

The current repository does not have a root Cargo workspace, root JavaScript
package, or desktop shell package manifest. Rust crates are currently
crate-local under `core/` and `validation/benchmarks/`. Python tests and
validation helpers live under `tests/` and `tools/`.

The provider-neutral readiness script therefore discovers existing manifests
instead of assuming a future workspace layout.

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

## 6. Packaging Skeleton

The current packaging skeleton is a checklist, not a package build:

1. Confirm source revision and working-tree state.
2. Run applicable local readiness profiles.
3. Confirm release quality gates in `docs/RELEASE_QUALITY_GATES.md`.
4. Confirm protected-content, private-data, and real-secret scan disposition.
5. Prepare release notes from `docs/RELEASE_NOTES_TEMPLATE.md`.
6. Record known limitations, unresolved `TBD` decisions, and human gate state.
7. If binaries or installers are later produced, record package path, target,
   build command, checksum, signing/notarization state, and publication state.

Desktop packaging is expected to follow the accepted Tauri-supported
macOS/Windows/Linux architecture baseline when a GUI package exists. Exact
target triples, installer formats, signing identities, notarization process,
and publication destinations remain `TBD`.

## 7. Future CI Mapping

When a CI provider is selected, the provider workflow should map to these
stable phases:

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

Release labels describe software maturity and validation evidence. They do not
approve a project-specific piping calculation, authenticate a user rule pack,
or replace competent professional review.

## 9. Open Decisions

- TBD: CI provider and hosted workflow location.
- TBD: final supported OS/architecture release matrix.
- TBD: installer/package formats.
- TBD: signing, notarization, checksum publication, and release attestation.
- TBD: coverage, performance, tolerance, and permitted-variance thresholds.
- TBD: maintainer quorum and release authority.
- TBD: final package/container format for desktop project files.
