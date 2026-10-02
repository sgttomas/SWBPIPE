# Additive execution receipt

The same TASK reviewer received the backcheck through collaboration.followup_task.
Write scope: <A1_WT>/<R>/design_review_RV28/backcheck_addendum_01/** only.
No scratch, source, index, guard, configuration or prior sealed evidence changed.

Run command, exit 0:

    <VENV>/bin/python -B <A1_WT>/<R>/design_review_RV28/backcheck_addendum_01/backcheck.py \
      <A1_WT> <A1_WT>/<R>/design_review_RV28/backcheck_addendum_01 \
      > <A1_WT>/<R>/design_review_RV28/backcheck_addendum_01/CHECK.stdout.json \
      2> <A1_WT>/<R>/design_review_RV28/backcheck_addendum_01/CHECK.stderr.txt

Fresh exact checker: 72 PASS; no failed numerical run or post-failure test change.
Source reads used cat/nl/sed/rg and read-only git show with GIT_OPTIONAL_LOCKS=0.
The source-location typo and correction are disclosed in BACKCHECK.md.
Actual Git argv/status/output hashes are in COMMANDS.json; exact pinned source
bytes can be recovered from those object/path records. Python version, platform
and executable hash are in BASIS.json. Numeric results are raw canonical JSON.

All three earlier seals and every listed file were verified unchanged. This
packet's SHA256SUMS inventories its eight files except the seal itself. No
implementation, final allocator size, solver outcome or performance was tested.

For a later rerun, provide an owned scratch OUT directory to backcheck.py;
do not overwrite this sealed backcheck.
