VERDICT: FINDINGS

Backcheck 4 covers `c0c7a66f0..c6dfc3679` (HEAD `c6dfc3679e101249362536554ad4d6f8674fb491`). B3-2 and B3-3 are closed, and B3-1 is closed except for one residual path. The merge and routing outputs reproduce byte for byte. Routing has two actionable defects: it withholds capabilities from packages that plausibly own them, and it tells workers which rows are the cross-area sample. The working tree also has uncommitted R2 briefs (`R2-MANAGER_brief.md`, `R2-WORKER_brief.md`), which I did not review.

## (a) Backcheck 3 findings

**B3-1 — mostly closed.**
- Replaying my synthetic `INV_FEATB` case now fails on four counts:
  - wrong Area;
  - ownership token `del-17-04`, caught case-insensitively;
  - a directory EntryPoint in a non-family area;
  - 22 of 22 area files uncovered.
- All 12 committed inventories pass `--coverage`.
- **Residual (B4-1 below):** in the areas allowed to cite directories (DATA, DOCS, CHECKS), the tokens `projects/chirality-piping` and `projects` still pass. A single DOCS row with `EntryPoints=projects/chirality-piping` gives PASS with 0 findings.

**B3-2 — closed.**
- The four `src-tauri` Rust files are now labelled `cargo_crate_sweep`.
- `build-wasm-engine.test.mjs` is now indexed.
- The 759 `validation/evidence` files are now included as `evidence_record`.
- A rerun of `build_r1_indexes.py` reproduces all three outputs byte for byte: 102 deliverables, 272 test files, 999 assets.
- Cosmetic only: the `.mjs` file is labelled `typescript`.

**B3-3 — closed.** In the authority map:
- the dual context/evidence role is stated;
- the A3a example is now DEL-11-01 R18, and the 2026-09-18 record is marked context only;
- the PRD reference is now line 16;
- the fences are retained through the project `AGENTS.md`;
- `AGENTS.md`, `LOOP_INIT.md` and `software-workflow.json` are added as governing;
- the reliability rule now defers to C10.

## (b) R1 merge

- `IMPLEMENTATION_SURFACES.csv` (598 capabilities, `9a6644f8…`) reproduces byte for byte. The rule is: concatenate the 12 inventories' body rows in partition order (VIEW, WSUI, FEATB, FEATC, SHELL, SOLVER, PHYS, COREB, COREC, DATA, CHECKS, DOCS), then write a new `#END` with the count.
- All 12 inventory hashes match their `RETURN` events.
- There is no committed merge script (B4-4).

## (c) Routing

- **Reproducibility:** `route_capabilities.py` rerun into scratch gives all 18 `ROUTING/PKG-*.csv` files byte for byte.
- **Coverage:** every area is served by at least 2 packages. Every capability is asked by area routing of at least 2 packages, and at least 3 in total.
- **Sample:** it is deterministic. Per-package rates run from 7.9% to 12.6% of non-area capabilities, which is consistent with 10%.
- **The router suggests no answers,** but the files it writes leak the sample (B4-3).
- **The AFFINITY table is defensible from package names and scope in outline,** but it leaves several capabilities unasked by their most plausible package (B4-2).

## (d) New findings

**B4-2. ACTIONABLE — routing never asks some packages about capabilities they most plausibly own.**
- Where: `tools/route_capabilities.py:34-53` (the AFFINITY table).
- Evidence: each capability below is not in the named package's routing file at all.
  - **PKG-09**, missing FEATB and FEATC:
    - CAP-FEATC-006, the build-readiness panel. DEC-074 O3 rules "build-readiness panel → DEL-09-05";
    - CAP-FEATB-031 (validation-evidence);
    - CAP-FEATB-029 (project-validation).
  - **PKG-10**, missing COREC: CAP-COREC-051..053 (`core/adapters/`). PKG-10's scope names import/export adapters.
  - **PKG-05**, missing FEATC: CAP-FEATC-036 (wind exposure) and CAP-FEATC-031 (self-weight), both load authoring.
  - **PKG-04**, missing FEATC: CAP-FEATC-034 (support configuration) and CAP-FEATC-004/005 (boundary authoring). Supports are in PKG-04's scope.
  - **PKG-03**, missing FEATC: CAP-FEATC-020 (material temperature) and CAP-FEATC-016 (hanger selection).
- Other packages are asked about each of these, so none is unrouted. But the likely owner never sees it, and R3 would then misread the gap as "unclaimed" or "claimed elsewhere".
- Smallest fix:
  - Add FEATB and FEATC to PKG-09, COREC to PKG-10, and FEATC to PKG-03, PKG-04 and PKG-05.
  - Or route FEATC and FEATB by feature folder rather than whole area.
  - Regenerate the routing files and rebind their hashes.

**B4-3. ACTIONABLE — the worker-facing files reveal which rows are the sample.**
- Where: `tools/route_capabilities.py:80-82`.
- Evidence: every `ROUTING/*.csv` row carries a `Routing` column with the value `AREA` or `SAMPLE`. §6 uses the 10% sample to catch anchoring and careless `NOT_MINE` answers. Workers who can see which rows are sampled can default to `NOT_MINE` on them, which defeats the check.
- Smallest fix: drop the `Routing` column from worker files (or shuffle rows without it), and keep the AREA/SAMPLE mapping in a separate manifest that only the verifier reads.

**B4-1. MINOR (residual of B3-1) — the project root still counts as a family directory.**
- Where: `tools/check_inventory.py:99-106`.
- Evidence: `rel = p[len(P):] if p.startswith(P)` never strips the prefix when the token has no trailing slash. So `projects/chirality-piping` and `projects` escape the whole-area-prefix test and cover the entire project.
- Smallest fix: reject any directory token that is not strictly under `projects/chirality-piping/`, or compare with `P.rstrip("/")`.

**B4-4. MINOR — two bound inputs have no generator.**
- Evidence:
  - `IMPLEMENTATION_SURFACES.csv` has no committed merge script. I reproduced it from the rule above.
  - `ROUTING_PATH_HINTS.json` has no committed generator. It is hash-bound in `BOUND_INPUTS` (`5e802cc2…`), but its mention counts cannot be regenerated.
  - The sample docstring says "lowest tenth", while the code uses `% 10 == 0`. The effect is the same.
- Smallest fix: commit the merge and hint scripts, or record the exact method in `RUN_BASIS`, and align the docstring.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 8, WARN 112). The four findings on this run's files are all the absolute-path REVIEWs on the stored `returns/`, which are already disclosed.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc4/`.

END-OF-RETURN