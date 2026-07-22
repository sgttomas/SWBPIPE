# N1 Return — Core/CLI/adapter/plugin/downstream inventory

**Status:** `SUCCESS`; read-only; frozen SHA verified.

The existing Python redaction helper has no non-test runtime caller. Eight
Python handoff/export foundations and their writers, the handoff workflow,
active headless-runner outputs, result-export serialization, and report
construction were traced. Current privacy/validation diagnostics frequently
mark a package blocked, but the writers still emit the package. The adapter
runtime is deny-only and no plugin runtime exists. `export-results` remains a
stub; active solve/validate/benchmark/regression stdout and `--output` are
current CLI egress routes.

Accepted route IDs and evidence are preserved in `ROUTE_MATRIX.csv`:
`REXC-CORE-001` through `REXC-CORE-012`, `REXC-CLI-001`, and
`REXC-REL-001`. The independently governed DEC-059 public-source exporter is
recorded as overlap but excluded from the product tranche.

Required preservation: source non-mutation; units; provenance and safe
checksums; existing diagnostics; adapter/plugin deny-only posture; protected-
content/report gates; professional-state distinctions. Internally assembled
diagnostic representations may exist, but no blocked/raw payload may reach a
filesystem, stdout, UI, print surface, or downstream handoff.

Key design constraint: current package shapes are not uniformly compatible
with the helper's metadata-bearing value shape, so a blind whole-object walk
is invalid. Each route needs an explicit metadata projection and materializer.

