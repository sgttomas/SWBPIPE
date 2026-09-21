VERDICT: PASS

Backcheck 6 covers `4736a2a6a..18c32ad4b`, the current HEAD. B4-3 and B5-1 to B5-5 are all closed, and there is no new blocking or actionable finding. One minor residual in B4-3 is worth disclosing, and the working tree has an uncommitted `RUN_STATE.jsonl` change (2 added lines) that I did not review.

## (a) Closure

**B4-3 — closed, with one inherent residual.**
- **File format.** Worker routing files now have only `CapabilityID`, `Capability`, `EntryPoints`, `Tests` and `Notes`. `CapabilityID` holds a routing-local `RC-<nn>-<NNNN>` ID. No worker-facing cell anywhere contains a `CAP-…` inventory ID.
- **Worker boundary.** The router, the path hints, the manifest, `IMPLEMENTATION_SURFACES.csv` and `R1_INVENTORY/` are all on the "never read" list. The line "outside your package's usual areas" is removed.
- **Reproducibility.** `route_capabilities.py` rerun into scratch reproduces all 18 `ROUTING/*.csv` files and `SAMPLE_MANIFEST.csv` byte for byte. The manifest maps each routing-local ID to package, inventory ID, area and AREA/SAMPLE, so R3 can resolve every answer.
- **Validator.** The `--inventory` path works with the new IDs. A full PKG-00 reverse file using `RC-` IDs passes. A file that swaps one row for `CAP-VIEW-001` fails on two counts: an unknown CapabilityID, and one routed capability left unanswered.
- **Residual (MINOR):** `EntryPoints` still gives the sample away for about half the rows.
  - 276 of the 521 sampled rows cite a module path prefix that no area row in the same package uses, so a worker can spot them from the paths alone.
  - This is inherent: workers need the paths to answer, so it cannot be removed without defeating the reverse pass.
  - Smallest fix: say in `RUN_BASIS.md` that about half the sampled rows are recognisable from their paths. Have the verifier weigh its "sampled against area" answer comparison with that in mind.

**B5-1 — closed.**
- `WAVE_PLAN.md` now sets out the verifier-rerun cycle: a fresh manager handles only the named deliverables, then a fresh verifier checks only those, at the same sampling.
- The gate is judged on the re-verified result. After one cycle, a deliverable still named for rerun goes to the owner without blocking the rest of its package.

**B5-2 — closed.** The worker brief's rerun clause lets the worker move the old files, unchanged, into `superseded_<n>/`. The manager's launch message points to that clause.

**B5-3 — closed.** Before sealing, pilots read only their R0 forward ledger, their notes and the forward repairs. They read the R0 reverse file and reverse repairs only after all their forward ledgers are sealed (B1).

**B5-4 — closed.** The 13-deliverable first wave is listed as owner item 1 at the wave 1 checkpoint, for confirmation under the precedence clause.

**B5-5 — closed.** All 44 bound hashes match their files, including `RESUME.md` and the three R2 briefs. That holds both for the committed `RUN_STATE.jsonl` at `18c32ad4b` and for the working copy.

## (b) New defects

None beyond the B4-3 residual above.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 10, WARN 112). The only findings on this run's files are the disclosed absolute-path REVIEWs on the stored `returns/`.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc6/`.

END-OF-RETURN