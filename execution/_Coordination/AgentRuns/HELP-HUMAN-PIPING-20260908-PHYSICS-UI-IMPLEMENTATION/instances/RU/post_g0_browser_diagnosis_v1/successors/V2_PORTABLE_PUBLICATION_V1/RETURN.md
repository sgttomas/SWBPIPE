# RU portable publication return

Status: PORTABLE_PUBLICATION_PACKAGED

The frozen V2 candidate meaning and hashes are unchanged:

- raw applicable patch SHA-256: e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc
- candidate postimage SHA-256: be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248
- V2 output-manifest SHA-256: e4cd61ccdd07d9b33f94788839a4074c2bbed3cd36c3a895c33437db95865fa6

The raw patch and candidate postimage are losslessly base64-archived. RESOLVER.json binds their frozen physical paths, encoded archives, decoded hashes, and publication rule. The successor CANDIDATE_TEST_CORRECTION_V2.diff is expressly a non-applicable pointer; consumers must decode the archive and verify the raw patch hash before application.

The path validator identified one machine-local path in the frozen V1 SEALED_BRIEF.md. Its exact original bytes and logical hash 28bb58011ae2db326e15dbc7b190012bb3459ed23867c1550fee8bc6d9da63ca are now archived; that physical evidence path is a portable pointer. The frozen V1 and V2 manifest files remain byte-unchanged, and the resolver states how their logical references resolve.

Validation:

- archive decode: PASS, 3/3 objects, exact byte lengths and hashes;
- raw patch path/applicability dry run: PASS, resolved only projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts;
- registered candidate-whitespace validator: the frozen raw diff reports its two required unified-diff context spaces; candidate postimage plus non-applicable publication pointer PASS;
- registered path-anchor validator final rerun: PASS, 4,275 checked files, zero findings.

The two initial validator failures are preserved losslessly under _run_records and classified in FAILED_COMMANDS.json. No source/test execution, browser run, build, CUA, or product edit occurred.
