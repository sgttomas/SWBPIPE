# B3 — Owner of runtime model-operation application, and the runtime edit contract

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Two linked questions:

1. **Who owns runtime application of model operations?** This is the Rust
   `operation_applier` apply path and the per-kind resolvers. It sits outside
   the DEL-16-02 and DEL-16-03 scopes, although DEC-020 places application in
   the applier seam.
2. **Which contract governs runtime edits** (T11 SS-02)? The choice is
   between the JSON Schema `schemas/model_operation.schema.json` and the Rust
   intent-structure check that the runtime actually applies.

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows for any
ownership move.

## 2. Background

**Earlier decisions.**
- **DEC-020** (`SD:611`; D-13, ruled 2026-06-11). Adopts the wasm32 build of
  `operation_applier` as the sole browser-mode operation engine, and retires
  the TypeScript mirror's validation and apply logic. It records ADR-0001. It
  does not name an owning deliverable for application.
- **SCA-003** ruled the container (T5A-C04 ledger notes).
- **DEL-16-02 REQ-16-02-005** places the "persistence/application API outside
  this slice". DEL-16-03's SOW also declares application outside its slice
  (T3-G1).

**What the code does now (freeze).**
- `apply_operation` is at `core/model_operations/operation_applier/src/lib.rs:866`.
- The crate emits `deliverable_refs` `["DEL-16-02","DEL-16-03"]` at
  `lib.rs:2003`.
- Runtime intents are checked by `check_intent_structure` at `lib.rs:2072`. The
  JSON Schema is not used. The Python preview engine validates with
  `Draft202012Validator`. Line 192 of
  `core/model_operations/validation_preview/engine.py` imports it (via
  T5A-C05; line checked).
- The PKG-16 verifier found "No issued key owns runtime application"
  (`WAVES/W1/PKG-16/PKG-16_VERIFICATION.md:356`, via T3-G1).
- The DEL-16-02 and DEL-16-03 SOWs present Python engines with test-only
  callers as the implementation (T5A-C04).

## 3. Options

**Question 1: application owner (T3-G1)**

| Option | Consequences |
|---|---|
| (a) Extend DEL-16-03 | It gave the UNKEYED apply-side answers (RC-16-0151, -0248, -0252) and holds the receipt and acceptance side. Its SOW's "outside this slice" text is amended. |
| (b) Extend DEL-16-02 | It owns validation and blocking before application and the Rust `OperationOutcome` (CAP-COREB-003). REQ-16-02-005 is amended. |
| (c) CREATE a PKG-16 application deliverable | New SOW, DAG node and keys. Both existing SOWs keep their "outside this slice" text, now pointing to a named owner. |

Under all three, the T5A-C04 record repair of the DEL-16-02/16-03 SOWs (naming
the applier seam) can proceed only after this ruling, and after A1 for the
Python-versus-Rust status (T5A-C04 preconditions).

**Question 2: runtime edit contract (SS-02, FG-DEL-16-01-01)**

| Option | Consequences |
|---|---|
| (a) The JSON Schema governs | Runtime validates intents against `model_operation.schema.json`. That is a code-fix candidate, not executed here. DEL-16-01 CLM-009.r01/.r03 and DEL-16-02 REQ-16-02-002 then close by code. |
| (b) The intent-structure contract governs | The requirement is restated for the Rust contract (R5 record repair of DEL-16-01 and DEL-16-02 text). The schema remains a published description. |

The DEL-16-02 REQ-16-02-002 row itself is a **C1** item ("DEL-16-02 runtime
schema validation", T5A-C05 non-MBF). B3 decides only the contract-ownership
framing. The option (a)/(b) intent ruling for that row is C1's. The two must
be ruled consistently (T5A-C05 decision 3 says "take this jointly with
FG-DEL-16-01-01").

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:611` DEC-020 | GOVERNING | Read at the freeze |
| `operation_applier/src/lib.rs:866`, `:2003`, `:2072` | EVIDENCE (freeze) | Checked by this writer |
| `PKG-16_VERIFICATION.md:356` | R2 verification report | Verifier finding, cited via T3 |
| DEL-16-03 reverse answers RC-16-0151/0248/0252 | R2 sealed ledger | Via T3-G1 |
| T3-G1, T5A-C04, T11 SS-02 | R3 PROPOSAL | Classification only |

T11 records the SS-02 finding as OBSERVED on DEL-16-02 REQ-16-02-002, with the
FIELD correction on DEL-16-01 CLM-009.r01. Nothing here is from a test run.

## 5. Affected claims

- **Class T5A-C04** (19 rows; Authority REVIEW; route R5_RECORD_REPAIR). This
  class is wholly within B3's subject and in no other topic list. Its rows
  are on the H4 route and carry BlockedOnPacket B3 (and A1). Filter:
  `CLASS_ASSIGNMENTS.csv` `ClassID == T5A-C04`. Keys:
  - DEL-16-02 (10): `DEL-16-02:SOW#CLM-005`; `DEL-16-02:SOW#CLM-006`;
    `DEL-16-02:SOW#CLM-012`; `DEL-16-02:SOW#CLM-013`;
    `DEL-16-02:SOW#CLM-019`; `DEL-16-02:SOW#CLM-022`;
    `DEL-16-02:SOW#CLM-026`; `DEL-16-02:SOW#CLM-010/REQ-16-02-005`;
    `DEL-16-02:SOW#CLM-010/REQ-16-02-007`;
    `DEL-16-02:SOW#completion-and-reliance-basis-epistemology/AC-001`.
  - DEL-16-03 (9): `DEL-16-03:SOW#CLM-005`; `DEL-16-03:SOW#CLM-006`;
    `DEL-16-03:SOW#CLM-009`; `DEL-16-03:SOW#CLM-013`;
    `DEL-16-03:SOW#CLM-016`; `DEL-16-03:SOW#CLM-020`;
    `DEL-16-03:SOW#CLM-026`; `DEL-16-03:SOW#CLM-028`;
    `DEL-16-03:SOW#completion-and-reliance-basis-epistemology/AC-001`.
- **Capabilities, T3-G1 (15, `T3_OWNERSHIP.csv`):** CAP-COREB-002, 004, 006,
  007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017 and CAP-SHELL-020.
  CAP-COREB-012 is LOW: its unowned part is the composite-support
  association metadata contract, and DEL-04-03 was never routed. The GUI for
  it is annex row 21, an H1 KEY_ISSUE.
- **SS-02 (T11_METHOD.csv rows `SS-02 Runtime edit contract`, `CAP-COREB-020`,
  `CAP-DATA-009`; route OWNER_DECISION):** 3 items.
- **Rows cross-referenced, not in my portion:**
  - `DEL-16-01:SOW#CLM-009.r01`, `.r03` (T6-C07, SCOPE_CHANGE_HANDOFF; H1;
    FIELD on .r01).
  - `DEL-16-02:SOW#CLM-010/REQ-16-02-002` (T5A-C05; C1; OBSERVED resolution,
    OtherCorrections give tier PROJECT_BASELINE; AuthorityNeeded OWNER is
    the row's current effective value).
- **Packages and deliverables.** PKG-16 (DEL-16-01, 16-02, 16-03), with
  DEL-07-09 and DEL-16-04 as relations.

## 6. Risks

- **Undecided.** The only mutating path of every editor operation has no
  accountable deliverable, no acceptance keys and no owner for its tests
  (T3-G1). The runtime reports "schema_validation" without JSON Schema
  validation (T5A-C05 risk). A reviewer reading the PKG-16 SOWs validates the
  test-only Python path and misses the product path (T5A-C04).
- **(1a)/(1b).** Either existing SOW takes a scope it currently disclaims.
  Its history needs R5 catch-up.
- **(1c).** One more PKG-16 node.
- **(2a).** A behaviour change on the product edit path. Tests needed.
- **(2b).** Ratifies the Rust check as the contract. The schema may drift
  further.
- **Text first.** A text catch-up done before the ruling would silently choose
  an engine (T5A-C04 preconditions).

## 7. Recommended routing

No recommendation; owner's call. The evidence supports one sequencing point:
rule question 1 and SS-02 before the T5A-C04 record repair (T5A-C04
precondition 1). Rule SS-02 together with C1's REQ-16-02-002 item.

## 8. On-ruling mechanism

- **(1a)/(1b)/(1c).** A scope-change handoff amending the chosen SOW or
  creating the deliverable, and issuing keys. Then R5 record repair of the
  reverse answers, and the T5A-C04 SOW revisions through the ordinary change
  path (separate R5 authorization).
- **(2a).** A CODE_FIX_CANDIDATE brief (H2) for runtime schema validation under
  a production brief.
- **(2b).** An R5 record repair restating DEL-16-01/16-02 requirement text.
- Nothing executes until the owner acts.

## 9. Dependencies

- **Depends on.** A1 (Python engines versus the Rust core; T5A-C04
  precondition 2).
- **Must be consistent with.** C1 (DEL-16-02 REQ-16-02-002).
- **Blocks.**
  - H4 rows of T5A-C04.
  - H1 items for T3-G1 and T6-C07's DEL-16-01 rows.
  - The H1 routing-gap items CAP-SHELL-019/022 → DEL-16-02 (T2-G5).
- **Related.** B2 (operations UI), B4 (DEC-094 routes palette commands through
  PKG-16).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
