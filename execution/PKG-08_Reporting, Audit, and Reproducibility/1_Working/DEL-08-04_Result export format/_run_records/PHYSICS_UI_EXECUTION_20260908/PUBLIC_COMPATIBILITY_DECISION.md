# CANDIDATE — audit D05 public compatibility choice

Decision identifier: `K8-D05-PUBLIC-COMPATIBILITY-20260908-V1`. This is the exact remaining Owner gate. It does not adopt a schema or authorize implementation.

## Recommended bundle: K-A

Adopt `0.2.0` as the pre-1.0 breaking successor; keep `0.1.0` immutable for historical validation; make new writers emit one `0.2.0` document after explicit version negotiation; allow bounded dual-read of `0.1.0` and `0.2.0` during consumer migration; never dual-write a purported complete `0.1.0`; reject an unknown or unimplemented version with `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` before any interpretation, while leaving the native mechanics payload accessible.

Why: the current contract is already pre-1.0, whole actual documents fail, and no urgent current library consumer is known. A single truthful full-coverage writer avoids prolonging a partial/invalid `0.1.0` output. Dual-read gives internal consumers a controlled migration path. Unknown-version refusal preserves the schema-first/no-bypass contract.

## Material alternatives

| Choice | Public behavior | Cost or consequence |
|---|---|---|
| **K-A — `0.2.0`, single current writer, dual-read, reject unknown** (recommended) | Exact requested version; new writer emits complete `0.2.0`; legacy reader support is time-bounded; unsupported request returns structured failure and native payload remains available. | Breaking pre-1.0 migration; every canonical consumer must declare version capability. |
| **K-B — `1.0.0` stabilization now** | Same complete schema shape, immediately designated stable; `0.1.0` is historical; single `1.0.0` writer; reject unknown. | Stronger compatibility promise before pressure and other source-specific semantics are normalized; future incompatible changes require a major version. |
| **K-C — dual artifacts during transition** | Produce a full `0.2.0` document and a separately labeled `0.1.0-partial-or-unsupported` compatibility artifact where requested; readers still validate exact identity and reject unknown versions. | More code and two artifacts to audit. The old artifact cannot be called complete or schema-conformant and may confuse downstream selection. |

## Separate forward-unknown handling sub-choice

- **K-U1 — reject** (recommended): do not retain or process the unknown document as a result envelope; return the structured unsupported diagnostic and supported versions.
- **K-U2 — opaque quarantine**: retain original bytes and their transport hash for forward relay, but prohibit parsing, rendering, comparison, report generation, conversion and governed-result validation.

Permissive parsing, version guessing, newest-known fallback, and silent field ignoring are excluded because they bypass the strict schema and can reinterpret source-specific rows.

## Exact Owner response requested

Select one bundle (`K-A`, `K-B`, or `K-C`) and one unknown-version behavior (`K-U1` or `K-U2`), or amend. The non-binding recommendation is `K-A + K-U1`.

On ruling, PKG08 records the adopted schema/version registry; PKG10 implements negotiation/adapter behavior; PKG04/05 and PKG13 provide accepted semantic/source-bridge inputs; PKG14 implements manifest/digest integration. Until then, all candidate files remain evidence only and no production schema or adapter changes are released.
