---
doc_id: R16-SOFTWARE-DECOMP-REMEDIATION-PLAN-001
package_role: run-local amendment / derived planning package
status: accepted_final_run_local_basis
managed_run: HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16
canonical_decomposition_effect: none
accepted: 2026-07-23
---

# R16 SOFTWARE_DECOMP remediation plan

This accepted package is the coherent run-local execution basis derived
through confirmed SOFTWARE_DECOMP Gates 1–6 and owner Gate-7 acceptance. It is
not a new authoritative decomposition snapshot and does not amend or publish
the repository's canonical SOFTWARE_DECOMP package.

## Purpose and accepted option

Repair the four fresh N5 blockers by:

1. binding a canonical DEL-08-02 current-session input manifest through
   DEL-14-02 rather than relabeling result evidence;
2. preserving source-declared dimensions and adding DEL-08-04 support for
   `linear_stiffness` and `rotational_stiffness`;
3. projecting copied current-session records as private/pending/user-local;
4. enforcing exact lowercase 64-hex SHA-256 at both package wire boundaries.

Option A remains binding: no new package member and no portable-replay claim.

## Confirmed SSOW

| ID | Owner | Scope |
|---|---|---|
| R16-SSOW-001 | DEL-08-02 | Deterministic current-session input-manifest object over exact model/input basis, units, solver/settings, load bases, active rule packs, and external assets. |
| R16-SSOW-002 | DEL-08-02 | Stable manifest ref and exact SHA-256 over declared canonical bytes; transformation without recomputation blocks. |
| R16-SSOW-003 | DEL-14-02 | Bind manifest ref/hash through immutable analysis-run reproducibility; never substitute a result envelope. |
| R16-SSOW-004 | DEL-14-02 | Carry explicit canonical dimensions on source run/result records; consumers do not infer from unit text. |
| R16-SSOW-005 | DEL-08-04 | Add the two accepted stiffness dimensions to schema and Rust exporter. |
| R16-SSOW-006 | DEL-08-01 | Consume manifest and source dimensions; missing/mismatched evidence blocks before picker/filesystem effect. |
| R16-SSOW-007 | DEL-08-01 | Project copied current-session records as private/pending/user-local without source mutation. |
| R16-SSOW-008 | DEL-08-01 | Require bare lowercase 64-hex values wherever the package wire declares SHA-256. |
| R16-SSOW-009 | DEL-08-01 | Full gates, native proof, one replacement sweep, fresh N5, then W3 only on COMMIT-SAFE. |

## Objectives and ownership

- `OBJ-R16-001`: actual input-manifest identity is recomputable, distinct from
  result evidence, and malformed/missing evidence blocks.
- `OBJ-R16-002`: linear and rotational stiffness preserve source-declared
  dimensions through schema, Rust, desktop bridge, and package.
- `OBJ-R16-003`: current-session copies remain private/pending and exact hash
  validation blocks malformed values without source mutation.
- `OBJ-R16-004`: focused/full/native gates and replacement sweep pass, followed
  by fresh N5 before W3.

PKG-08 owns DEL-08-02, DEL-08-04, and DEL-08-01 work. PKG-14 owns DEL-14-02.
PKG-02 remains read-only semantic authority. Nine SSOW items have nine exact-one
assignments; shared files remain serialized under N4.

## Executable graph

```text
SLICE-01 → SLICE-02 → SLICE-03 → SLICE-04
         → SLICE-05 → replacement sweep → fresh N5
         → COMMIT-SAFE → DEL-08-01-only W3
```

N4 is sole serialized implementation owner. The exact file union is
`WRITE_MATRIX.csv`; any excluded-path need stops for amendment.

## Accepted residuals

- Current-session ref/hash integrity is proven; portable replay is not.
- No input-manifest package member is added.
- Predecessor deliverable status/memory remains untouched.
- The prior sweep is immutable superseded evidence.
- Dimensions are declared at DEL-14-02; solver semantics remain unchanged.
- Shared files do not create shared ownership.

After all slice-05 focused/full/native gates pass, run exactly one replacement
DEC-025 sweep, freeze the tree, and dispatch fresh N5. W3 remains gated on N5
`COMMIT-SAFE`; CHANGE retains Git closeout ownership.
