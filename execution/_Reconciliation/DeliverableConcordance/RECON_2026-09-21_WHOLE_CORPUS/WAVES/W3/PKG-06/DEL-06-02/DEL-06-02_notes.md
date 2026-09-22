# DEL-06-02 Sandboxed unit-aware expression evaluator: worker notes (W3, PKG-06, worker G1)

Forward ledger: 139 rows (88 required keys). `.rNN` splits of CLM-005, CLM-006,
CLM-008, CLM-015, CLM-022 and CLM-024. Sub-claims: SOW.s01 and .s02,
CLM-007.s01, CLM-017.s01, CONTEXT .s01 and .s02, MEMORY.s01 and .s02. Sealed
in `DEL-06-02_SEAL.txt`. Everything was read from the evidence checkout at
`00115c719`. Agent judgments, not owner rulings.

## Path aliases

- The evaluator is `core/rules/expression_evaluator` (crate
  `open_pipe_stress_expression_evaluator`). Its corpus is
  `fixtures/rule_expressions/conformance_corpus/` (69 cases), and the harness
  is `tests/conformance_corpus.rs`.
- `INIT.md` was removed on 2026-07-04 (9c4caf8fd). The current bootstrap route
  is `loop/LOOP_INIT.md`.
- The SOW cites "`docs/SPEC.md` section 6" for the evaluator, but SPEC §6 is
  loads and stress recovery and §7 is the rule-pack evaluator. This was
  already true at the initial migration. It is recorded once, as SOW.s02
  (CP-02).

## Judgment calls

- **SOW surface → STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE.** The whole
  surface specifies a "future" evaluator that now exists. It is not CP-04,
  because the SOW text does not carry the former name. The crate name does,
  but the SOW never names the crate.
- **OUT-001 → PARTIALLY_IMPLEMENTED · INVARIANT · SECURITY.** No explicit
  "bounded evaluation" limit was located: the evaluator and the document
  decoder both recurse, with no depth or size limit. Evaluator findings are
  also not bound into a governed result envelope. The resource-bound gap
  touches the sandbox subject (OPS-K-RULE-2).
- **Comparison tolerance (CLM-006.r03, CLM-015.r04) → CP-10,
  IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · OWNER.** The code compares
  values exactly, equality included, and the DEC-022 blessed corpus pins
  those outcomes. No ruling on comparison tolerance was found. DEC-024 and
  DEC-026 cover verification tolerances only. The owner may decide that
  DEC-022 settles it.
- **Plugin and adapter no-bypass (CLM-006.r05, REQ-06-02-010, CLM-016
  REQ-010) → CP-11 PARTIALLY_IMPLEMENTED.** The condition holds only because no
  adapter path reaches the evaluator. The declaration gate is deny-only
  (BLOCKED_RUNTIME_NOT_SELECTED).
- **AB-00-06 diagnostics (CLM-006.r04, CLM-016 REQ-009, CLM-017.s01) →
  PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE.** Findings carry only
  code, subject and message. The final taxonomy is still open (Remaining R02).
- **Remaining R01 to R03** are accurate and still open. Each is ALIGNED with
  `OPEN_ACTION` on the governing row (F2): REQ-06-02-011, CLM-016 REQ-009 and
  REQ-06-02-010.
- **Grammar, parser, binding and quantity TBDs (FG-01)** were ruled by DEC-022,
  DEC-031, DEC-037, DEC-038 and DEC-039. REQ-06-02-006 asked for a decision
  before implementation. The first slice (7490f67, 2026-05-02) came before
  DEC-022 (2026-06-11), and the ruling then adopted the in-repo AST. The row
  records this.
- **CLM-024 setup-state checks (.r05, .r06)** say the state stays
  SEMANTIC_READY; the actual state is IN_PROGRESS. Disposed
  STALE_SETUP_SPECIFICATION · RECORD_DRIFT · RECORD;LIFECYCLE.
  `tools/validation/validate_enum.py` does not exist.
- **Product caller:** the evaluator is called by the rule-check runner
  (desktop `run_rule_checks`) and the document validator (desktop
  `validate_rule_pack`). No F7 marker was needed.

## Canonical departures

None.

## Convention friction

- CP-10 and C6(d) do not say whether a blessed golden corpus under a grammar
  ruling counts as ruling the behaviour it pins. The tolerance rows are
  routed to the owner.

## Smallest check for each UNKNOWN row

All three (FG-DEL-06-02-02: CLM-006.r01, REQ-06-02-007, CLM-016 REQ-007) share
one check: find a protected-content and private-data review record covering
`fixtures/rule_expressions/` and the invented demo pack (DEC-058 scan record
or reviewer disposition).

## Reverse pass

- CLAIMED_BY: RC-06-0178 (evaluator) and RC-06-0038 (grammar version).
- PARTIAL: RC-06-0041 (corpus; its checksum-binding payloads are DEL-06-04's),
  RC-06-0256 (codec, which sits in the DEL-06-04 document crate) and
  RC-06-0006 (the runner's evaluation stage).
- COVERS: RC-06-0182, RC-06-0120, RC-06-0197 and RC-06-0282.
- Everything else is NOT_MINE. F5-specific reasons are given where entry
  points overlap my citations (adapter framework, units, model schema,
  governance docs, the shared desktop `lib.rs`).
- The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over the three sealed ledgers: PASS, 0 findings.

## Other

- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` (C9).
- Rename residue for R3: the crate name, the Cargo description ("…for
  OpenPipeStress") and the `expression_language` token
  `open_pipe_stress_declared_expression` are active identifiers carrying the
  former name. The DEL-06-02 SOW does not name them.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
