# Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Design Guidance

Treat CAEPIPE MBF as a narrow, source-confirmed target profile. The purpose is not to claim compatibility; it is to define and test what a bounded deterministic writer foundation may safely emit, what it must omit, and what it must report.

The safe posture is conservative: if a record family, required field, stable ID carrier, option, or target behavior is not confirmed by admitted source evidence, mark it `TBD` and require a loss-report or profile diagnostic. The first implementation foundation therefore uses sidecar stable-ID mapping and carries direct MBF ID carrying as `TBD-17-01-003`.

Profile-basis fields are guardrails, not target-support claims. Until later source-confirmed closure exists, blank values or informal target-version/record-subset labels should block the package because they would make unresolved CAEPIPE scope look settled.

Source-basis refs are also guardrails. A caller-provided CAEPIPE MBF profile should not be able to omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, and should not promote DEL-17-03 implementation-pattern evidence into CAEPIPE target/source authority.

## Interpretation Guidance

- Use DEL-17-01 for CAEPIPE source facts and unanswered questions.
- Use DEL-17-02 for manifest, profile, ID-map, and loss-report structure.
- Do not treat MBF text output as equivalent to professional acceptance.
- Do not turn pass-through target options into local code-checking logic.
- Do not infer undocumented MBF behavior from the target name or common practice.
- Treat generated MBF text as a deterministic invented smoke artifact, not as target acceptance evidence.

## Classification Guidance

Unsupported target behavior must be classified before implementation as blocking, diagnostic-only, delegated, omitted, approximated, or `TBD`. Until that classification exists, the writer contract should preserve the uncertainty rather than enabling a broad MBF support claim.

For this foundation, explicit unsupported behavior defaults to warning-level loss-report evidence unless the loss entry marks it blocking. Missing unsupported loss coverage, malformed unsupported references, or `info` severity for unsupported behavior are blocking because they would hide material target-scope limits.

Pass-through CAEPIPE options are metadata for the target profile because they may name target-side analysis settings. They must not become OpenPipeStress local code-checking logic, local professional acceptance logic, or a substitute for user-supplied rule packs and human review.

## Open Questions

| TBD | Question | Later closure path |
|---|---|---|
| TBD-17-04-001 | Which CAEPIPE version/profile is the first target? | Public documentation review or CAEPIPE developer/support clarification. |
| TBD-17-04-002 | Which MBF record families and required fields are in the first subset? | Source review and bounded implementation design. |
| TBD-17-04-003 | Can MBF carry stable canonical IDs directly? | Source review; otherwise sidecar-only mapping. |
| TBD-17-04-004 | Which unsupported entities block export versus produce non-blocking diagnostics? | This foundation uses warning by default with explicit blocking allowed; later source-confirmed profiles may specialize entity-specific severity. |

## Foundation Guidance

The first implementation foundation should remain sidecar-first and loss-report-first:

- deterministic text output is acceptable only for the invented smoke subset;
- every canonical identity must appear in a sidecar mapping or an explicit loss entry;
- every unsupported entity must be a stable reference and must have an explicit unsupported loss entry;
- target-version, record-subset, and direct stable-ID uncertainty must remain carried as `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`;
- missing smoke-subset inputs, sidecar IDs, or loss reports should block the package;
- missing, blank, or weakened profile-basis evidence should block the package;
- missing or unsafe CAEPIPE source-basis refs should block the package;
- external execution, CSV parsing, target result interpretation, and target code/check logic remain out of scope.
