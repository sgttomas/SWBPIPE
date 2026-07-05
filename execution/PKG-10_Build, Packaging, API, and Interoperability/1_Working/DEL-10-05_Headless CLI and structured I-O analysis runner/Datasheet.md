# Datasheet: DEL-10-05 Headless CLI and structured I/O analysis runner

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-10-05 | `_CONTEXT.md` |
| Package ID | PKG-10 | `_CONTEXT.md` |
| Package | Build, Packaging, API, and Interoperability | `_CONTEXT.md` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
| Lifecycle target for setup | SEMANTIC_READY | `skills/semantic-matrix-build/SKILL.md` |
| Current production posture | Stable local `openpipestress-runner` CLI policy implemented for the bounded DEC-065 solve/validate/stub surface; release packaging and public transport remain outside this deliverable record | `DEC-065`; TP-RUNNER-015 |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary purpose | Headless or structured I/O solve execution for early solver verification, regression, and automation before full GUI maturity | `docs/_Registers/ScopeLedger.csv` row SOW-054; `docs/_Registers/Deliverables.csv` row DEL-10-05 |
| Required alignment | Schema-first command/query/job envelopes and result exports | `docs/_Registers/Deliverables.csv` row DEL-10-05; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 AB-00-03 |
| Reproducibility basis | Deterministic, unit-safe model data flow; canonical JSON and JCS-compatible hashing where JSON payload hashes are used | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Diagnostics basis | Result envelopes and diagnostics carry machine-readable warning/error information and must not claim certification or code compliance | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 AB-00-06; `docs/CONTRACT.md` invariant OPS-K-AUTH-1 |
| Build/automation relationship | Must remain compatible with reproducible build, packaging, and CI/CD workflows; exact CI provider and release matrix are TBD | `docs/_Registers/ScopeLedger.csv` row SOW-032; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Public/private boundary | No protected standards data, proprietary catalog values, private rule packs, or private project data are public defaults | `INIT.md`; `docs/DIRECTIVE.md` sections 3-4; `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-PRIV-1 |

## Conditions

| Condition | Status |
|---|---|
| Exact CLI command names | Settled for the local runner surface: `openpipestress-runner` with verbs `solve`, `validate-input`, `export-results`, `run-benchmark`, and `run-regression` |
| Exact structured input schema fields | Settled for the TP-RUNNER-015 local CLI input wrapper: `request`, optional `solve.preview_model`, and optional `rule_check_aggregate`; future persisted-project input remains separate |
| Public API transport | TBD per SCA-001 remaining-TBD boundary |
| External import/export format list | TBD; schema-first JSON envelopes are the baseline |
| CI provider and coverage thresholds | TBD; release gates remain future implementation detail |
| Process/network/filesystem policy | Settled by `DEC-065`: single foreground local process; no network, daemon, telemetry, hidden filesystem mutation, repository-default private-data write, user-home scanning, or direct SQL/SQLite bypass; stdout JSON default with explicit `--output` path allowed |
| Protected-data status | No protected standards text, tables, examples, material allowables, SIF/flexibility tables, proprietary values, or certification claims introduced here |

## Construction

The current bounded implementation defines the stable local CLI runner surface,
not a release package or public transport. The implemented `solve` path accepts
schema-first JSON input, runs the invented preview-model payload through the
validated in-memory kernel bridge, and emits structured JSON on stdout. The
`validate-input` path validates runner metadata. The `export-results`,
`run-benchmark`, and `run-regression` verbs are stable command vocabulary and
currently return structured blocking diagnostics until downstream payload
bindings are supplied by later bounded tranches.

The runner must not bypass unit validation, provenance validation,
result-envelope diagnostics, rule-pack sandbox boundaries, private-data
controls, report controls, or human professional review boundaries. CI
workflow files, release packaging/signing/publication, public API transport,
external adapter formats, and lifecycle issuance remain outside this
implementation.

## References

- `_CONTEXT.md` for sealed deliverable identity, SOW/OBJ coverage, acceptance/risk notes, and architecture-basis injection.
- `docs/CONTRACT.md` for invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-PRIV-1/2, OPS-K-AUTH-1, and OPS-K-AGENT-1..4.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 8, 9, 11, and 12 for architecture basis, scope ledger, open issues, and accepted decisions.
- `docs/_Registers/Deliverables.csv` row DEL-10-05.
- `docs/_Registers/ScopeLedger.csv` rows SOW-054 and SOW-032.
- `docs/SPEC.md` sections 1, 7, 8, 9, and 10 for layer boundaries, diagnostics/reporting, V&V, and deliverable mechanics.
- `docs/TYPES.md` sections 3, 4, 6, 8, and 9 for deliverable type, analysis-status vocabulary, data boundaries, object registry, and lifecycle semantics.
