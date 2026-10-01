# Execution record

TASK RV29 reported directly to ROOT, with no delegation. Used read-only
GIT_OPTIONAL_LOCKS=0 git show/diff/ls-tree and existing VENV Python -B for
source, hash, manifest, JSON and raw-field comparisons. No Rust/probe/comparator
rerun, new source fixture, maintained edit, Git/index mutation or host change.
BASIS.json preserves actual immutable origins/hashes; VERIFICATION.json preserves
check results, feature/source/binary binding and read stdout summaries.
REVIEWED.diff is the full four-file delta from the prior RV29 f118 review.

Exploration corrections: the old response handoff was initially sought under
the wrong response-local T3 alias; the immutable tree located the actual T3/AUDIT
path, which was read before old probe source. accessor_01 uses HASHES.json, not
the initially tried IDENTITIES.json. The first evidence-check script incorrectly
looked for compared_rows in comparator JSON; the actual schema stores rows.
It stopped before producing a completed result. The corrected read-only check
counts that array, verifies all hashes and completes with19 selected/1 nonselected
and708 rows. No input/evidence/oracle was edited or runtime job repeated.

All intentional writes are this additive backcheck_04 packet. Earlier seals,
original source/test bytes and original runtime outputs remain untouched. The
probe library archive and target binaries were read only for verification; no
target-wide inventory or duplicate source tree was created.
