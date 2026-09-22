# B10 — Plugin and adapter runtime, grant model, and the PDU-034 quarantine taxonomy

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Three linked selections that the records leave to the owner:
1. **Runtime execution model.** Should a plugin and adapter runtime be
   authorised and selected, or does the runtime stay unselected, so the gate
   remains deny-only?
2. **Grant model.** What permission grant model applies, with private-library
   and rule-pack scopes?
3. **Quarantine and signoff taxonomy.** Which quarantine and readiness
   taxonomy, reviewer and signoff taxonomy and destructive-workflow policy
   apply (PDU-034, PDU-004)?

These also decide whether the DEL-12-04 security and private-library helpers
are wired at product boundaries or kept as a design reference.

**Holder: OWNER.** Security review follows any wiring. That is REVIEW (H3),
not decided here.

## 2. Background

**Earlier decisions.**
- **DEC-012** (`SD:603`). Keeps dependency versions, public API transport,
  import/export formats and similar items as implementation-level TBDs unless
  a later ruling resolves them. T12-C04 records that the contested rows read
  the missing plugin runtime as held under DEC-012.
- **DEL-10-01 CLM-007.** The deliverable "records a contract boundary, not an
  implementation" (T12-C04).
- **PDU-034 and PDU-004.** These are proposed-deliverable-update identifiers
  from the July D-41 run. They reach this run only through the deliverables'
  own records (for example the DEL-17-09 `_STATUS.md` and MEMORY at the
  freeze). No DEC or register row rules them. The July material is CONTEXT
  only.

**What the code does now (freeze; lines checked by this writer).**
- `api/api_boundary_contract.yaml:27` carries `plugin_runtime` as `TBD`.
- `core/adapters/framework/adapter_framework.py:532` returns
  `BLOCKED_RUNTIME_NOT_SELECTED`. The dispatch gate never dispatches.
  `validate_adapter_declaration` is at `:539` and is called only by tests.
- `AdapterFrameworkPanel` (`apps/desktop/src/App.tsx:50`) builds its own
  preview packet (T12-C04).
- `core/security/secret_private_library/controls.py` is imported only by its
  tests (T12-C05). It keeps records metadata-only and secrets reference-only,
  blocks by default, and never promotes unknown status to public.
- `lib.rs::save_local_library` refuses quarantined records (FIELD correction
  on DEL-12-04 CLM-011.r06, via T6-C04).

## 3. Options

As T6-C04 D2/D3 and T12-C04/C05 state them:

| Option | Consequences for deliverables and code | Other packets |
|---|---|---|
| (a) Authorise and select a runtime execution model and grant model; select the quarantine taxonomy and destructive-workflow policy | Code briefs add the runtime and no-bypass tests, the grant model with private-library and rule-pack scopes, quarantine-routing tests and alignment of CHECKLIST_CATEGORIES. The DEL-12-04 helper is wired at import, report and export boundaries. DEL-02-04 and DEL-10-01 reopen, because lifting the hold is their trigger. | Security review items (H3). B12 D4 (storage roots and secret provider) becomes a prerequisite for wiring. B7's pattern applies to DEL-10-02. |
| (b) Keep the runtime unselected and the hold standing | R5 narrows or annotates the claims: the gate stays deny-only, the helper is recorded as reference-only, and REQ-007 of DEL-17-09 is amended. The 50 T12-C04 contract rows stay NO_ACTION. | The DEL-10-02 panel versus engine question still follows the B7 decision (T12-C04). |
| (c) Split: select the taxonomy (D3) now and hold the runtime (D2) | Quarantine routing and review can be specified and tested on the existing import path. The runtime stays deny-only. | Partial wiring of the DEL-12-04 helper for quarantine only. |

Option (c) is not stated as such in one task output. It combines the D2
option (b) with the D3 option (a), each of which T6 states separately.

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:603` DEC-012 | GOVERNING | Read |
| `api_boundary_contract.yaml:27`, `adapter_framework.py:532`, `:539`, `App.tsx:50` | EVIDENCE (freeze) | Checked by this writer |
| PKG-10 and PKG-12 sealed ledgers; `PKG-12_VERIFICATION.md` (9.5% outlier, F1/CP-11/F7 pattern) | R2 records | PKG-12 rows carry a known higher error rate (W3 assessment) |
| Adopted FIRM/FIELD/OBSERVED resolutions on DEL-12-0x rows | R2 adopted resolutions | Adopted |
| PDU-034/PDU-004 origin | CONTEXT (July run) | Not evidence |
| T6-C04 D2/D3; T12-C04, T12-C05 | R3 PROPOSAL | Classification |

## 5. Affected claims

- **Class T6-C04** (87 rows; split between **B10** and **B12**). B10's portion
  is decision groups **D2 and D3, 20 rows**. The filter is the explicit key
  list below; B12 takes the other 67. Dispositions: 19 PARTIALLY_IMPLEMENTED and 1
  DOCUMENTED_UNIMPLEMENTED (`DEL-17-09:STATUS#remaining/R03`).
  - **D2** (14):
    - `DEL-00-07:AB#normative-requirements/REQ-07-03`;
    - `DEL-12-01:SOW#CLM-010/LFSP-REQ-010`;
    - `DEL-12-01:SOW#CLM-024.r06` (FIRM: AuthorityNeeded NO → OWNER);
    - `DEL-12-02:SOW#CLM-004`;
    - `DEL-12-02:SOW#CLM-012/REXC-REQ-012`;
    - `DEL-12-02:SOW#CLM-027` (FIRM: NO → OWNER);
    - `DEL-12-04:SOW#CLM-004.r06`;
    - `DEL-12-04:SOW#CLM-011.r07` (FIELD: tier PROJECT_BASELINE → INVARIANT);
    - FG-DEL-10-02-01 (6): `DEL-10-02:SOW#CLM-011/REQ-10-02-02`,
      `DEL-10-02:SOW#CLM-011/REQ-10-02-03`,
      `DEL-10-02:SOW#CLM-011/REQ-10-02-04`,
      `DEL-10-02:SOW#CLM-011/REQ-10-02-05`,
      `DEL-10-02:SOW#CLM-011/REQ-10-02-07`,
      `DEL-10-02:SOW#CLM-011/REQ-10-02-08`.
  - **D3** (6):
    - `DEL-12-04:SOW#CLM-004.r05`;
    - `DEL-12-04:SOW#CLM-007.r05`;
    - `DEL-12-04:SOW#CLM-011.r06` (FIELD);
    - `DEL-12-04:SOW#CLM-024` (FIRM: NO → OWNER; PRODUCT_CALLER NONE);
    - `DEL-17-09:SOW#CLM-013/DEL-17-09-REQ-007` (FG-DEL-17-09-02);
    - `DEL-17-09:STATUS#remaining/R03` (OWNER_HOLD).
  - **Rows known only from OtherCorrections.** Three rows reached T6-C04 only
    through FIRM AuthorityNeeded corrections (DEL-12-01 CLM-024.r06, DEL-12-02
    CLM-027, DEL-12-04 CLM-024). One takes its INVARIANT tier from a FIELD
    correction (DEL-12-04 CLM-011.r07).
- **T12-C04** (57 rows; filter `T12_UNREACHED.csv` `ClusterID == T12-C04`):

  | Deliverable | Rows | Route |
  |---|---|---|
  | DEL-02-04 | 33 | NO_ACTION |
  | DEL-10-01 | 16 | NO_ACTION |
  | DEL-00-07 | 1 | NO_ACTION |
  | DEL-10-02 | 7 | OWNER_DECISION |

  The NO_ACTION rows are listed because this ruling is their reopening
  trigger. Four are CONTESTED:
  - `DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-12`;
  - `DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-18`;
  - `DEL-10-01:SOW#CLM-006`;
  - `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-05`.
- **T12-C05** (27 rows, DEL-12-04, all OWNER_DECISION; filter `ClusterID ==
  T12-C05`). It overlaps T6-C04 on one key, `DEL-12-04:SOW#CLM-024`, which is
  counted once. The one divergent row is that same CLM-024.
- **Distinct B10 keys:** 20 + 57 + 27 − 1 = **103**.
- **Packages.** PKG-00, PKG-02, PKG-10, PKG-12, PKG-17.

## 6. Risks

- **Undecided.** The claims stay partially met indefinitely. 11 of the 20
  T6-C04 rows are at INVARIANT tier:
  - 7 in D2 (6 by effective tier, plus DEL-12-04 CLM-011.r07 after its FIELD
    correction);
  - 4 in D3.

  This matches T6's D2 (7) and D3 (4). The security boundary is tested in
  isolation and guards no product path, so it could be mistaken for enforced
  behaviour.
- **(a).** A new attack surface (plugin execution). Needs security review
  and no-bypass tests before any release claim.
- **(b).** Low risk while the hold stands (T12-C04). If a runtime is built
  later without these contracts, the tested guards could be bypassed.
- **(c).** Quarantine semantics are fixed before the grant model and may need
  revision.

## 7. Recommended routing

No recommendation; owner's call.

## 8. On-ruling mechanism

- **(a).** Owner ruling recorded as a DEC, with PDU-034/PDU-004 dispositions
  recorded against it. Candidate code-fix briefs (runtime, grant model,
  quarantine routing, helper wiring) go through the change path under a
  production brief. They stay unexecuted until the owner selects them into a
  work graph. The security review is registered in H3.
- **(b).** A DEC recording that the hold stands, then R5 record repair that
  narrows or annotates the 20 T6-C04 rows and records the DEL-12-04 helper as
  reference-only. The T12-C04 NO_ACTION rows are unchanged.
- **(c).** A DEC selecting the taxonomy only, then code briefs for D3 and R5
  annotation of D2.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Coordinates with.**
  - B12 D4 (storage roots and secret provider are a precondition for wiring
    the DEL-12-04 helper).
  - B7 (DEL-10-02 panel versus engine follows B7's pattern).
  - A8 (secret provider, CF-001/CF-002; see the B12 overlap note).
- **Blocks.**
  - Code-fix briefs on the adapter gate and the security helper (H2).
  - The three DEL-06-02 CP-11 rows whose no-bypass branch waits on the D2
    runtime (T6-C03, T6 observation O5; CFB-15; H2 marks them B10):
    `DEL-06-02:SOW#CLM-006.r05`, `DEL-06-02:SOW#CLM-013/REQ-06-02-010` and
    `DEL-06-02:SOW#CLM-016/REQ-06-02-010`.
  - The H2 row `DEL-17-09:CONTEXT#description` (T6-C01, CFB-54; H2 adds
    B10).
  - R5 narrowing of the 20 rows (H4).
- **Depends on.** None.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
