# DEL-06-01 Rule-pack schema: worker notes (W3, PKG-06, worker G1)

Forward ledger: 93 rows (71 required keys; `.rNN` splits of CLM-004 and
CLM-012; sub-claims SOW.s01, CLM-021.s01, CONTEXT architecture-basis .s01/.s02
and MEMORY.s01). Sealed in `DEL-06-01_SEAL.txt`. Everything was read from the
evidence checkout at `00115c719`. Agent judgments, not owner rulings.

## Path aliases

- `schemas/…`, `tests/…`, `examples/…`, `core/…`, `apps/…` are project-root
  tokens under `projects/chirality-piping/`.
- Deliverable-local files are cited in full, with spaces:
  `projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema/…`.
- SOW migration and parity records are repository-root tokens
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`. They
  exist only at the root, not in the project.
- The Rust canonicalizer is `core/serialization/canonical_json`. The Python
  canonicalizer is exported by `core/project_persistence` (`service.py`).
- The file is named `.yaml`, but the schema is written in JSON syntax.

## Judgment calls

- **Requirements written as "the future schema shall…"** (REQ-06-01-001 to
  -012) are judged on their substance, under C6(b). The stale "future"
  framing is judged once, on CLM-010, and also on CLM-003 and CLM-017
  (FG-03). The verification-approach column counts as part of each row. It is
  met when a located test exercises the named case.
- **Protected-content review (FG-02).** Six rows depend on a review of the
  public schema and the invented demo: CLM-004.r08, REQ-007, CLM-013, CLM-014,
  CLM-020 and CLM-021.s01. No such review was located. There is no DEC-058
  scan record (`validation/evidence/releases/` does not exist), and no
  reviewer disposition either. The demo's own `protected_content_review:
  completed_no_protected_content` is a self-declaration. These rows are
  UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA · REVIEW. An agent
  reading of the schema is not a review (R0 review F8).
- **REQ-06-01-011 → PARTIALLY_IMPLEMENTED, INVARIANT.** The schema has the
  public-example classification values, but nothing makes a public-example
  classification require invented values or a non-engineering notice.
- **CLM-006 (eleven record groups) → ALIGNED.** The block asks that the groups
  be "evaluated", with fields "to resolve later". Every group exists. Some
  listed fields were never adopted (namespace, timestamps, import path, source
  hash). The resolve-later wording allows that. A verifier may read the block
  more strictly.
- **Grammar TBD (FG-01).** Unconditional TBDs are
  STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING (DEC-022, DEC-037):
  CLM-004.r11, OI-006, CONTEXT .s02 and MEMORY.s01. AC-001 was written at the
  SOW migration, after DEC-022, so it is STALE_REVIEW_OR_EVIDENCE. The TBD
  statements in CLM-005, CLM-025 and CLM-027 are conditional ("unless later
  ruled") and stay ALIGNED.
- **Implementation-level TBDs settled in code** (CLM-012.r03 file layout,
  .r04 JCS library): STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE. These are
  not CP-10 holds, because the architecture basis leaves such choices to
  implementation.
- **OI-010** (encryption default) is accurate and still open, so ALIGNED.
  **OI-011** (container) is overtaken by DEC-017, DEC-028 and DEC-057 (FG-04).
- **SPEC section 7** is behind the code. It says the schema "does not select
  the expression grammar", and it calls the evaluator "deliberately small".
  This is recorded on CLM-014.
- **Product caller:** the desktop `validate_rule_pack`, `compute…checksum` and
  `run_rule_checks` commands reach the schema-governed documents. No F7 marker
  was needed.

## Canonical departures

None. The CS-01, CS-02, CS-04, CS-06 and CS-07 rows inherit their canonical
fields. CP-02, CP-03, CP-04, CP-05 and CP-09 are applied as written.

## Convention friction

- F4 flags ALIGNED rows whose notes describe tested "missing …" behaviour. The
  GAP_WORDING_CHECKED clauses on REQ-001 to -006, -008 and CLM-004.r10 explain
  this.
- CP-09 covers "OUT-001 and parity rows". I applied it to the matrix OUT-001
  and VER-001, and judged the purpose-section OUT-001 on substance.

## Smallest check for each UNKNOWN row

All six (FG-DEL-06-01-02) share one check: find a protected-content review
record covering `schemas/rule_pack.schema.yaml` and
`examples/rule_packs/invented_demo.yaml`, either a DEC-058 scan record or a
named reviewer disposition. If none exists, one review closes the group.

## Reverse pass

- Answers: CLAIMED_BY RC-06-0099 (the schema); PARTIAL RC-06-0197 (SPEC §7);
  COVERS RC-06-0200 (invented demo; DEL-06-05 owns it), RC-06-0065 (checksum
  computation fills the schema's checksum fields), RC-06-0182 (document
  validation) and RC-06-0256 (codec for the schema's ExpressionNode). All
  other capabilities are NOT_MINE. Where entry points share paths with my
  citations, the NOT_MINE reasons are specific to the capability (F5).
- The reverse pass did not change my view of any sealed row. The rule-pack
  document crate (validation, codec, checksum) was built under TP-C2-RPLIFE-001
  (DEL-06-04 lifecycle), which agrees with my forward treatment of it as
  downstream evidence.

## Batch consistency

`--batch` over the three sealed ledgers: PASS, 0 findings.

## Other

- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row
  (C9). Piping selects work through owner-steered work graphs.
- For R3: the decomposition's own OI-006 row still reads "TBD" at revision
  0.12, although DEC-022 ruled it.
- Rename residue: the SOW surface carries CP-04 (the "OpenPipeStress CONTRACT"
  label). The schema itself carries former-name identifiers (`$id`
  openpipestress.org, `rule_pack_kind` `open_pipe_stress_rule_pack`, grammar
  tokens). The SOW does not name them, so they are noted here for R3 and are
  not rows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
