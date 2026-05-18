# Specification: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

## Scope

This deliverable specifies the setup basis for a GUI solve-execution workflow that keeps background solve execution, progress display, cancellation, diagnostic logs, solver warnings, and result-envelope status reviewable.

This setup does not implement GUI source code, UI tests, schemas, application-service code, background job code, solver code, report/export code, package manifests, or protected engineering data. Future implementation must consume the command/job/progress/cancellation and diagnostics/result-envelope contracts supplied by PKG-00 rather than inventing product semantics in the GUI slice.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| REQ-07-07-001 | The solve UX shall initiate solve execution through an application-service command or job contract and shall not directly mutate domain-core or solver state from GUI components. | `_CONTEXT.md` Architecture Basis Injection; DEL-00-03 REQ-03-01 and REQ-03-03 |
| REQ-07-07-002 | The solve UX shall treat solve execution as a background job where cancellation, progress, and reproducibility metadata are provided by the service/job contract. Exact progress phases, percentages, and event names remain `TBD` until the job contract fixes them. | DEL-00-03 REQ-03-04; SOW-055 |
| REQ-07-07-003 | Progress presentation shall display only service-reported states, phases, counts, or messages; it shall not synthesize misleading completion percentages when the job contract does not supply them. | OPS-K-AGENT-1/2; DEL-00-03 REQ-03-04 |
| REQ-07-07-004 | Cancellation controls shall issue a cancellation request through the application-service boundary and shall preserve the final job/result-envelope state returned by that boundary. The GUI shall not bypass cancellation boundaries or claim successful cancellation without envelope evidence. | DEL-00-03 REQ-03-04; OPS-K-AGENT-1 |
| REQ-07-07-005 | Diagnostic presentation shall preserve machine-readable diagnostic fields for code, class, severity, source, affected object, message, remediation, and provenance where supplied. | DEL-00-06 REQ-06-01 |
| REQ-07-07-006 | The solve UX shall preserve warning classes for `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` when those classes are present in diagnostics/result envelopes. | `docs/SPEC.md` section 7; DEL-00-06 REQ-06-02 |
| REQ-07-07-007 | The solve UX shall distinguish invalid input, incomplete model, mechanics solved, rule-check result, and human-review-needed states; it shall not expose automatic code-compliance, certification, approval, sealing, or professional-authentication states. | DEL-00-06 REQ-06-04/05; `docs/TYPES.md` section 4; OPS-K-AUTH-1 |
| REQ-07-07-008 | Solve-required missing data and rule-check-required missing data shall remain separate visible findings. Missing values shall not be supplied by the GUI as silent defaults. | OPS-K-DATA-1/2/3; `docs/DIRECTIVE.md` section 3 |
| REQ-07-07-009 | Any solve-run review summary intended for reports or exports shall retain traceability to available reproducibility metadata such as model hash, input manifest, solver version, rule-pack version/checksum, warnings, assumptions, and limitations. | OBJ-007; `docs/SPEC.md` section 8; DEL-00-03 REQ-03-04 |
| REQ-07-07-010 | Private project data, private rule-pack data, proprietary values, and protected standards/code content shall not be transmitted, displayed as public defaults, or copied into public setup artifacts. | OPS-K-IP-1/2/3; OPS-K-PRIV-1/2; OPS-K-DATA-1 |
| REQ-07-07-011 | Future implementation evidence shall include UI tests or equivalent review evidence for background job state rendering, progress display, cancellation request/terminal-state handling, diagnostic filtering/detail review, and professional-boundary notices. | DEL-07-07 anticipated artifacts; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` |

## Standards

No external engineering code or protected standards text is introduced by this deliverable. The controlling references for this setup are the OpenPipeStress governance, decomposition, register, and PKG-00 architecture-basis artifacts listed in `_REFERENCES.md` and `Datasheet.md`.

Any future code-specific acceptance criterion, stress limit, load-combination default, allowable, SIF/flexibility value, or proprietary component value must be supplied by a user-owned/private source with provenance and redistribution status. This GUI slice may display resulting diagnostics/statuses; it must not define or certify code compliance.

## Verification

| Verification ID | Method | Expected evidence |
|---|---|---|
| VER-07-07-001 | Document review | Four-document kit exists and matches DEL-07-07 scope. |
| VER-07-07-002 | Boundary review | No GUI source, tests, schemas, job/solver code, package manifests, repo-level docs, or `ISSUED` artifacts were edited. |
| VER-07-07-003 | Contract review | Requirements explicitly route progress, cancellation, diagnostics, and result status through PKG-00 command/job and result-envelope contracts. |
| VER-07-07-004 | Protected-content review | Setup artifacts contain no protected standards text, proprietary engineering values, certification claims, or automatic code-compliance claims. |
| VER-07-07-005 | Semantic setup review | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and provide complete lens coverage. |
| VER-07-07-006 | Dependency-register validation | `Dependencies.csv` validates against the v3.1 schema and contains evidence-linked ACTIVE rows. |
| VER-07-07-007 | Future implementation test | UI test coverage for job launch state, progress, cancellation, diagnostics, blocked states, and report/export traceability remains `TBD` until implementation work is authorized. |

## Documentation

Required setup records for this deliverable are:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

## Acceptance Notes

`SEMANTIC_READY` means the setup artifacts are prepared for review. It does not mean product implementation is complete, UI behavior is tested, solver behavior is verified, protected data is authorized, code compliance is established, or professional approval has occurred.
