# Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Design Guidance

Treat CAEPIPE MBF as a narrow, source-confirmed target profile. The purpose is not to claim compatibility; it is to define what a future deterministic writer may safely emit, what it must omit, and what it must report.

The safe posture is conservative: if a record family, required field, stable ID carrier, option, or target behavior is not confirmed by admitted source evidence, mark it `TBD` and require a loss-report or profile diagnostic.

## Interpretation Guidance

- Use DEL-17-01 for CAEPIPE source facts and unanswered questions.
- Use DEL-17-02 for manifest, profile, ID-map, and loss-report structure.
- Do not treat MBF text output as equivalent to professional acceptance.
- Do not turn pass-through target options into local code-checking logic.
- Do not infer undocumented MBF behavior from the target name or common practice.

## Classification Guidance

Unsupported target behavior must be classified before implementation as blocking, diagnostic-only, delegated, omitted, approximated, or `TBD`. Until that classification exists, the writer contract should preserve the uncertainty rather than enabling a broad MBF support claim.

Pass-through CAEPIPE options are metadata for the target profile because they may name target-side analysis settings. They must not become OpenPipeStress local code-checking logic, local professional acceptance logic, or a substitute for user-supplied rule packs and human review.

## Open Questions

| TBD | Question | Later closure path |
|---|---|---|
| TBD-17-04-001 | Which CAEPIPE version/profile is the first target? | Public documentation review or CAEPIPE developer/support clarification. |
| TBD-17-04-002 | Which MBF record families and required fields are in the first subset? | Source review and bounded implementation design. |
| TBD-17-04-003 | Can MBF carry stable canonical IDs directly? | Source review; otherwise sidecar-only mapping. |
| TBD-17-04-004 | Which unsupported entities block export versus produce non-blocking diagnostics? | Later profile and implementation tranche. |
