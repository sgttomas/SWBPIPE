# B8 — Do model-state persistence and comparison enter the product?

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Does the product persist named model states and compute analysis-run and
model-state comparisons through the PKG-14 engines (and the DEL-08-06 report
sections)? Or are the desktop TypeScript previews the product?

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows for option
(c). A production brief follows for (a) or (b).

## 2. Background

**Earlier decisions.**
- **D-21 / DEC-056.** Adopted the v0.2 milestone set, which inserted "v0.2 R3
  States/Comparison" (register row D-21, RULED 2026-07-02). D-29 carries the
  scope-change propagation as residual work. This is the scope basis for
  PKG-14.
- **DEC-009** (`SD:600`). Rust core. The Python engines' status is A1.
- **DEC-028** (`SD:619`). The native package is a multi-member archive, and
  the compatibility window is left open (B12 D10). The desktop project store
  persists model and run envelopes but no model-state records (T12-C02).

**What the code does now (freeze; lines checked by this writer).**
- `compare_analysis_runs` is at `core/comparison/analysis_run/engine.py:165`.
  It is reached only from tests.
- `compare_model_states` is at `core/comparison/model_state/engine.py:64`. It
  is called only by its tests.
- The desktop Comparison panel
  (`apps/desktop/src/features/comparison/ComparisonPanel.tsx`) and the report
  projection (`apps/desktop/src/features/report/stateComparisonHandoffSections.ts`)
  are separate TypeScript modules (T12-C02).
- DEL-14-01:SOW#CLM-011.r01 records the product-behaviour gap: no desktop
  command saves a named model state.

## 3. Options

Options (a)–(c) are as T12-C02 states them.

| Option | Consequences for deliverables and code | Consequences for other packets |
|---|---|---|
| (a) Wire the engines through a runtime service | A production brief adds a runtime service plus desktop commands to save and compare states. The model-state schema becomes a persisted product record. | Needs A1 to permit Python in the product path. The DEC-009 hash-basis choice (A1/A2) then binds a real product record. B12 D7 (comparison-result schema) becomes live. |
| (b) Port to the desktop or Rust path, and keep the Python engines as parity oracles | A production brief ports and adds parity checks. | Consistent with DEC-009. The Python engines follow T12-C06 status (H3). |
| (c) Accept the TypeScript previews as the product and re-scope PKG-14 | Scope change re-scopes DEL-14-01/03/04/05 and DEL-08-06. Model-state persistence may leave scope, or be restated for the desktop store. | May conflict with the D-21 v0.2 States/Comparison milestone. If so, it needs a milestone amendment through scope-change (D-29 propagation). |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| Register D-21, D-29; `SD:600`, `SD:619` | GOVERNING | Read |
| `analysis_run/engine.py:165`, `model_state/engine.py:64` | EVIDENCE (freeze) | Checked by this writer |
| PKG-14 sealed ledgers (W3) | R2 sealed ledgers | Effective values from `CORPUS_CLAIMS.csv` |
| T12-C02 | R3 PROPOSAL | Engine attribution by first code path |

T12 observation 3 notes that 7 further ALIGNED rows cite `analysis_run`
without the F7 marker. The population is a lower bound.

## 5. Affected claims

No class in `CLASS_INDEX.csv` is in B8's portion.
- **T12-C02, all 95 rows.** Filter: `T12_UNREACHED.csv` `ClusterID ==
  T12-C02`.

  | Package | Deliverable | Rows | Route |
  |---|---|---|---|
  | PKG-14 | DEL-14-01 | 14 | OWNER_DECISION |
  | PKG-14 | DEL-14-03 | 21 | 20 OWNER_DECISION + 1 CODE_FIX_CANDIDATE |
  | PKG-14 | DEL-14-04 | 30 | OWNER_DECISION |
  | PKG-14 | DEL-14-05 | 22 | OWNER_DECISION |
  | PKG-08 | DEL-08-06 | 8 | OWNER_DECISION |
  | | **Total** | **95** | |
- **Both views** (a T12 row that is also a class row or a T8 row):
  - `DEL-14-01:SOW#CLM-004`, `SOW#CLM-011.r01`, `SOW#CLM-011.r04`:
    - T12 view: OWNER_DECISION;
    - class view: T6-C02 (CODE_FIX_CANDIDATE, H2).
    - H2 should mark them BlockedOnPacket B8.
    - CLM-004 and CLM-011.r04 are hash-basis rows. In H2 they carry A2.
    - The T12-C02 prose associates them with the DEC-009 cluster. However,
      `T8_ROWS.csv` holds only CLM-005 in that cluster, and A1 does not list
      them, so no A1 routing is claimed here.
  - `DEL-14-01:SOW#CLM-005`:
    - T12 view: OWNER_DECISION;
    - class view: T6-C02 (CODE_FIX_CANDIDATE);
    - T8 view: DEC-009, OWNER_DECISION;
    - it is CONTESTED and appears in `T8_ROUTE_DISAGREEMENTS.csv`.
    - A1 owns the DEC-009 reading. H2 should also mark this row
      BlockedOnPacket B8.
  - `DEL-14-01:SOW#CLM-024`:
    - T12 view: OWNER_DECISION;
    - T8 view: F1_ON_CONTEXT, CODE_FIX_CANDIDATE;
    - class view: NOT_DIVERGENT.
    - It is CONTESTED. C7 owns the F1 reading.
  - `DEL-14-03:SOW#CLM-012/REQ-14-03-001`:
    - T12 view: CODE_FIX_CANDIDATE;
    - class view: T6-C01 (CODE_FIX_CANDIDATE).
    - The comparison tests do not validate against the state schema.
- **Also CONTESTED.** `DEL-08-06:SOW#completion-and-reliance-basis-epistemology/AC-001`.
  PKG-08 judges AC-001's subject differently per deliverable. This is part of
  the corpus-level F7 question (C6).
- **Related B12 rows.** D7 (DEL-14-04 FG-DEL-14-04-01, the comparison-result
  schema). Those rows are in B12's portion, not here.

## 6. Risks

- **Undecided.** Comparison and state guarantees are checked only off the
  product path. The DEC-009 hash-basis choice cannot bind any product record
  while no product record exists (T12-C02). DEL-14-01's product gap stays
  open.
- **(a).** Python in the product path, and a new runtime service.
- **(b).** Port effort and twin drift.
- **(c).** Possible conflict with the D-21 milestone set, which needs an
  explicit amendment.

## 7. Recommended routing

No recommendation; owner's call. The evidence supports ruling B8 with or
after A1, and consistently with B7, because the three options mirror B7's.

## 8. On-ruling mechanism

- **(a)/(b).** A code brief through the change path under a production brief.
  The DEL-14-01 T6-C02 rows and DEL-14-03 REQ-14-03-001 join it.
- **(c).** A scope-change handoff re-scoping DEL-14-01/03/04/05 and DEL-08-06,
  plus a milestone amendment if D-21's States/Comparison scope changes. R5
  record repairs follow.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Depends on.** A1 (DEC-009). A2 (hash basis).
- **Coordinates with.** B7 (same pattern), B12 D7 (comparison-result schema),
  B12 D10 (compatibility window).
- **Blocks.** The H2 briefs on the DEL-14-01 T6-C02 rows and DEL-14-03
  REQ-14-03-001.
- **Related.** C7 (F1 on CONTEXT for DEL-14-01 CLM-024), C6 (the F7
  corpus-level ruling for DEL-08-06 AC-001).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
