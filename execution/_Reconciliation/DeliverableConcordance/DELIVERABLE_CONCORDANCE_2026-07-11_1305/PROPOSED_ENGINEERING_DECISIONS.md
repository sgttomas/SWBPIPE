# Proposed Engineering Decision Findings — R3 Evidence Only

> **Epistemic status:** agent-authored, non-operative R3 synthesis from the
> corrected claim ledgers and all 18 package fan-in records at frozen source
> SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. These are candidate findings
> and authority routes required by the run artifact contract. They are not
> engineering rulings, decision options, repair authorizations, validation
> approvals, or R4 packets.

## ED-001 — Canonicalization label fidelity

Finding: multiple serializers produce deterministic sorted compact JSON while
emitting or discussing `JCS_compatible` labels without RFC-8785 conformance
vectors. This is one fidelity species, not one gap per downstream exporter.

- Evidence: CU-007; `DEL-14-02-REQ-007`, `DEL-14-02-ACC-004`,
  `DEL-15-03-REQ-008`, `DEL-17-03-REQ-006`, `DEL-17-06-ACC-008`, and
  `DEL-17-09-REQ-001`; `PACKAGE_SUMMARIES/PKG-14.md` risk 2,
  `PKG-15.md` risk 3, and `PKG-17.md` risk 3.
- Reliability: UNVERIFIED agent/fan-in evidence; deterministic tests do not
  supply the missing external conformance basis.
- Authority route: ENGINEERING, separately applied to each owning serializer.
- Smallest next action: execute accepted RFC-8785 vectors against each owner or
  narrow the label. No serializer or label is changed in R3.

## ED-002 — Unit authority and numeric normalization validation

Finding: the canonical units crate exists, while consumer literals and
caller-supplied conversion paths preserve a duplicate-authority/drift risk;
unit-presence tests do not establish numeric conversion suitability.

- Evidence: CU-008; `DEL-02-02-REQ-002`, `DEL-02-02-REQ-006`,
  `DEL-09-01-REQ-004`, `DEL-14-03-REQ-007`, and `DEL-14-04-REQ-003`.
- Reliability: UNVERIFIED; the frozen vocabulary match and focused tests are
  verified software facts, while engineering suitability is not validated.
- Authority route: ENGINEERING for numeric/normalization basis; OWNER only for
  any later ownership/delegation record.
- Smallest next action: state the minimum independent normalization witness and
  identify which consumer literals must call or derive from the canonical owner.

## ED-003 — Solver, benchmark, and release threshold suitability

Finding: threshold residuals recur across solver convergence, performance,
stress benchmark, tolerance, and comparison packages. Named decisions permit
some bounded deferrals, but do not select broader numeric floors.

- Evidence: CU-009; `DEL-04-04-REM-002`, `DEL-04-04-REM-003`,
  `DEL-04-04-REM-004`, `DEL-04-05-REM-001`, `DEL-05-03-ACC-001`,
  `DEL-09-02-REQ-007`, and `DEL-14-04-ACC-003`.
- Reliability: UNVERIFIED; existing fixture results and permitting records are
  not engineering approval of broader thresholds.
- Authority route: ENGINEERING after each recorded stage gate; do not count
  ALIGNED remaining rows as decisions already made.
- Smallest next action: consolidate candidate thresholds by owning policy and
  evaluate only when the existing gate permits.

## ED-004 — Production-path oracle binding

Finding: section-property witnesses and benchmark calculations can re-derive
formula values without executing the production calculator path, leaving a
verification-versus-validation gap.

- Evidence: CU-010; `DEL-03-08-REQ-003`, `DEL-09-01-ACC-004`, and
  `PACKAGE_SUMMARIES/PKG-03.md` Package reading.
- Reliability: UNVERIFIED; current tests pass but do not close the cited path
  binding.
- Authority route: ENGINEERING for witness suitability; implementation owner
  for any later test wiring.
- Smallest next action: run the production path against the existing
  rights-safe oracle values and record the comparison without changing the
  oracle basis.

## ED-005 — Runtime control reach and no-bypass evidence

Finding: privacy, redaction, telemetry, protected-content, external-reference,
and schema guards are implemented at helper/contract grain, but several named
runtime, adapter, report, and export seams are unbound.

- Evidence: CU-011; `DEL-12-02-REQ-012`, `DEL-12-03-REQ-009`,
  `DEL-13-01-REQ-011`, `DEL-14-01-REQ-009`, `DEL-15-02-REQ-011`, and
  `DEL-17-05-REQ-014`.
- Reliability: UNVERIFIED; negative focused tests do not prove whole-runtime
  security or privacy sufficiency.
- Authority route: OWNER/PRODUCT first for seam and default-policy selection;
  ENGINEERING for the resulting no-bypass design and evidence.
- Smallest next action: name the authoritative seams/defaults, then define
  bounded tests for only those seams. No security assurance is inferred.

## ED-006 — Field-scalar traceability and result-envelope mapping

Finding: object/DTO links, hashes, and diagnostics exist, but field-scalar and
runtime-result trace chains plus formal application result-envelope mapping are
incomplete.

- Evidence: CU-012; `DEL-13-03-REQ-006`, `DEL-13-04-REQ-004`,
  `DEL-13-04-ACC-003`, `DEL-15-03-REQ-008`, and `DEL-17-06-ACC-006`.
- Reliability: UNVERIFIED; current focused tests establish the bounded object
  grain only.
- Authority route: ENGINEERING for the trace model; OWNER/PRODUCT for the
  application-service home if not already explicit.
- Smallest next action: define the minimal trace link from source scalar through
  transformation/result envelope before assessing downstream preservation.

## ED-007 — Review-geometry coordinate and validation breadth

Finding: JSON glTF centerline output is deterministic and schema-tested, but
source-to-target transform provenance, identifier species, impossible-bend/
mapping completeness diagnostics, GLB checks, and rendered visual QA are not
all present.

- Evidence: CU-016; `DEL-17-08-REQ-009`, `DEL-17-08-REQ-015`,
  `DEL-17-08-REQ-026`, and `DEL-17-08-ACC-011`.
- Reliability: UNVERIFIED; invented fixture equality is not viewer,
  coordinate, or engineering validation.
- Authority route: OWNER/PRODUCT selects format and geometry scope;
  ENGINEERING evaluates transforms/diagnostics only for the selected scope.
- Smallest next action: preserve JSON glTF as the current bounded profile and
  define extra evidence only after a broader scope is selected.

## ED-008 — Benchmark/result-envelope integration

Finding: benchmark crates verify direct mechanics behavior, but governed
result-envelope/export binding and project-catalog dimensional coverage remain
partial.

- Evidence: `DEL-09-01-REQ-004`, `DEL-09-01-ACC-004`,
  `DEL-09-02-REQ-008`; `PACKAGE_SUMMARIES/PKG-09.md` risks 2, 5, and 6.
- Reliability: UNVERIFIED; clean benchmark runs are not release readiness or
  end-to-end result-envelope validation.
- Authority route: ENGINEERING for dimensional/benchmark sufficiency;
  OWNER/PRODUCT for the result-envelope integration home.
- Smallest next action: bind one rights-safe benchmark output through the
  governed envelope without inventing release thresholds.

## R3/R4 fence

This file stops at evidence-backed candidate findings, routing, and smallest
next checks. It does not draft alternatives, recommend a ruling, authorize
implementation, change a dependency/register, or perform R4/R5.
