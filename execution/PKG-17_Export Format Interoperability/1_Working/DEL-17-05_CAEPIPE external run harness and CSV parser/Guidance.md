# Guidance: DEL-17-05 CAEPIPE external run harness and CSV parser

## Purpose

Use this deliverable to keep CAEPIPE external-run work useful without crossing the project boundary. The harness can help confirm that a user-owned CAEPIPE environment accepts a generated MBF and emits CSV output, and the parser can extract limited regression/handoff evidence from that CSV. Neither result is an engineering approval state.

This Phase A kit is intentionally conservative. It records what later implementation must preserve, what remains unresolved, and what reviewers should reject.

## Principles

- External execution is opt-in and user-owned.
- A configured executable path is evidence of user configuration, not a bundled project dependency.
- Public CI and public fixtures must not require CAEPIPE.
- Parsed CSV data is regression/handoff evidence only.
- Target behavior that is not source-confirmed remains `TBD`.
- The harness shall preserve DEL-17-02 manifest, stable-ID, loss-report, and boundary vocabulary.
- The parser shall prefer narrow, documented coverage over broad guesswork.
- Unsupported, missing, unstable, or unmapped output shall become diagnostics, not silent success.

## Vocabulary

Use `target CSV artifact` as the preferred term for the CAEPIPE CSV file produced or expected from a user-owned external run. `Expected CSV path`, `observed CSV path`, `CSV result file`, and `CSV output` are allowed aliases only when they describe a specific state of that same artifact. Do not split these aliases into separate concepts unless a later implementation schema deliberately does so.

## Considerations

The public CAEPIPE import documentation supports MBF as a text model input path and describes command-line CSV creation from an MBF input. The batch-mode documentation also describes CSV output from MBF input, but its command shape includes a batch-mode argument pattern. DEL-17-05 should therefore carry the exact first invocation profile as `TBD` until the project chooses a documented profile or receives support clarification.

The export-data documentation supports CSV/text output surfaces for model and result data. It does not, by itself, define which CSV sections are stable enough for automated parser coverage in the first OpenPipeStress harness. Parser coverage should be explicit, small, and test-backed.

The harness should not attempt to infer CAEPIPE solver validity. It can record operational facts such as input file written, executable invoked, CSV discovered, parser completed, counts matched where meaningful, and rows correlated to canonical IDs where possible.

Invocation-profile and parser-section TBDs should move to implementation scope only when the project has enough evidence to avoid hidden target-behavior claims. The ruling path is: first use public/official CAEPIPE documentation where it is explicit; otherwise use CAEPIPE support clarification or a human project-authority decision recorded in the deliverable's accepted implementation brief. Fixture-confirmed parser behavior may support parser coverage only when the fixture is invented, rights-cleared, or private/user-controlled with documented handling.

## Trade-offs

| Decision area | Conservative choice | Trade-off |
|---|---|---|
| Executable discovery | Require explicit user configuration. | Less convenient, but avoids bundled dependency and license-bypass implications. |
| Invocation profile | Keep exact command profile `TBD` until reconciled. | Slower implementation, but prevents a false support claim. |
| Parser scope | Parse only known/fixture-confirmed sections. | Narrow evidence at first, but clearer diagnostics. |
| CSV identity | Use manifest/ID-map correlation where possible. | Requires sidecars and run metadata, but avoids row-order assumptions. |
| CI behavior | Skip external-run tests without configured executable. | Public CI will not exercise live CAEPIPE, but remains legal and reproducible. |
| User CSV handling | Default to private unless redistribution rights are recorded. | Fewer shared fixtures, but preserves data/IP boundaries. |
| TBD closure | Require explicit ruling evidence before implementing invocation or parser-section assumptions. | Slower closure, but keeps target behavior and authority boundaries reviewable. |

## Examples

Acceptable Phase A examples:

- A document-level run record listing the MBF input path, expected CSV path, parser status, and boundary note as fields to capture later.
- An invented parser fixture that exercises CSV header handling without copying a vendor or client file.
- A skipped external-run test record that states no user-owned executable path was configured.

Not acceptable:

- A committed CAEPIPE executable, installer, commercial example model, or copied vendor fixture.
- A claim that OpenPipeStress is CAEPIPE-compatible because a CSV file was produced.
- A claim that a successful CAEPIPE run proves code compliance, professional acceptance, or formal validation.
- A parser that silently treats unknown sections or unmapped rows as supported.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-17-05-CONF-001 | Public CAEPIPE pages describe related MBF-to-CSV execution patterns, but the exact first OpenPipeStress invocation profile is not selected. | `CAEPIPE-IMPORT-MBF`, command-line operation, lines 15-20. | `CAEPIPE-BATCH`, batch-mode notes, lines 40-52. | `Specification.md` REQ-003/004; `Procedure.md` run-profile steps. | Carry invocation profile as `TBD` until support clarification or human profile decision. | TBD |
| DEL-17-05-CONF-002 | CSV export surfaces are documented, but stable parser-section coverage for automated regression is not selected. | `CAEPIPE-EXPORT-DATA`, export list, lines 31-43. | DEL-17-01 `TBD-17-01-004` and CQ-17-01-005. | `Specification.md` REQ-009; parser coverage records. | Keep first parser coverage `TBD`; require fixture-confirmed parser coverage before implementation claims. | TBD |

## Review Guidance

Reviewers should reject later DEL-17-05 work if it:

- executes CAEPIPE without explicit user configuration;
- introduces bundled binaries, installers, proprietary examples, or license-bypass behavior;
- treats public documentation as permission to claim broad compatibility;
- treats CSV parsing as professional acceptance or code compliance;
- commits private/user CAEPIPE outputs without provenance and redistribution review;
- hides target version, invocation, parser-section, unit, coordinate, or ID-map uncertainty;
- expands into schema/code implementation outside an approved implementation tranche.
## D-41 R5 T3 PDU-016 Boundary

Invented public fixtures remain rights-safe test inputs, but user-provided CSV/runtime evidence must not inherit their public classification by default.
