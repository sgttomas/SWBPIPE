# Datasheet: DEL-10-04 Build, packaging, and CI/CD pipeline

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-10-04 |
| Package ID | PKG-10 |
| Package | Build, Packaging, API, and Interoperability |
| Type | CI_CD_CHANGE |
| Scope item | SOW-032 |
| Objectives | OBJ-008; OBJ-009 |
| Context envelope | L |
| Current setup state | SEMANTIC_READY |

## Attributes

| Attribute | Value |
|---|---|
| Deliverable purpose | Define the setup basis for reproducible builds, packaging, and CI/CD workflow work. |
| Anticipated implementation artifacts | CI workflows; packaging scripts; release notes template. |
| Runtime/UI baseline | Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI where GUI-facing. |
| Test gate baseline | Cargo tests; Vitest; Playwright; validation gates; protected-content/provenance gates. |
| Desktop packaging baseline | Tauri-supported macOS, Windows, and Linux targets. |
| Release-quality boundary | Development/release automation evidence only; no engineering certification or code-compliance claim. |

## Conditions

| Condition | Status |
|---|---|
| CI provider | TBD - human/project authority decision required. |
| Platform release matrix | TBD - this setup run does not finalize exact OS/architecture coverage. |
| Coverage thresholds | TBD - no final numerical thresholds are set by this setup run. |
| Performance thresholds | TBD - no final timing/size thresholds are set by this setup run. |
| Dependency versions | TBD - exact versions remain implementation-level decisions. |
| Release signing process | TBD - governance/release authority decision required. |

## Construction

This setup artifact constrains later implementation work without creating implementation files. A future authorized DEL-10-04 implementation pass may draft CI workflows, packaging scripts, and release-note templates only after human authority confirms the CI provider, release matrix, thresholds, and write scope.

The build and packaging pipeline must preserve the OpenPipeStress boundaries:

- public automation must not introduce protected standards text, protected data tables, proprietary vendor data, or private project/rule-pack data;
- test and release gates must keep mechanics verification separate from user rule checks and professional approval;
- release artifacts must disclose validation status, limitations, data-boundary constraints, and professional-responsibility limitations;
- adapters, plugins, and packaging steps must not bypass unit, provenance, diagnostic, or privacy checks.

## References

- `INIT.md` - bootstrap and data-boundary posture.
- `AGENTS.md` - Type 2 sealed-deliverable dispatch rules.
- `docs/DIRECTIVE.md` - founding intent, stop rules, and no-certification boundary.
- `docs/CONTRACT.md` - invariant catalog, especially OPS-K-IP, OPS-K-DATA, OPS-K-PRIV, OPS-K-AUTH, and OPS-K-AGENT.
- `docs/SPEC.md` - architecture layers, reporting/audit requirements, V&V mechanics, and agentic implementation mechanics.
- `docs/VALIDATION_STRATEGY.md` - benchmark families and release gate expectations.
- `docs/PRD.md` - platform criteria and release milestones, including PRD 19.2, 19.3, and 22.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.4 scope, architecture basis, decisions, and open issues.
- `docs/_Registers/Deliverables.csv` - row DEL-10-04.
- `docs/_Registers/ScopeLedger.csv` - row SOW-032.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-10-04.

## TBD and Human-Ruling Slots

- TBD: CI provider.
- TBD: exact supported platform/release matrix.
- TBD: coverage and performance thresholds.
- TBD: release signing/notarization/publishing policy.
- TBD: exact packaging artifact names and distribution channels.

