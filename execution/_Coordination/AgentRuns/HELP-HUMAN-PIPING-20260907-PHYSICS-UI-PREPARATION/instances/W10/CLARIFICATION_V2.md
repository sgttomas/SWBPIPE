# W10 additive clarification V2 — option order and package ownership

Parent requested this bounded clarification after reviewing the frozen V1 packet. It supplements CANDIDATE_REPAIR_BRIEF.md and RETURN.md; V1 files and manifest remain immutable. No product, schema, authoritative state or adoption changes. Base and candidate gates unchanged. Standard claim fence applies (F-PIP-2; DEC-081).

## Option order

Option B is optional containment, **not a prerequisite for option A**. Given the Owner priority to complete physics and design functionality, directly preparing/adopting the exact option A versioned complete-coverage design is a valid completion route. B is useful only if an identified canonical consumer needs an immediate truthful unavailable outcome while A is designed; no such urgency was established in W10. V1's recommended first containment sequence is non-binding, and must not be converted into a required work dependency or implementation gate. The Owner may choose A directly; A still needs exact schema version, semantic coverage, consumer migration and package assignment before implementation.

Fresh follow-up source inspection found the observed failing field is library-only: `core/runner/headless/src/lib.rs:671–682` marks result_envelope_document with serde(skip_serializing). `core/runner/headless/src/bin/openpipestress-runner.rs:788–815` selects runner_result and mechanics_envelope for CLI output; export-results separately requires a report-package payload (line 342). Search for result_envelope_document across apps/, tools/ and tests/ produced no matches; core references were the headless producer and its own tests. These observations do not prove no external Rust library consumer exists. They establish **no known urgent current consumer blocking case**; W10 does not infer a UI solve/product block from the schema findings. Broader public exposure remains audit D05-gated, and the current complete-document conformance failure remains real for any library consumer that reads this field.

## Ownership correction

Replace the overbroad V1 phrase “PKG13 owns native producer semantic clarification” and any equivalent shorthand with:

- PKG04/PKG05 own the actual mechanics, loads and result-producing semantics implemented in product_physics and the corresponding solver/load/recovery modules, by their accepted deliverable scopes.
- PKG13 owns the physical-to-analytical mapping bridge and trace relationships, including DEL-13-04; its held bridge prerequisites remain separate.
- PKG08/DEL-08-04 owns canonical result-schema/export vocabulary; PKG08 also owns audit/hash contracts through the applicable deliverables.
- PKG10/DEL-10-05 owns the headless adapter integration, with HELP_HUMAN coordinating cross-package boundaries.

A future scoped activation must resolve the exact mechanics/result producer deliverable per changed semantics. This correction assigns no new authority and authorizes no writes in those packages. It prevents a preparation shorthand from silently transferring native product_physics ownership to PKG13.

## Follow-up verification

Read-only searches: `rg -n 'result_envelope_document|build_result_export_document' projects/chirality-piping/core --glob '*.rs'`; `rg -n 'result_envelope_document' projects/chirality-piping/apps projects/chirality-piping/tools projects/chirality-piping/tests`; source inspection at the cited lines. No new numerical build or test warranted by prose clarification. Prior source hashes remain binding; the CLI source hash is added to the successor manifest metadata. Root synthesis and R1 review should consume V1 plus this clarification. Preparation remains pending independent review/root acceptance.
