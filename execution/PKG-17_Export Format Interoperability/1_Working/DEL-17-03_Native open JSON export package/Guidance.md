# Guidance: DEL-17-03 Native open JSON export package

## Design Guidance

Treat the native JSON package as the least target-specific export path. Its main value is traceability: it should make source identity, package contents, canonical IDs, diagnostics, and losses visible before any target adapter consumes the package.

The package foundation should be deterministic. Runtime timestamps, if retained for reportability, should be declared rather than hidden inside hashes or manifests.

## Interpretation Guidance

- Use DEL-17-02 as the contract foundation.
- Use DEL-17-01 for target-boundary discipline and source authority.
- Prefer explicit omissions over silent defaults.
- Treat validation reports as package evidence, not solver validation.
- Keep adapter-specific behavior in later deliverables.
- Treat the native JSON schema, builder, invented fixture, and focused tests as DEL-17-03-owned foundation outputs.
- Keep API/CLI/GUI integration, project-store export flow, downstream target adapters, and target-specific behavior in later deliverables or separately approved tranches.

## Closed Foundation Questions And Residual Boundaries

| Item | Question | Current disposition |
|---|---|---|
| TBD-17-03-001 | Which concrete JSON schemas will bind package members? | Closed for the bounded foundation by `schemas/native_json_export.schema.json`; future schema changes require governed follow-up. |
| TBD-17-03-002 | Which hash canonicalization helper will package writer code use? | Closed for the bounded foundation by `core/handoff/native_json/package.py` canonical JSON and member-hash behavior; broader project hash unification remains an integration concern. |
| TBD-17-03-003 | Which invented fixtures will exercise native JSON round trips? | Closed for the bounded foundation by `fixtures/native_json/invented/native_json_export_package.json` and focused tests. |
| TBD-17-03-004 | Which concrete schema and writer binding source will authorize native JSON implementation? | Closed for the bounded foundation by TP-EXPORT-006 implementation evidence; API/CLI/GUI and project-store binding remain future scoped work. |
