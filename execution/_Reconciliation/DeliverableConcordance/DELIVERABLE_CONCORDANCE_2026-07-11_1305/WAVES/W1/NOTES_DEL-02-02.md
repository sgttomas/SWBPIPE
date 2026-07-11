# Notes — CLAIM_CONCORDANCE_DEL-02-02 (W1, R2)

Deliverable: **DEL-02-02 — Unit system and dimensional-analysis core contract** (PKG-02).
Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (read-only worktree).
Requirement-ID mapping (non-self-identifying `U-*` → addendum-12 ClaimID form):
`U-001→DEL-02-02-REQ-001` … `U-016→DEL-02-02-REQ-016` (1:1, in order).

**Run-level `NormativeSource` path alias (addendum 12, declared once here):**
`KIT/` = `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/`.
All other paths in the ledger are repo-root-relative under `projects/chirality-piping/`.

## 1. Histograms (recount from CSV; must reproduce exactly)

**Disposition histogram (26 rows):**

| Disposition | Count |
|---|---|
| ALIGNED | 14 |
| PARTIALLY_IMPLEMENTED | 7 |
| STALE_SETUP_SPECIFICATION | 4 |
| VERIFIED_NOT_VALIDATED | 1 |
| **Total** | **26** |

**ClaimType histogram (26 rows):**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 16 |
| DECLARED_STATE | 7 |
| EXCLUSION | 3 |
| **Total** | **26** |

Per-ClaimType disposition breakdown:
- REQUIREMENT (16): ALIGNED 8 (U-003,004,005,007,009,011,013,014); PARTIALLY_IMPLEMENTED 7 (U-001,002,008,010,012,015,016); VERIFIED_NOT_VALIDATED 1 (U-006).
- EXCLUSION (3): ALIGNED 3 (EXC-001,002,003).
- DECLARED_STATE (7): STALE_SETUP_SPECIFICATION 4 (Specification, Datasheet, Guidance, Procedure); ALIGNED 3 (_STATUS, MEMORY, core/units/README.md).

## 2. Census decisions (row set)

- **16 REQUIREMENT rows** — one per current requirement ID U-001..U-016; substance disposition each (convention 1; requirement rows never take `STALE_SETUP_SPECIFICATION`).
- **3 EXCLUSION rows** — the three "Out of scope" bullets in `KIT/Specification.md §Scope`.
- **7 DECLARED_STATE rows** — addendum-1 census: 4 four-document-kit surfaces + `_STATUS.md` + `MEMORY.md` + one deliverable-owned in-tree README (`core/units/README.md`, the deliverable's anticipated "core/units module contract" artifact).
- **0 ACCEPTANCE rows** — the Specification "Verification" table and the setup-era `_REVIEW.md` AC-001..AC-009 merely restate/mirror the U-* requirements 1:1; no independent acceptance grain (addendum 12 / brief census).
- **0 REMAINING_WORK rows** — `_STATUS.md ## Remaining` carries ONLY the seeded `(gated: D-41)` bootstrap item, recorded verbatim in the `_STATUS` DECLARED_STATE row's `RecordedRemaining` and excluded from all residual/gate/selectability analysis (addendum 2). No non-bootstrap residual exists (matches `DELIVERABLE_INVENTORY.csv`: `NonBootstrapItems=NONE`, `SelectableUnderCurrentLoop=NO`).
- **0 IMPLEMENTED_UNMAPPED rows** — every material surface in this deliverable's orbit (`core/units` crate SURF-140, `schemas/units.schema.yaml` SURF-209, `fixtures/units` SURF-168, `unitCatalogService.ts` SURF-062) is already attributed to DEL-02-02 in `IMPLEMENTATION_SURFACES.csv`; none unmapped.

## 3. Key finding — DEC-018 resolved a large block of setup-era TBDs

The four-document kit (all `status: draft`, created 2026-04-30) declares the unit catalog, base-dimension vector, conversion constants, numeric representation, offset-temperature and gauge/absolute-pressure semantics, canonical basis, and tolerance policy as `TBD`. **DEC-018 (ruling of record D-01, human project authority 2026-06-10; record `execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md`; codified `SOFTWARE_DECOMP.md §12`)** later accepted all of those, and the `core/units` crate (SURF-140) implements them as the "Phase B1" slice. Under convention 1's two-signal split this produces:
- substance dispositions on the REQ rows (ALIGNED / PARTIALLY_IMPLEMENTED / VERIFIED_NOT_VALIDATED), and
- `STALE_SETUP_SPECIFICATION` (addendum 4, widened) on the four kit DECLARED_STATE surface rows whose prose no longer describes the frozen implemented slice.

DEC-018 explicitly did **not** rule (D-01 §1 "Not ruled here"): unit-identifier namespace/alias policy (→ U-016), dimensionless/ratio/percentage/coefficient classification (→ U-015), angle/rotation treatment, persisted quantity shape + hash canonicalization, schema file layout/tooling (→ U-009), and unit diagnostic-code namespace (→ U-010). Those remain open decisions and drive the `PARTIALLY_IMPLEMENTED` + `AuthorityNeeded=OWNER` encodings on U-009/U-010/U-015/U-016.

## 4. Calibration-flagged item (conversion catalog / addendum 5 / convention 3)

Per the dispatch calibration note (the R0b DEL-05-03 pilot homed a conversion-catalog residual candidate and verified DEL-02-02's `_STATUS` as bootstrap-only): I re-verified that DEL-02-02's `## Remaining` carries **only** the `(gated: D-41)` bootstrap item — no conversion-catalog residual is homed here. The two-tier conversion-witness + tolerance **validation** corpus is a documented Phase B3 handoff governed by **DEC-026** (D-04; class-tiered tolerance) and is **not present** at the frozen SHA and **not recorded** as a DEL-02-02 residual. I therefore did **not** open a `REMAINING_WORK` row for it; instead U-006 (deterministic conversion, MECHANICS/numeric) takes **`VERIFIED_NOT_VALIDATED`** — crate-local determinism is implemented and unit-tested, but engineering-grade conversion validation explicitly remains open (method §6: a unit test is not promoted to validation). Addendum 5's `ACCEPTED_DIVERGENCE` branch does **not** apply because DEC-018 *resolved* these items rather than *permitting a deferral* (addendum 11 threshold not met).

## 5. Evidence-execution log

Both side-effect-free suites were **re-executed at the frozen SHA** per addendum 9, with build artifacts redirected outside the frozen tree and porcelain checked before and after (all empty):

| Suite | Command | Result | Porcelain |
|---|---|---|---|
| `core/units` cargo tests (RUST-33) | `cargo test --manifest-path core/units/Cargo.toml --offline` with `CARGO_TARGET_DIR=<scratch>` | **13/13 pass**, 0 doc-tests | clean before/after |
| `tests/test_units_schema.py` (PY-74) | `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest … -p no:cacheprovider` | **3/3 pass** | clean before/after |

Named cargo tests observed (evidence the accepted semantics are exercised): `catalog_uses_si_canonical_units_with_dual_display_entries`, `multiplicative_conversions_use_reviewed_public_definitions`, `absolute_temperature_uses_affine_conversion`, `temperature_intervals_are_not_offset`, `quantity_kind_blocks_temperature_scale_interval_mixups`, `gauge_absolute_pressure_conversion_requires_explicit_reference`, `pressure_unit_conversion_does_not_change_gauge_absolute_kind`, `incompatible_dimensions_are_rejected`, `nonfinite_values_are_rejected`, `accepted_dimension_vocabulary_includes_force_per_length_and_rejects_retired_aliases`, `schema_dimension_enum_matches_crate_vocabulary`, `catalog_definitions_carry_schema_facing_conversion_metadata`, `conversion_can_be_bound_to_semantic_dimensions_with_shared_units`.

The R1 `VERIFICATION_INDEX.csv` cites both suites as `CONTENT_IDENTICAL` at the frozen SHA (ancestor commit `e648462f1d05`, diff empty over the covered paths); my re-execution reproduces the pass at the frozen tree directly. No validation/benchmark evidence for `core/units` conversion witnesses exists in `VALIDATION_AND_PROVENANCE_INDEX.csv` (consistent with the B3/DEC-026 handoff above).

## 6. Convention-friction notes

- **`core/units/README.md` as a DECLARED_STATE surface (addendum 1).** Addendum 1 says "one per deliverable-owned in-tree README." `core/units/README.md` is a product-tree (not kit-folder) file and the `core/units` crate is a *shared* surface (SURF-140 attributes it to 12 deliverables). I nonetheless treated the README as DEL-02-02's deliverable-owned in-tree README because it *is* the deliverable's anticipated "core/units module contract" artifact and reads as DEL-02-02's own current-state declaration ("DEL-02-02 defines the contract…"). Judgment call — see self-flag.
- **`_REVIEW.md` / `Review_Findings.csv` not minted as DECLARED_STATE rows.** These deliverable-owned review files (dated 2026-04-30, `SEMANTIC_READY`, decomposition rev 0.4, "no concrete core/units files found") are heavily overtaken by later implementation, but they are **not** in the addendum-1 declared-state census (kit + `_STATUS` + `MEMORY` + README only). I recorded their staleness here rather than as a row. See self-flag.
- **EXCLUSION with a fired contingency (EXC-003).** The exclusion's own "TBD unless later accepted by human review" clause was satisfied by DEC-018. The conditional sentence is not strictly false, so I encoded `ALIGNED` (convention 1: exclusions take substance disposition) but routed `AuthorityNeeded=REVIEW` and cross-referenced the Specification staleness row, because the framing can mislead. See self-flag.
- **`ValidationEvidence=NOT_APPLICABLE` on MECHANICS rows (convention 5 / D5).** Used with in-cell reasons for U-005 (deterministic dimensional-compatibility logic; no engineering-numeric allowable) and U-001/U-002 (contract/architectural claims). U-006, the one MECHANICS row asserting numeric conversion correctness, is the exception and carries `NONE_FOUND` → `VERIFIED_NOT_VALIDATED`.

## 7. Cross-surface observation (routed, not a DEL-02-02 disposition)

D-01 packet evidence E4/§7 records a `CONFLICT`: `core/model_operations/validation_preview/engine.py` carries its own `CANONICAL_DIMENSIONS` list (29 ids, omitting `force_per_length`) and mirrors — rather than calls — the `core/units` authority, while the crate's vocabulary (30 ids incl. `force_per_length`) is the accepted single source. This bears on U-002 (adapters/consumers "shall validate units instead of bypassing the domain contract") but the surface is owned by **DEL-16-02** (operation validation/diff preview), not DEL-02-02. Recorded in U-002 `RemainingWork` as a duplicate-authority risk; no DEL-02-02 disposition or edit made. Flagged for the reviewer to route to the owning deliverable.

## 8. Self-flagged rows

- **DEL-02-02-REQ-006 (U-006)** — `VERIFIED_NOT_VALIDATED` on the conversion-catalog claim is the calibration-sensitive judgment (addendum 5 declared-TBD branch considered and rejected; convention 3 homing confirmed `_STATUS` bootstrap-only). Reviewer eyes requested on whether crate-local determinism tests + the DEC-018 T1 "software-testing policy (not an engineering allowable)" posture should instead support `ALIGNED` with an explicit "validation open" statement.
- **DEL-02-02-REQ-002 (U-002)** — `PARTIALLY_IMPLEMENTED` rests partly on the cross-deliverable duplicate-dimension-authority observation (§7) on a DEL-16-02 surface; the DEL-02-02 slice alone (crate + contract ownership) could read `ALIGNED`.
- **DEL-02-02-REQ-015 (U-015) and REQ-016 (U-016)** — `PARTIALLY_IMPLEMENTED` turns on reading DEC-018 §1 "Not ruled here" as leaving the dimensionless-classification and namespace/alias decisions genuinely open despite the crate's structural `QuantityKind`/catalog machinery; judgment on whether structural presence is enough for `ALIGNED`.
- **DEL-02-02-REQ-001 / 008 / 010 / 012** — the `PARTIALLY_IMPLEMENTED` split between "core contract implemented here" and "system-wide wiring is B2/B3 or another deliverable" is a judgment about how much of a system-wide requirement a *core-contract* deliverable is answerable for; reviewer may prefer these split differently.
- **DEL-02-02-DECL-006 (MEMORY.md)** — judgment call to mint a MEMORY DECLARED_STATE row at all (the file is entirely dated log entries with no standalone current-state header) and to disposition it `ALIGNED` per addendum 1's "drift-inside-dated-entries is a note, never a staleness disposition."
- **DEL-02-02-DECL-007 (core/units/README.md)** — judgment call to treat a product-tree README of a *shared* crate as this deliverable's "deliverable-owned in-tree README" (§6 friction note).
- **DEL-02-02-EXC-003** — `ALIGNED` on an exclusion whose "unless later accepted" contingency fired via DEC-018 (§6 friction note); `AuthorityNeeded=REVIEW` routed.

## 9. Boundary-compliance statement

- All discovery reads were from the frozen worktree `.claude-worktrees/piping-frozen-551f84ef6` @ `551f84ef6`; writes confined to the two output files under `WAVES/W1/` (`CLAIM_CONCORDANCE_DEL-02-02.csv`, `NOTES_DEL-02-02.md`).
- Frozen tree porcelain **empty** before and after both re-executions (build artifacts redirected outside the tree; no writes to git-ignored paths in the frozen tree).
- No lifecycle transition, no DAG mutation, no cross-project edit, no edit to any `_STATUS.md`/register/product file. `LIFECYCLE_REASSESSMENT_REQUIRED` not applied (none warranted). No `DEFERRED_AGENT_WORKFLOW` implications arose.
- No release-readiness, issuance, certification, sealing, professional-approval, or code-compliance claim made anywhere (F-PIP-1..5). All dispositions are agent judgments, routed via `AuthorityNeeded`, never phrased as owner/engineering rulings.
- No STOP-worthy contradiction encountered.
