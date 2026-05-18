# Guidance: DEL-17-03 Native open JSON export package

## Design Guidance

Treat the native JSON package as the least target-specific export path. Its main value is traceability: it should make source identity, package contents, canonical IDs, diagnostics, and losses visible before any target adapter consumes the package.

The package should be deterministic where future implementation can make it deterministic. Runtime timestamps, if retained, should be declared rather than hidden inside hashes or manifests.

## Interpretation Guidance

- Use DEL-17-02 as the contract foundation.
- Use DEL-17-01 for target-boundary discipline and source authority.
- Prefer explicit omissions over silent defaults.
- Treat validation reports as package evidence, not solver validation.
- Keep adapter-specific behavior in later deliverables.

## Open Questions

| TBD | Question | Later closure path |
|---|---|---|
| TBD-17-03-001 | Which concrete JSON schemas will bind package members? | Future implementation/schema tranche. |
| TBD-17-03-002 | Which hash canonicalization helper will package writer code use? | Future code tranche using project hashing policy. |
| TBD-17-03-003 | Which invented fixtures will exercise native JSON round trips? | Future test/fixture tranche. |
| TBD-17-03-004 | Which concrete schema and writer binding source will authorize native JSON implementation? | Future implementation/schema tranche; reread Guidance.md Open Questions for F-001. |
