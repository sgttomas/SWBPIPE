# Guidance: DEL-10-04 Build, packaging, and CI/CD pipeline

## Purpose

This guidance explains how to interpret the DEL-10-04 setup artifacts. The deliverable exists to prepare a bounded future implementation path for reproducible builds, packaging, and CI/CD while keeping release engineering, interoperability, data-boundary, and professional-responsibility constraints visible.

## Principles

- Treat this setup kit as draft/proposal evidence until a human review gate accepts it.
- Keep build automation reproducible and auditable: a future release should be traceable to source revision, build inputs, test results, validation status, and known limitations.
- Preserve SCA-001 architecture baselines: Rust core/application services, Tauri 2 desktop shell where GUI-facing, TypeScript/React/Vite GUI where applicable, schema-first envelopes, and layered software-quality gates.
- Leave unresolved implementation choices as `TBD` unless a cited human ruling resolves them.
- Do not let CI, packaging, adapters, or plugins bypass unit safety, provenance, diagnostics, private-data handling, or protected-content controls.
- Do not present a passing build, passing test suite, packaged desktop app, or release label as engineering certification or code compliance.

## Considerations

DEL-10-04 is a large context envelope item. Future implementation may need to split into smaller tasks if it expands beyond one bounded change. Natural split points include CI workflow skeleton, desktop packaging skeleton, release-note template, signing/publishing policy, protected-content/provenance gates, and release-checklist automation.

The accepted baseline names Cargo, Vitest, Playwright, validation, and protected-content/provenance gates. It does not finalize the CI host, coverage thresholds, performance thresholds, installer formats, signing identities, release publishing destinations, dependency versions, or exact platform/architecture matrix.

## Trade-offs

- A minimal CI skeleton can provide early feedback, but final release gating needs human decisions about provider, supported platforms, and thresholds.
- Broad platform coverage improves availability, but it increases signing, notarization, packaging, and validation burden.
- Strict gates reduce release risk, but thresholds must be defensible and should not be invented before solver, GUI, and validation maturity are known.
- Packaging convenience must not weaken local-first privacy or protected-data controls.

## Examples

TBD. This setup run does not create example workflow files, packaging scripts, release templates, or implementation snippets because the write scope is limited to deliverable-local setup documentation and registers.

## Human-Ruling Queue

- TBD: CI provider and hosting model.
- TBD: supported OS and architecture release matrix.
- TBD: installer/package formats per supported platform.
- TBD: coverage and performance thresholds.
- TBD: signing, notarization, checksum, and release publishing policy.
- TBD: whether release notes template belongs in this deliverable or a later release-governance deliverable.

## Conflict Table (for human ruling)

No source conflicts were identified during this setup pass. The unresolved items above are authority gaps, not conflicts.

