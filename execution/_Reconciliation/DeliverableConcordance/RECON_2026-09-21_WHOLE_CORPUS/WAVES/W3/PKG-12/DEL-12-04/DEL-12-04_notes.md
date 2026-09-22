# DEL-12-04 notes — Secret and private-library handling (W3, PKG-12, worker G2)

Forward ledger sealed: `DEL-12-04_forward.csv`, SHA-256
`85e41a75ee4bcf89a44ddabad82dcdbe51cae7fdaecd27bf48ec07f4baff6f09`
(113 rows: 59 required keys, optional blocks CLM-004, CLM-006, CLM-007,
CLM-011, CLM-013 and CLM-019 split all-or-none, and sub-claims `SOW.s01`,
`CLM-018.s01`, `CLM-023.s01` and `CONTEXT#architecture-basis-injection.s01/.s02`).
All evidence was read from the freeze checkout at `00115c719`. Every value here
is an agent judgment, not an owner ruling.

## Path aliases

- The helper is `core/security/secret_private_library/` (`controls.py`,
  `__init__.py`). Only `tests/security/test_secret_private_library_handling.py`
  imports it; no product caller exists (F7).
- The SOW cites project documents as bare `docs/...`. The ledger cites them
  explicitly as `projects/chirality-piping/docs/...`, because the root has
  files with the same names.
- SOW references such as `_run_records/...` and "package fan-in" resolve to
  the deliverable's and PKG-12's `1_Working/_run_records/` folders.
- The desktop surface `apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx`
  carries `deliverable_id: "DEL-12-04"`, but the SOW never names it. Its
  references and guard decisions are hard-coded invented values; it does not
  call the Python helper.

## Judgment calls

- **Engine and product (F7).** Claims about the helper or its tests are
  `ALIGNED` with `PRODUCT_CALLER: NONE`. Claims about product behaviour are
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`: conditions CLM-004.r01–r06,
  requirements R1, R4–R8 and R10, and example block CLM-027.
- **Behaviour owned elsewhere.** R2 (telemetry) is `ALIGNED`, because the
  DEL-12-03 default-off policy and pre-payload guard satisfy it at product
  level. R9 (identity-only rule-pack references) is `ALIGNED` through the
  reporting audit manifest (MEDIUM confidence). The owners are named in Notes
  and in the reverse file (`COVERS`).
- **Telemetry test claims (FG-DEL-12-04-01).** CLM-007.r03, CLM-018.s01 and
  CLM-023.s01 say DEL-12-04's focused tests cover telemetry exclusion. The
  helper, its tests, its documentation and its June 7 run record contain no
  telemetry case at all. Tier `LOCAL_DESIGN · RECORD`, because the
  OPS-K-PRIV-2 boundary holds under DEL-12-03 (F8).
- **Quarantine (INVARIANT).** CLM-004.r05, CLM-007.r05 and CLM-011.r06 are
  `INVARIANT · IP_DATA`. The helper blocks `protected_suspected` records, but
  it routes nothing, no test exercises that status, and the taxonomy is held
  under PDU-034. STATUS remaining item R01 names CLM-011.r06 as its
  OPEN_ACTION.
- **CLM-006 registry fields.** The block text says the proposal names do not
  bind. The purposes of r01–r07 and r10 are met under different field names.
  Two fields are `IMPLEMENTED_DIFFERENTLY · DOC_BEHIND_CODE`: `credential_ref`
  (r08), which the helper models as a separate record kind, and
  `default_transmission_allowed` (r09), which it derives as a guard posture.
- **Settled TBD (FG-DEL-12-04-02).** SCA-003 settled the physical project
  package/container. CLM-025 and `CONTEXT…injection.s02` still say it is TBD,
  so both are `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`.
  Statements that only say "not resolved by this specification" (CLM-014),
  "does not finalize" (CLM-023), "out of scope" (CLM-010) or "deferred to
  owning workflows" (CLM-004.r08) remain accurate. The same stale element also
  appears inside CLM-017 and CF-002, which are non-aligned for other reasons.
- **Conflict rows CF-001 and CF-002** are `UNKNOWN · AUTHORITY_UNCLEAR`
  (C6(f)). The governing sources are silent (PRD open question 11 only asks),
  and both rows record Human ruling TBD.
- **Remaining R02** is `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED`. No governing
  row carries RF-001, so F2 does not allow ALIGNED with OPEN_ACTION.
- **CLM-013.r06** (permission path) is `ALIGNED` with `GAP_WORDING_CHECKED`.
  Its signal is documentation-level deny-by-default, which holds. Grant
  persistence is excluded from scope by CLM-010.
- **DEC-051 (open runtime residency)** does not affect DEL-12-04. Its
  local-first claims say that no cloud service is *required*, and that still
  holds.

## Canonical departures

None. CS-01, CS-02, CS-04, CS-06 (OK variant) and CS-07 are inherited as
assigned. The CP patterns used are CP-01, CP-02, CP-04, CP-05 and CP-09. CP-11
is applied in substance to R4, R7 and CLM-004.r06; it is named in their Notes
but not in `CanonicalSituation`, because those rows also carry landed slices.

## Convention friction

- A block with no issued `.rNN` keys but mixed parts (CLM-018, CLM-023, and
  the SOW surface for pointer drift) used `.sNN` sub-claims, as C1 allows.
  Blocks that were not split and have one gapped part (CLM-027) take the gap
  disposition for the whole block.
- CP-02 on the CONTEXT SURFACE row for the file-level revision 0.7 basis
  partly overlaps the keyed CS-01/CS-04 rows. It is applied the same way in
  DEL-12-05.
- A6: the June 7 run records give focused pass counts. The B4.4 sweep is cited
  as suite-level evidence only (`pytest -q tests`, 1,138 passed). Neither was
  rerun.

## Smallest checks for UNKNOWN rows

- `SOW#CLM-028/DEL-12-04-CF-001`: an owner ruling on the local secret provider
  and the encrypted-storage default, or an owner confirmation that both stay
  TBD. Before that, check the post-freeze `_DECISIONS/_REGISTER.md` for any
  ruling.
- `SOW#CLM-028/DEL-12-04-CF-002`: an owner ruling on the remaining deferrals
  (storage roots, grant persistence, API transport, external secret manager,
  approval choices), recording that package/container is already settled by
  SCA-003.

## Did the reverse pass change my view of anything sealed?

Yes, in one respect. The ledger is not edited.

- Routing rows RC-12-0177 and RC-12-0361 show that the product has a local
  private-library store. `save_local_library`, `open_local_library`,
  `list_local_libraries` and `delete_local_library` in
  `apps/desktop/src-tauri/src/lib.rs` persist accepted library imports per
  project, and `apps/desktop/src/services/libraryImportService.ts` exposes
  them. The store's gate refuses quarantined imports (Rust test
  `local_library_store_gate_admits_accepted_and_refuses_quarantined_imports`).
- My forward Notes on CLM-004.r01 ("No product flow registers a private
  library"), CLM-011.r01 ("No product feature stores private libraries or rule
  packs") and CLM-027 (example 1 "has no product flow") are therefore
  inaccurate as written.
- I would keep those rows `PARTIALLY_IMPLEMENTED`. The store does not use the
  DEL-12-04 reference records, opaque path references, credential references
  or transmission posture, and storage roots remain TBD. But the evidence
  wording should say "a library-import store exists; DEL-12-04's reference
  handling is not applied to it". The R6 rows (CLM-011.r06, CLM-004.r05) gain
  product-level quarantine evidence from the same gate. The verifier should
  weigh this.
- RC-12-0379 (the DEL-12-04 desktop panel) is `UNKEYED`. No issued key
  mentions a desktop surface.
- Rename residue in active code identifiers that the SOW does not cite:
  `openpipestress.technical_preview.secret_private_library_boundary_review`
  and the `openpipestress-preview-secret-private-library-*.json` download name
  in the panel. They are recorded in SURFACE-row Notes for R3/R4 clustering
  under CP-04.

## Batch consistency

`validate_ledger_v2.py --batch` over the DEL-12-04 and DEL-12-05 forward
ledgers returned **PASS, 0 consistency findings**.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Items for the verifier or owner

- Protected-check or baseline rows: none. No protected check was removed or
  weakened, and there are no ISSUED rows.
- INVARIANT rows: CLM-004.r04, CLM-004.r05, CLM-007.r05, CLM-011.r01,
  CLM-011.r06 and CLM-011.r10, covering the IP/data and quarantine subject.
- Owner items: the CP-04 surface row, CF-001 and CF-002 (`UNKNOWN`), the
  PDU-034 quarantine taxonomy and destructive-workflow policy, and the RF-001
  and RF-002 review dispositions.
- Possible defect: none. The code contradicts no governing claim. The gaps are
  missing integration and record inaccuracy.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
