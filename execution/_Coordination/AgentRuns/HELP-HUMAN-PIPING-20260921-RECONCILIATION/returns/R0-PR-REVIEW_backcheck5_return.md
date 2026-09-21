VERDICT: FINDINGS

Backcheck 5, delta `c6dfc3679..4736a2a6a` (HEAD `4736a2a6a93f35d42a42770a9881f84fac204278`): B4-1, B4-2 and B4-4 are closed, but B4-3 is not. The R2 plan and briefs add two actionable defects and three minor ones. None of this blocks the R0/R1 PR, but B4-3 and B5-2 should be fixed before wave 1 is dispatched.

## (a) Backcheck 4 findings

**B4-1 — closed.**
- The tokens `projects/chirality-piping`, `projects` and `projects/chirality-piping/` each fail as "not strictly under". A whole-prefix family such as `…/governance` also fails.
- All 12 committed inventories still pass `--coverage`.

**B4-2 — closed.**
- CAP-FEATC-006 (the build-readiness panel), FEATB-031, COREC-051..053, FEATC-036, FEATC-020 and FEATC-034 now reach PKG-09, PKG-10, PKG-05, PKG-03 and PKG-04 as intended.
- Every area is still served by at least 2 packages.

**B4-4 — closed.** Each tool reran into scratch and matched the committed files byte for byte:
- `merge_inventories.py` reproduces `IMPLEMENTATION_SURFACES.csv`;
- `build_path_hints.py` reproduces `ROUTING_PATH_HINTS.json`;
- `route_capabilities.py` reproduces all 18 `ROUTING/*.csv` files and `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.

**B4-3 — not closed.** The `Routing` column is gone and rows are hash-ordered, but a worker can still tell which rows are the sample:
- **By the `Area` column.** Routing is by whole area, so all 521 sampled rows carry an Area that never appears among that package's area rows. Each one is recognisable from the `Area` column alone.
- **By reading the router.** The worker brief does not forbid reading `RUN/tools/route_capabilities.py`, which contains the AFFINITY table, or `ROUTING_PATH_HINTS.json`.
- **By being told.** `R2-WORKER_brief.md:115-116` says the file "includes capabilities from outside your package's usual areas".
- Smallest fix:
  - Drop the `Area` column (and `Kind`, if you want) from the worker routing files.
  - Add `tools/route_capabilities.py` and `ROUTING_PATH_HINTS.json` to the worker's do-not-read list.
  - Reword line 115 to "judge every row on its evidence alone".

## (b) Wave plan and R2 briefs

**What holds:**
- **Cap accounting fits.** Wave 1 has 7 live agents: Agent 0, 2 managers and 4 workers. Verifiers are launched only after the managers return. Later waves can peak at 1 + 3 + 12 = 16.
- **Independence holds.**
  - Only Agent 0 launches verifiers, which are fresh and evidence-only.
  - The manager never judges claims or edits ledgers.
  - Workers cannot read other workers' folders or `ROUTING_SAMPLE/`.
- **Sealing before the reverse pass holds.** Workers may not read the routing file, `IMPLEMENTATION_SURFACES.csv` or `R1_INVENTORY/` until all their forward ledgers are sealed. Managers check each forward file's hash against both the seal and the worker's report.
- **The brief instructions produce ledgers that pass the validator.**
  - The sentinel count goes in `Notes`.
  - The validator is run before sealing.
  - Canonical rows are inherited.
  - Reverse answers cover every routed capability.
- **The §7 manager-brief items are carried,** either in the briefs or through `CONVENTIONS.md`.
- **Verifier sampling matches §7 at double rates,** plus the owner-confirmed 100% on shared-text rows. It adds 100% on `PROTECTED_CHECK` and `FROZEN_CONTRACT` rows, which is stricter than §7 asks.

**Findings:**

**B5-1. ACTIONABLE — the verifier-triggered rerun has no procedure.**
- Where: `R2-VERIFIER_brief.md:79-82`; `WAVE_PLAN.md:38-47`.
- Evidence:
  - The verifier can return `RERUN <DEL list>`, but nothing says who reruns those deliverables or whether a fresh verifier then re-verifies.
  - The manager's rerun rule (`R2-MANAGER_brief.md:73-82`) covers only validator, seal and file defects.
  - Gate condition 1 simply fails on a `RERUN` verdict.
- Smallest fix: state in `WAVE_PLAN.md` that Agent 0 relaunches the package manager with the named deliverables, and a fresh verifier then checks only those deliverables. Also say whether that rerun counts against the gate or goes to the owner.

**B5-2. ACTIONABLE — no brief authorises moving superseded files.**
- Where: `R2-MANAGER_brief.md:78-81`; `R2-WORKER_brief.md:31-38`.
- Evidence:
  - The manager's rerun launch message tells the fresh worker to move the old files into `superseded_<n>/`.
  - The worker's boundary allows writing only the four named files, `_scratch_*` and its notebook.
  - The manager may write only inside its own record.
  - So neither agent may make the move.
- Smallest fix: add `superseded_<n>/` (move only, never edit) to the worker's write boundary.

**B5-3. MINOR — calibration pilots in wave 1 can see R0 ownership answers before sealing.**
- Where: `R2-WORKER_brief.md:79-82`.
- Evidence:
  - DEL-07-02 and DEL-07-09 are R0 pilots, and both are in wave 1.
  - The brief says to read "its R0 ledger and notes" and the named repairs before starting. That includes `<DEL>_reverse.csv` and repairs such as re-answering the nine reverse answers.
  - These carry ownership signals before the forward seal, against B1's rule that the worker never sees the reverse inventory before sealing.
- Smallest fix: before sealing, read only `<DEL>_forward.csv`, the notes and the forward repairs; read the reverse file and reverse repairs after sealing.

**B5-4. MINOR — the 13-deliverable first wave departs from ruled text.**
- Where: `WAVE_PLAN.md:16-20`.
- Evidence:
  - The owner adopted "a 12-deliverable first wave" (R0 ruling item 1).
  - Package sizes confirm no package has exactly 3 deliverables, so PKG-07 (9) plus one other whole package cannot total 12.
  - The departure is disclosed, but under the `CONVENTIONS.md` precedence clause it still needs owner confirmation.
- Smallest fix: list it for the owner at the wave-1 checkpoint, alongside the canonical table.

**B5-5. MINOR — one bound hash is stale.**
- Where: `RUN_STATE.jsonl`.
- Evidence: `RESUME.md` changed in this commit, but the latest `BOUND_INPUTS` for it still gives the old hash (`4ea46468…`). The other 43 bound entries match, including the three R2 briefs (`AR:` paths).
- Smallest fix: rebind `RESUME.md`.

## (c) Other new defects

None beyond B5-1 to B5-5.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 9, WARN 112). The only findings on this run's files are the disclosed absolute-path REVIEWs on the stored `returns/`.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc5/`.

END-OF-RETURN