# B9 — PKG-13 product status: wire, hold or retire

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Is the PKG-13 design-knowledge, constraint and transform stack in product
scope for the current baseline? The options are to wire it into the product,
hold it as a staged foundation, or retire it. The decision includes two
product symptoms. The desktop Knowledge panel bypasses the design-knowledge
schema, and the desktop shows constraint-validation status that no engine
produces and nothing owns.

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows for
retirement or merger. A production brief follows for wiring.

## 2. Background

**Earlier decisions.** No ruling selects PKG-13's product role. The PKG-13
verifier says "No ruling permits this, so it goes to you at R4"
(`WAVES/W3/PKG-13/PKG-13_VERIFICATION.md:488-491`). DEC-009's Python/Rust
question for DEL-13-03/13-04 (class T7-C03) is topic A1.

**What the code does now (freeze).**
- `validate_constraint_envelope` is at `core/constraints/validation/engine.py:157`.
  Only `tests/test_constraint_validation.py` imports it (line checked; importer
  per T12).
- `transform_physical_to_analytical` is at
  `core/model_transform/physical_to_analytical/contract.py:113`. It is called
  only from tests (line checked).
- The design-knowledge and constraint schemas are loaded only by their
  structural tests (T12-C03).
- The Knowledge panel is mounted at `apps/desktop/src/App.tsx:904` (checked).
  It reads a preview-shaped fixture with string provenance and does not
  validate it against the schema (`PKG-13_VERIFICATION.md:493-495`).
- `DesignWorkspacePanel.tsx:204-209` hard-codes a constraint warning count and
  a blocking flag as "core_contract_evidence". The `lib.rs` proposal and
  preview payloads carry `constraint_validation` labels (`:1895`, `:4213`).
  No PKG-13 deliverable claims these surfaces, and no engine produces them
  (`PKG-13_VERIFICATION.md:290-300`, `:463-468`). Unlike T12-C01/C02, there
  is no desktop counterpart engine.

## 3. Options

Options (a)–(c) are as T12-C03 states them. The two symptoms are handled
under each.

| Option | Stack | Knowledge panel | Desktop constraint status | Other packets |
|---|---|---|---|---|
| (a) Wire constraint validation and the transform into the authoring or solve path | A production brief adds the runtime path. Records become true product guards. | Validate against `design_knowledge.schema.json` (code brief) | Produced by the engine | Needs A1 if the engines stay Python |
| (b) Hold as a staged foundation with no product obligation now, and record it as such | R5 marks DEL-13-01..04 foundation-only. The lifecycle check goes to T9. | Remove the schema claim from the panel, or label it as preview (C1 decides the REQ-13-01-011 intent) | Remove, or label as illustrative. It must not be presented as "core contract evidence" | — |
| (c) Retire or merge through scope change | Scope change retires or merges the PKG-13 deliverables | The panel is re-scoped or withdrawn | Removed | Affects the DEL-15-0x exporter that names the transform contract as a path string |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `constraints/validation/engine.py:157`, `physical_to_analytical/contract.py:113`, `App.tsx:904` | EVIDENCE (freeze) | Checked by this writer |
| `DesignWorkspacePanel.tsx:204-209`, `lib.rs:1895`, `:4213` | EVIDENCE (freeze) | Cited by the PKG-13 verifier; not re-read here |
| `PKG-13_VERIFICATION.md` §6, §8 items 3–5; `PKG-13_RERUN1_VERIFICATION.md` | R2 verification reports | Verifier findings |
| W3 assessment owner items (PKG-13; "INVARIANT, OWNER") | R2 gate record | Collected owner item |
| T12-C03 | R3 PROPOSAL | Classification |

T12 observation 3 records 29 more ALIGNED rows that cite
`core/constraints/validation` without the F7 marker. They were not sampled.

## 5. Affected claims

No class in `CLASS_INDEX.csv` is in B9's portion.
- **T12-C03, all 112 rows.** Filter: `T12_UNREACHED.csv` `ClusterID ==
  T12-C03`.

  | Deliverable | Rows | Route |
  |---|---|---|
  | DEL-13-01 | 25 | 24 OWNER_DECISION + 1 REVIEW |
  | DEL-13-02 | 52 | OWNER_DECISION |
  | DEL-13-03 | 21 | OWNER_DECISION |
  | DEL-13-04 | 14 | OWNER_DECISION |
- **Both views:**
  - `DEL-13-01:SOW#CLM-005.r05`:
    - T12 view: REVIEW;
    - class view: T7-C06 (CODE_FIX_CANDIDATE);
    - T8 view: UNIT_VOCABULARY, CODE_FIX_CANDIDATE.
    - It is FIRM, with AuthorityNeeded REVIEW (add `force_per_length`, or
      derive from the units schema).
  - `DEL-13-03:SOW#CLM-005.r04`:
    - T12 view: OWNER_DECISION;
    - class view: T6-C01 (CODE_FIX_CANDIDATE).
    - It is WEAK, and holds only on a paths reading (C6 F7 ruling).
- **Cross-referenced, not in my portion:**
  - `DEL-13-01:SOW#CLM-009/REQ-13-01-011` (T7-C05; the Knowledge panel intent;
    topic **C1**). Its FIELD correction is known only from OtherCorrections.
  - DEL-13-03/13-04 T7-C03 rows (A1).
  - T6-C04 D7 rows DEL-13-03 CLM-013.r06, CLM-028.r06 and DEL-13-04
    CLM-014.s02 (B12; result-envelope home).
  - DEL-13-04 CLM-017/REQ-007 and STATUS R03 (engineering validation; H3/C3).
- **Desktop constraint status.** No claim row carries it. It is recorded only
  in the PKG-13 verification report §6. It is an UNOWNED surface, not a row.
- **CONTESTED/OBSERVED visible.** DEL-13-02 CLM-003.r15 and CLM-023
  (CONTESTED); CLM-008 and CLM-010/R-13-02-006 (OBSERVED).

## 6. Risks

- **Undecided.** Constraint and provenance validation exists but guards no
  product input. The desktop presents fixed constraint counts as core contract
  evidence, so a reader may believe product data is constrained when it is not.
  The Knowledge panel's schema bypass sits at INVARIANT tier.
- **(a).** A significant new product path. Python placement per A1.
- **(b).** The foundation label must be applied consistently, or records
  overstate. The desktop constraint display still needs a code change.
- **(c).** Loses tested contract work. Downstream path references (the
  exporter) need repair.

## 7. Recommended routing

No recommendation on the stack; owner's call. The evidence supports one point
under every option: the hard-coded constraint status shown as "core contract
evidence" has no producing engine. Each option therefore implies either
wiring (a) or removal or labelling (b)/(c). The display cannot stay as is
under any option.

## 8. On-ruling mechanism

- **(a).** A code brief through the change path under a production brief
  (constraint validation, transform, Knowledge panel schema validation,
  desktop status from the engine).
- **(b).** R5 record repair marking DEL-13-01..04 foundation-only, and the
  T9 lifecycle check. A CODE_FIX_CANDIDATE brief (H2) removes or relabels the
  desktop constraint status, and C1 decides the panel intent.
- **(c).** A scope-change handoff (retire or merge), then R5 and code removal
  briefs.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Depends on.** A1 (T7-C03 rows for DEL-13-03/13-04).
- **Coordinates with.**
  - C1 (REQ-13-01-011 intent).
  - B7 (the handoff exporter names the transform contract).
  - B12 D7 (result-envelope home for ValidationResult and TransformResult).
  - C7 (unit-vocabulary reading for DEL-13-01 CLM-005.r05).
- **Blocks.** H2 briefs touching the desktop constraint display and the
  Knowledge panel. H4 foundation-only record repairs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
