**piping(T3): records after #1102 (records only)**

This PR carries T3's execution records to main after U8's merge (#1102). It changes only `projects/chirality-piping/execution/`, and no product code, test or CI file.

## What it carries

From the integration branch `codex/piping-numerical-integrity-20260926` at `ea0e288e8a`: 104 added files and 2 modified, with none deleted.

- **U8's merge record** (`IMPLEMENTATION/U8_MERGE/`): the gates and the DEC-025 comparison. The first comparison hit a host build-directory artefact in `operation_applier`; a rerun in a fresh build directory is clean (38 of 40 identical, plus added tests only).
- **#1101's merge record** (`IMPLEMENTATION/RECORDS_MERGE_2026-10-06/`).
- **Review records:** RV97's addenda for #1102, RV101's review of the T6 slice, and RV102's review of #1101.
- **The brief for T3-SI1b** (the point-path panic repair), and a host-tool copy.
- **Modified:** `ROOT_RULINGS_V1.md` (append-only), and the work graph's T3 section.

## Gates (records-only)

- GEN-8 on the exact head, the automatic CI, and an independent review. These are agent reviews, not personal review by the owner.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

