# Result family reservation — version 2

ROOT HELP_HUMAN selects this prospective implementation contract under the owner's [correctness authority](../CORRECTNESS_ACTIVATION.md). It extends and supersedes the prospective selection in [VERSION_RESERVATION.md](VERSION_RESERVATION.md), whose committed bytes remain historical. The [compatibility review](VERSION_COMPATIBILITY_REVIEW.md) supplies the inspected contracts and rationale. This selection was sent to both implementation managers before dependent consumer work; recording it does not assert that the implementation or its checks are complete.

| Family | Selected version / identity | Meaning |
|---|---|---|
| Raw mechanics | `0.2.0` | New required producer, formulation and numerical-quality metadata; finite unrounded values |
| Product producer | `0.2.0` | Explicit new producer contract, not inferred from the raw version alone |
| Canonical result derivative | `0.3.0` | Both outer and nested result-envelope versions; exact recognized source semantics |
| Analysis record | `0.3.0` | Outer version and `strict_analysis_run_v0_3` record contract; its nested analysis record has no separate schema-version field |
| Stress-neutral export | `0.3.0`, `ops.stress_neutral.v3` | Outer version, export-profile version and profile reference, including manifest linkage |
| Model document | Retain `0.2.0` | Pressure model `0.3.0` remains a separate qualified input change |

The first new interpretation is exactly `openpipestress.result_semantics/0.3.0/precision-1`, bound to its reviewed mapping hash. Do not activate a second prospective meaning of analysis `0.2.0` through an `analysis_run.v0.2.precision-1` schema. The historical analysis 0.2 schema and stress-neutral v2 profile pin old semantic contracts; keeping their numbers while changing those contracts would obscure the compatibility boundary.

Analysis 0.3 is an explicitly versioned carrier with a typed registry of exact supported semantic ID/hash pairs. Initially only the qualified precision-1 pair is supported. Unknown or contradictory raw/producer/semantic tuples fail the new construction and interpretation path; they do not fall back to an old builder. A future reactions or pressure semantic revision requires deliberate qualification of its producer and consumers, not an automatic minor bump of every artifact or implicit trust through the carrier's version.

Preserve historical schemas, fixtures, semantic tables, serialization routines, values and hash verification. Authentic old raw 0.1 and the existing synthetic headerless raw 0.2 fixture remain readable under their original contracts. The latter is historical, unqualified evidence: a fresh current builder must reject its missing new headers. Do not relabel that fixture, manufacture metadata, weaken the new builder, or reconstruct lost precision to obtain compatibility. Explicit historical adapters must stay outside the Current composition path.

Input freshness, checksum identity, supported interpretation, source/build binding, mechanics status, numerical eligibility and physical formulation scope are distinct checks. A matching model hash or a new schema number alone never makes a result integrity-qualified Current. Old or new unassessed results remain inspectable, with recomputation required where appropriate; original raw statuses and hashes remain unchanged. Complete actual case evidence is required for numerical eligibility. Newly generated browser fixtures must come from the admitted producer and actual gate, with honest model and build bindings.

SOLVER_MANAGER owns the core producer/consumer integration. AUTHORING_MANAGER owns the agreed desktop/native consumer files and their connection to that contract. ROOT owns this shared reservation and cross-branch integration. Required checks include unchanged historical verification, signed subquantum round trips through store/reopen/export, unknown/mismatched-header rejection, source/row semantic and hash binding, unsupported and not-assessed standing, and actual native/headless composition. Fresh complete-diff review, clean candidate checks and native witnesses remain open. This is neither a release nor a claim that numerical eligibility proves all physical behavior.
