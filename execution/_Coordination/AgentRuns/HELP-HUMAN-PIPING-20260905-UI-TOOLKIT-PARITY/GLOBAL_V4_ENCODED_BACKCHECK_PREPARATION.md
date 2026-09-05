# V4 encoded global backcheck preparation — no freeze

Status: PREPARATION_ONLY_WAIT_N1_TERMINAL_SOURCE_REVIEW_TESTS
RunID: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY
Do not materialize V4 source membership/hashes/diffs until N1 tests and fresh review complete, exact manager acceptance/source freeze and parent relay. Recorder source writes held; N1's existing3file release remains separate. Prior V1-V3 and .gitattributes immutable.

Expected authored repair scope is App.tsx, styles.css and e2e/gui-workflow-validation.spec.ts only. V3 contains App/styles; GUI workflow test is absent from V3 membership because it previously was unchanged from original base. At finalization derive actual complete base-to-current tracked changes plus untracked nonignored product membership, not a forced anticipated count. Partition V3 unchanged, changed, added-to-review existing tracked/new files, and removed/reverted; include every source/docs/contract member under accepted classification. Preserve exact predecessor source commit and all existing accepted config changes as separately classified supporting configuration where parent specifies final review scope. Do not omit newly changed tracked GUI test.

## Mandatory encoding contract

Every NEW raw patch/diff/log is persisted only as lossless encoded JSON/base64. No new raw.patch/raw.diff/rawlog and no .gitattributes wildcard/exemption. Existing historical raw V1-V3 artifacts remain untouched.

Encoded container schema: artifact_kind, encoding=base64, decoded_media_type, decoded_bytes, decoded_sha256, content_base64, logical_name, base/predecessor references where relevant. Serialize portable paths/anchors in metadata. Compute raw SHA from exact original bytes before encoding; roundtrip strict base64 decode and verify byte count+SHA before accepting container. Container SHA is separately computed over persisted JSON bytes and referenced from manifest/dispatch. Never substitute containerSHA for decoded rawSHA. Avoid a self-referential container hash field inside its own bytes.

Prepare future GLOBAL_PRODUCT_DIFF_V4_ENCODED.json and GLOBAL_REPAIR_DELTA_V4_ENCODED.json, not raw patch files. Exact fullbase/current and predecessor/current unified diff bytes are built in memory, including new textual source and correct no-final-newline markers; JSON encodes all bytes, including whitespace, without loss. New check logs follow same rule. Persist no decoded temporary raw artifacts under active repository; reviewer decodes in memory or an authorized transient location only.

## Future manifest / review protocol

V4 manifest binds full final membership/hash/mode as appropriate, exact accepted source+test/review evidence, predecessor V3 manifest/review/source commit, encoded containerSHA and decoded rawSHA/bytecount for each diff, exhaustive partition and separate evidence inventory. Verify source and membership before/after extraction; all input references portable. Unknown postreview drift blocks new snapshot. Product source count is only fixed at actual final freeze.

Independent reviewer must verify container bytes SHA, strictly decode full base64 and verify rawSHA/count, then reconstruct EVERY fullbase-to-current file from decoded unified diff and original base preimages. Likewise reconstruct exact prior-V3-to-V4 delta using verified committed/predecessor V3 source. Verify partition completeness/disjointness, unchanged source hashes/prior111 review, entire3file repair and all relevant unchanged interactions. No truncated decoded inspection counts as full coverage. PASS only all final membership covered via prior equal-hash review plus full independent delta and no actionable findings.

N1 preserved four actual old-CSS controlled-interception failure proofs parent-reported. These are local controlled-position tests, NOT measurements of hosted pixel geometry. Preserve original hosted20PASS2FAIL, sourceproof failures and postrepair results separately; do not fabricate screenshots/hostvisual observations. Actual postrepair source/dist viewport matrices/fullbuildVitest/fresh3file review remain pending. Verify ordinaryClose/dropdown/backdrop guards, no pointer/z-index/visibility test overrides, originalR2 untouched, Boolean menuclass/data/state/ARIA unchanged.

Actual N7 dispatch after final binding only. Own new review evidence must use encoded rawlogs/diffs and portable metadata; no product/attributes/Git/receipt edits. New sourcecommit/fullcleanDEC025/localCI/exacthostedCI remain later gates; no local result implies hostedPASS or lifecycle acceptance.
