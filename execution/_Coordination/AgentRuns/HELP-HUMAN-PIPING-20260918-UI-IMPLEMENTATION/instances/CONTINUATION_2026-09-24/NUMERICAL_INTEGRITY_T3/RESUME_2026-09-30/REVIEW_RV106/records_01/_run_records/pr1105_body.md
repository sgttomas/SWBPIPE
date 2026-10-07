**piping(T3): records after #1104 (records only)**

This PR carries T3's execution records to main after the T6 slice's merge (#1104). It changes only `projects/chirality-piping/execution/`, and no product code, test or CI file.

## What it carries

From the integration branch `codex/piping-numerical-integrity-20260926` at `030020aca3`: 218 added files and 2 modified, with none deleted.

- **Merge records:** #1103 (`IMPLEMENTATION/RECORDS_MERGE_2026-10-06B/`) and #1104 (`IMPLEMENTATION/T6S_MERGE/`), with T6S's DEC-025 comparison and the package's draft run records.
- **Review records:**
  - RV101's two addenda for the T6 slice;
  - RV103's review of #1103;
  - RV105's review of B0, with its addendum.
- **Returns:**
  - I75's repair of the bound text on 16- and 17-digit ties;
  - I78's B0 contract design and its revision;
  - I79's point-path panic repair (T3-SI1b, under review).
- **Briefs:** the B1 probe, the cap/M study, I80's package, and the RV103–RV105 reviews.
- **U8's merge record:** the sealed counts extract (RV103 S-1).
- **Modified:**
  - `ROOT_RULINGS_V1.md` (append-only), including the owner's decision of 2026-10-06 on the memory ceiling;
  - the work graph's T3 section.

## Gates (records-only)

- GEN-8 on the exact head, the automatic CI, and an independent review. These are agent reviews, not personal review by the owner.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

