# N1 write-fence amendment — DAG-pointer consumer test

- RequestedBy: `HELP_HUMAN`
- AppliedBy: `PROJECT-SETUP-SCA009-INSTRUMENTS`
- Date: 2026-08-21
- Classification: `AMEND` N1 in place; mechanically coupled pointer-consumer repair only.
- Discovery: `projects/chirality-piping/tests/test_release_readiness_script.py` contains four expected strings pinned to DAG-009, while the production release-readiness implementation resolves the current graph through `execution/_DAG/_LATEST.md`.
- Added write target: `projects/chirality-piping/tests/test_release_readiness_script.py` only.
- Authorized delta: replace exactly four DAG-009 expected-path/text strings with DAG-010 after the accepted DAG-010 pointer move.
- Acceptance: focused release-readiness test file passes; no production source, behavior, lifecycle, or graph semantics change.
- Exclusions: every other test/source path remains outside this amendment.

HELP_HUMAN ruled this disposition `AMEND` in-session after the coupled test was surfaced.
