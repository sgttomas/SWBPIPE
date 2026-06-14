---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C3-IMPORTVALIDATE-001
timestamp: 2026-06-13T18:30:00-0600
completed: 2026-06-13T19:25:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/core/library_import/library_import_document
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-IMPORTVALIDATE-001 — library-import provenance crate (Phase C3 foundation slice)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C3** (private library management GUI)
  foundation slice — the runtime **library-import validation crate**, the first
  bounded increment of C3 after C2 landed, selected per the `_COORDINATION.md`
  Application Integration And Issuance Loop step 3.1 (earliest unblocked item on
  the R3/Phase C dependency spine: C2 done → C3 → C4). C3 is large (≈6 sub-slices);
  this slice builds the foundation every downstream C3 surface needs, mirroring
  how `open_pipe_stress_rule_pack_document` underpins C2 and how the B1 units
  crate (`TP-UNITS-B1-CATALOG-001`) landed crate-first.
- Authority: **DEL-03-07** (Public-private library import provenance checker;
  CHECKING). The authored design contract is the Python module
  `core/library_import/provenance_checker.py` and its conformance test
  `tests/test_library_import_provenance.py`. That module cannot run in the
  Tauri/Rust desktop runtime, so absorbing DEL-03-07's mature design into the
  application means porting it to a runtime Rust crate. PRD §13 (material/section/
  component library requirements), §13.5 (import warnings: missing fields,
  missing/unclear provenance and redistribution, suspected protected content,
  unit metadata), FR-022 (private libraries). IP boundary (no protected tables/
  constants; private data never redistributed), CONTRACT no-silent-defaults,
  OPS-K-PROV-1.
- Regression gate (loop step 3.2): the desktop Vitest baseline was checked first.
  A direct `npm test` appeared to fail (34 tests), but the cause was traced in
  full — the desktop Vitest requires `npm run build:wasm:desktop` first (DEC-020
  wasm operation engine), and `App.test.tsx` integration tests time out under
  concurrent load. With the wasm engine built and the suite run unloaded it is
  green (51/52; the lone holdout is a save/open round-trip with a hardcoded
  10 000 ms budget that passes in isolation), matching every recent five-surface
  sweep recording `desktop_vitest => pass` at the C2 HEAD. **No regression**; the
  gate is satisfied and this is new in-stage C3 scope, not regression repair.

## Changes (new standalone crate; no app wiring in this slice)

1. **`core/library_import/library_import_document/` (new crate
   `open_pipe_stress_library_import_document`)** — runtime Rust port of the
   DEL-03-07 provenance contract. Public API:
   - `validate_library_import(payload, library_kind, intended_visibility)
     -> ImportValidationResult` and the seam helper
     `validate_library_import_tokens(payload, "material|section|component",
     "public|private")` which returns `Err` for unsupported tokens (the seam
     rejects, never guesses — mirrors the Python `ValueError`).
   - `LibraryKind` / `IntendedVisibility` enums with `from_token` / `token`
     round-trips; `ImportFinding { code, severity, path, message, remediation }`
     with a `to_diagnostic()` PKG-02 import-boundary envelope projection;
     `ImportValidationResult { outcome, library_kind, intended_visibility,
     accepted, findings }` with `diagnostics()`.
   - Semantics ported one-for-one from `provenance_checker.py`: the 7 required
     provenance fields; the material/section/component `(library, records)`
     dispatch; missing library-metadata / record-set / non-object-record blocks;
     missing / incomplete provenance; protected-suspected **quarantine**;
     rejected-source block; the public-disposition ladder (private-data-public
     block, missing redistribution rights, unaccepted redistribution, review
     required); nested unit-bearing-value unit-metadata and value-provenance
     checks; and the severity-precedence outcome
     (`QUARANTINE > REJECTED > REVIEW_REQUIRED > PRIVATE_LOCAL_ONLY /
     ACCEPTED_PUBLIC`). Python truthiness and the `item.get(x) or
     provenance.get(x)` status resolution are reproduced explicitly
     (`is_truthy` / `truthy_str`). The projected diagnostic `source` honestly
     names the runtime crate (`core.library_import.library_import_document`);
     semantic parity is on codes/severities/paths/outcomes, not the identity
     string.
2. **`tests/provenance_parity.rs` (new)** — cross-language parity: mirrors all
   seven `test_library_import_provenance.py` cases over the **same** invented
   fixtures (`fixtures/material/invented_material_library_valid.json`,
   `fixtures/component/invented_section_component_library_valid.json`), including
   the Python helper `accepted_public_component_payload()`. The two
   implementations must move together; this test is the guard against silent
   divergence.
3. **`src/lib.rs` `#[cfg(test)]` unit tests (11)** — branch coverage beyond the
   shared fixtures: token-parser rejection; missing library metadata + record
   set; non-object record; sorted incomplete-provenance field list; nested
   value without units/provenance; private-only public block; unaccepted public
   redistribution; **rejected-source disposition**; **private `privacy_class`
   public block even with permissive redistribution**; **item-level status
   overriding provenance status**; and the PKG-02 diagnostic-envelope shape.
4. **`Cargo.toml`, `README.md` (new)** — crate manifest (deps: serde,
   serde_json only) and a README documenting the API, the runtime-port relation
   to DEL-03-07, and the parity guard.

No `apps/desktop` file changed; no Tauri command, frontend service, persistence,
or UI exists yet (those are the next C3 slices). No other crate depends on this
one yet, so no other Rust/Python/TS surface is affected.

## Validation

- `cargo test --manifest-path core/library_import/library_import_document/Cargo.toml`
  — **18/18** (11 lib unit + 7 parity; 0 doctests). Clean build, no warnings.
- `cargo clippy ... --all-targets -- -D warnings` — clean (two lint nits fixed:
  `is_none_or`, `!is_empty()`).
- Python parity oracle `pytest tests/test_library_import_provenance.py` — **7/7**
  still green (confirms the parity basis the Rust port reproduces is unchanged).
- The new crate's `Cargo.toml` is auto-discovered by
  `tools/release/check_release_readiness.py --profile cargo` (the sweep's
  `cargo_crate_sweep` surface), so the DEC-025 merge-gate sweep now exercises it.
- DEC-025 five-surface evidence sweep run at the committed HEAD; summary
  committed alongside this tranche. Pre-existing external-scope dirty paths
  (`init/` app-dev coordination files, root `INIT.md`) are not products of this
  tranche — recorded and bypassed per the entry prompt's external-scope rule.
- Evidence posture (H4): a crate-only foundation slice with no user-visible
  desktop behavior change, so no Playwright/Vitest UI evidence is owed; the
  appropriate evidence is the crate's cargo tests plus the cross-language parity
  corpus, both green. This matches the B1 units-crate precedent.

## Pre-commit adversarial review and dispositions

Self-review across four lenses (port fidelity vs. the Python source line-by-line;
parity-test non-vacuousness; clippy/borrow correctness; boundary/IP). Findings
and dispositions:

- **Fidelity — control flow.** The Python `if/elif/elif` protected→rejected→
  public ladder, the missing-provenance early return, and the additive
  incomplete-fields finding are all reproduced; verified each fixture case and
  three extra branch tests (rejected source, privacy-class public block,
  item-over-provenance precedence) that the shared fixtures do not exercise.
- **Fidelity — truthiness.** Python `or`/`not` truthiness on JSON values is
  reproduced explicitly; the `item.get(x) or provenance.get(x)` fallback is
  pinned by `item_level_status_overrides_provenance_status` and by the
  protected-suspected case (which relies on the provenance-level fallback because
  the fixture's `material_library` carries no top-level `redistribution_status`).
- **Parity non-vacuousness.** The parity test asserts both `outcome` and finding
  **code sets**, not just acceptance; the all-green-empty public-component case
  would redden if any spurious finding were emitted.
- **Boundary.** The crate parses no external file formats, makes no legal/license
  determination, emits only software findings (no professional/certification/
  code-compliance claim), and contains no protected tables/constants or private
  data — it validates already-parsed payloads exactly as the Python contract
  does. No lifecycle state changed; DEL-03-07 remains CHECKING.

## Residuals and hand-offs (next Phase C / C3 work)

- **C3-next (seam):** Tauri `validate_library_import` command wrapping this crate
  + a typed `libraryImportService.ts` with the browser-unavailable seam (mirrors
  the units `get_unit_catalog` seam and the rule-pack service), with a Rust
  command test + Vitest service test.
- **C3 persistence:** local-only private-library store (CRUD) for imported
  material/section/component libraries, private-by-default, never committed.
- **C3 UI:** import-wizard panel (upload → kind → preview → §13.5 warnings →
  confirm) + a workspace section + list panel, replicating the C2 RulePack
  manager pattern; §13.5 blocking-vs-advisory warning display; rule-pack ↔
  library reference wiring; boundary notice.
- **C4:** end-to-end rule checks on authored models consume imported libraries.
- Parity discipline: if `provenance_checker.py` or the shared fixtures change,
  `tests/provenance_parity.rs` must move with them.
