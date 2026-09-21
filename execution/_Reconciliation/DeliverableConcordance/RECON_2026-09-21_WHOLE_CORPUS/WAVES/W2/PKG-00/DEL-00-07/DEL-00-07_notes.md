# DEL-00-07 notes — API boundary and adapter contract map

Wave W2, package PKG-00, worker G2. Forward ledger 40 rows (31 required keys,
5 `.rNN` rows for the handoff table, 4 `.sNN` sub-claims). Reverse file 387
answers. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.

## Path aliases

- `AB` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-07_API boundary and adapter contract map/ArchitectureBasis.md` (cited only in `NormativeSource`/`ContextRefs`).
- Handoff block key `AB#req-07-05-handoff-obligations-d-41-r5-t2b-2026-0`; its `.r01`-`.r05` are the five-boundary table rows (Storage, Reports, Private libraries, Local FEA, External automation).
- `api/api_boundary_contract.yaml` TBD slots L15-L31; runner binary name at `core/runner/headless/Cargo.toml` L12; product caller of library-import validation at `apps/desktop/src-tauri/src/lib.rs` L3376.

## Judgment calls

- **Rename residue (SURFACE, CP-04).** The basis names `.opsproj` (DEC-057 rider) and the `openpipestress-runner` binary (DEC-065). The SURFACE row takes the `.opsproj` fields (PROJECT_BASELINE, NONE, RECORD, OWNER) with the DEC-101 persistence-compatibility obligation; the runner name alone would take default CP-04 fields. Both are R4 code-change candidates. Because the SURFACE row carries this, the header's rev 0.9 pin is on `AB.s01` (CP-02).
- **REQ-07-03 (CP-11).** Adapters enforce no-bypass controls and cannot dispatch without a selected runtime, but there is no plugin runtime (TBD in the contract), so the plugin half holds only by construction: PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · BASELINE · OWNER (the runtime slots await a human architecture ruling).
- **External format list hold (open-holds.s02).** SCA-004 admitted CAEPIPE MBF, stress-neutral CSV/JSON, conservative PCF and GLB/glTF review geometry, and PKG-17 packages with schemas and tests exist, yet the hold says the list is genuinely open and routed to a human ruling. The API contract and `plugin_boundary.md` still hold the slots TBD, treating SCA-004 formats as boundary concepts. STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE · OWNER, MEDIUM: whether SCA-004 resolves the list for these boundary documents is an owner reading.
- REQ-07-04 ALIGNED: format choices are kept TBD in the boundary contracts and the export writers operate under SCA-004.
- REQ-07-02 ALIGNED MEDIUM: the Python adapter framework has no product caller (`PRODUCT_CALLER: NONE` noted), while the library-import path is product-called through the Tauri shell.
- Handoff block and `.r01`-`.r05`: judged at architecture grain for contrary evidence in sampled owning surfaces (MEDIUM).
- Resolved-decisions block: accurate record of DEC-028, DEC-057, DEC-064 and DEC-065 with a correct §12 pointer; the former-name identifiers are carried on the SURFACE row.
- realized-artifacts: not split (both rows ALIGNED).

## Canonical departures

None.

## Convention friction

- CP-04 has two identifier classes on one surface with different fields; the stronger tier was used for the single SURFACE row (F8 analogue). If the verifier prefers the runner as a separate sub-claim, that is a representation choice.
- The required-invariants and currency bodies are shared with DEL-00-08; batch check confirms matching treatment.

## UNKNOWN rows

None.

## Reverse pass

23 CONSTRAINS, 2 COVERS, 362 NOT_MINE. Export format writers (RC-00-0127, 0171, 0284, 0338) are NOT_MINE with capability-specific reasons: open-holds.s02 cites their schemas only as evidence that SCA-004 formats exist. The reverse pass did not change my view of any sealed row.

## Batch consistency

Batch over the four G2 ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
