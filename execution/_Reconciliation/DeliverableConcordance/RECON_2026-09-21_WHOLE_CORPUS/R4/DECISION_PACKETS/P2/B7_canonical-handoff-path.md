# B7 — Canonical handoff and export path: wire the Python engines, port them, or treat the desktop previews as the product

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

For each handoff and export record family in PKG-15 and PKG-17, which
implementation is canonical? The choice is between the tested Python builder
and schema (reached only from tests), a port into the desktop or Rust path,
and the desktop technical preview the product actually emits. This covers
PKG-15 schema compliance ("no product path emits schema-compliant handoff
packages") and the extra field the desktop prover packet adds, which its
strict schema forbids.

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows for option
(c). A production brief through the change path follows for options (a) and
(b).

## 2. Background

**Earlier decisions.**
- **DEC-009** (`SD:600`). Adopts a Rust core and application services with
  a TypeScript/React GUI. Whether Python engines are permitted is topic
  **A1** (T7-C03, T8 DEC-009), which B7 does not decide.
- **DEC-010** (`SD:601`). Schema-first envelopes and canonical JSON (the hash
  basis is topic A2).
- **Precedent.** DEL-03-07's product calls a Rust port of the same contract
  that its Python checker implements (T12-C01 option (b); T12-C06).
- **F7** (W1 gate ruling, Effect 3). A tested engine with no product caller
  satisfies a claim about that engine, not a claim about app behaviour. This
  is why 197 T12-C01 engine rows are ALIGNED while the product claims are
  partial.

**What the code does now (freeze; lines checked by this writer).**
- `build_target_mapping_contract` is at `core/handoff/target_mapping/contract.py:67`.
  Only tests import it. The desktop builds its own `target_mapping` object at
  `apps/desktop/src/features/handoff/HandoffPanel.tsx:236`.
- `build_native_json_export_package` is at `core/handoff/native_json/package.py:78`.
  No app or runtime module imports it. `NativePackagePanel` builds its own
  review record.
- The PKG-15 verifier (`WAVES/W3/PKG-15/PKG-15_VERIFICATION.md:416-424`)
  found:
  - no product code calls any `core/handoff` module;
  - HandoffPanel's export does not follow `handoff_package.schema.json`
    (FG-DEL-15-03-01);
  - the desktop external-prover packet adds `unit_policy_evidence`, which its
    strict schema does not allow (FG-DEL-15-04-01).
- Stress-neutral already splits by version. The Python 0.1 builder is
  test-only, and the product emits the 0.2 family (DEL-17-06 Notes; D-67).

## 3. Options

Options (a)–(c) are as T12-C01 states them. The owner may choose per record
family (target mapping, handoff package, exporter, external-prover metadata,
native JSON, MBF, external run, stress-neutral, PCF, review geometry,
export-adapter SDK).

| Option | Consequences for deliverables and code | Consequences for other packets |
|---|---|---|
| (a) Wire the desktop export to call, or validate against, the Python builder and schema through a runtime service | A production brief adds a runtime service. Desktop exports become schema-validated. The 16 divergent DEL-15-02/15-04 engine defects become product defects and must be fixed first (T12 observation 1). | Needs A1 to permit Python engines in the product path. Interacts with A2 (hash labels). |
| (b) Port the contract into the desktop or Rust path, and keep the Python builder as a parity oracle | A production brief ports each contract. Parity tests are needed. This follows the DEL-03-07 precedent. The guards (silent defaults, provenance substitution, presence-only units) are carried into the port, not fixed in the Python engine. | Consistent with DEC-009 Rust core (A1 "port"). The Python engines' status then follows T12-C06 (ENGINEERING, H3). |
| (c) Declare the desktop preview canonical, and retire or re-scope the Python builders | Scope change re-scopes DEL-15-0x and DEL-17-0x SOWs to the desktop record. Schema-validated guarantees are lost unless restated for the preview. The prover-packet field becomes a schema amendment. | Makes the B6 panels the product surface. Code-fix briefs on the Python engines lapse. |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:600`, `SD:601`; W1 gate ruling F7 | GOVERNING / owner ruling | Read |
| `target_mapping/contract.py:67`, `HandoffPanel.tsx:236`, `native_json/package.py:78` | EVIDENCE (freeze) | Checked by this writer |
| `PKG-15_VERIFICATION.md:410-432`; `PKG-15_RERUN1_VERIFICATION.md:155-170` | R2 verification reports | Defects "confirmed by code reading, not by a test" |
| W3 assessment owner items (PKG-15) | R2 gate record | Collected owner item |
| T12-C01 | R3 PROPOSAL | Engine attribution by first code path |

Known only from code reading: the silent-default and provenance defects in the
target-mapping builder. T12 observation 3 notes that the marker population is
a lower bound. 10 more ALIGNED rows cite target_mapping without the F7 marker.

## 5. Affected claims

No class in `CLASS_INDEX.csv` is in B7's portion. The source cluster is T12's.
- **T12-C01, all 213 rows.** Filter: `T12_UNREACHED.csv` `ClusterID ==
  T12-C01`.

  | Package | Deliverable | Rows | Route |
  |---|---|---|---|
  | PKG-15 | DEL-15-01 | 32 | OWNER_DECISION |
  | PKG-15 | DEL-15-02 | 46 | 31 OWNER_DECISION + 15 CODE_FIX_CANDIDATE |
  | PKG-15 | DEL-15-03 | 16 | OWNER_DECISION |
  | PKG-15 | DEL-15-04 | 28 | 27 OWNER_DECISION + 1 CODE_FIX_CANDIDATE |
  | PKG-17 | DEL-17-03 | 27 | OWNER_DECISION |
  | PKG-17 | DEL-17-04 | 36 | OWNER_DECISION |
  | PKG-17 | DEL-17-05 | 22 | OWNER_DECISION |
  | PKG-17 | DEL-17-06 | 2 | OWNER_DECISION |
  | PKG-17 | DEL-17-07 | 1 | OWNER_DECISION |
  | PKG-17 | DEL-17-08 | 1 | OWNER_DECISION |
  | PKG-17 | DEL-17-09 | 2 | OWNER_DECISION |
  | | **Total** | **213** | **197 OWNER_DECISION; 16 CODE_FIX_CANDIDATE** |
- **The 16 CODE_FIX_CANDIDATE rows** are on the H2 route (classes T7-C06 ×12,
  T6-C03 ×3, T6-C01 ×1). H2 marks them BlockedOnPacket B7.
  - They include DEL-15-02:SOW#CLM-020 (FIRM; the verifier offers
    re-disposition or a new `.s04` split).
  - The DEL-15-02 CLM-005 tier is contested evidence (W3 Departures item 5).
  - DEL-15-02:SOW#CLM-005.r01 is also in T8 UNIT_VOCABULARY (C7).
- **Both views (T8 against T12).** `DEL-15-04:SOW#completion-and-reliance-basis-epistemology/AC-001`:
  - T12 view: OWNER_DECISION.
  - T8 K7 view: NO_ACTION, reading PARTIALLY_IMPLEMENTED ·
    DEFERRED_BY_RULING.
  - It is CONTESTED, is in `T8_ROUTE_DISAGREEMENTS.csv` as NOT_DIVERGENT, and
    B12 D1 is related.
- **W3 PKG-15 product rows, cross-referenced and not in my portion.**
  FG-DEL-15-03-01 (9 rows) and FG-DEL-15-04-01 (`DEL-15-04:SOW#CLM-004.r01`)
  are class T6-C02 (CODE_FIX_CANDIDATE, H2). Their fix shape depends on this
  ruling, so they are BlockedOnPacket B7.
- **Rows known only from OtherCorrections.** The FIRM on DEL-15-02 CLM-020 and
  the WEAK on DEL-15-04 OUT-001 (AuthorityNeeded NO, two readings surviving).

## 6. Risks

- **Undecided.** Two record shapes for one handoff concept drift apart. The
  schema-validated guarantees (provenance, stable IDs, loss reports, blocking
  diagnostics) are tested only where the product does not run. The desktop
  preview carries no schema conformance check. The prover packet violates its
  own strict schema.
- **(a).** Adds a runtime service, and possibly Python, to the product path.
  That conflicts with DEC-009 unless A1 permits it. Engine defects become live.
- **(b).** Porting effort. The Python and port twins need parity checks, or
  they drift silently (T12-C06 risk).
- **(c).** Guarantees exist only where re-stated. Records may overstate what
  the product enforces until SOWs are re-scoped.

## 7. Recommended routing

No recommendation; owner's call. Two sequencing points follow from the
evidence:
- H2 code briefs on the DEL-15-02 builder should follow this ruling, not
  precede it (T12 observation 1).
- B7 and A1 should be ruled together or A1 first, because option (a)
  presupposes Python in the product path.

## 8. On-ruling mechanism

- **(a) or (b).** A code brief through the WORKING_ITEMS change path
  (chirality-change) under a production brief, per record family. The
  FG-DEL-15-02-01/02/03 repairs and the FG-DEL-15-03-01 and FG-DEL-15-04-01
  gaps join it. For (b), a parity-check brief is added.
- **(c).** A scope-change handoff that re-scopes the DEL-15-0x and DEL-17-0x
  SOWs to the desktop record (and amends the prover schema if the field is
  kept). R5 record repairs follow, and the lapsed code briefs are withdrawn.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Depends on.** A1 (DEC-009 engine placement). A2 (hash basis and "JCS"
  labels on these records).
- **Blocks.**
  - The H2 briefs for the 16 T12-C01 divergent rows.
  - FG-DEL-15-03-01 and FG-DEL-15-04-01.
  - The DEL-15-02 defaults (topic file H2 example).
- **Related.**
  - B6 (option (c) makes the preview panels the product surface).
  - B5 item 2b (native package panel).
  - B8 and B9 (same pattern for PKG-14 and PKG-13).
  - B10 (DEL-10-02 follows this pattern).
  - A7 (export plan pointers on PKG-17 rows).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
