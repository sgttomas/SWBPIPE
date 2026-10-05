# Reviewer-only sparse fixture

The fixture contains selected copied entrypoints and the frozen test file plus
the separately retained reviewer_controls.rs addition. Other branches are
symlinks to SOURCE. This is a sparse explicit overlay, not a full checkout copy.
It was created and executed within minutes of receipt, within the ten-minute
setup allowance. It adds one bounded test; it changes no production arithmetic.

Before replay, require SOURCE at the clean candidate recorded in EXECUTION.json
and verify SOURCE_BEFORE.json. Symlinks are references, not immutable snapshots;
do not silently rely on later target bytes. The candidate Git revision and
FIXTURE.json record their source basis and the copied-file hashes. The reviewer
Cargo command and isolated target are in reviewer_fixture.command.json.

The actual retained-factor center is tested first. The second center is an
intentional finite reviewer choice, not a naturally reached factor output.
Source containment and residual/radius facts are independently checked by
independent_exact.py. The logged 52/52 versus 7/52 predicate counts in this
additional fixture come from the unchanged production-side diagnostic helper;
the independent checker does not claim full predicate rechecking on these two
added logs because their gate operands were not exported.
