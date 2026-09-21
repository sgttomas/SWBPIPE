# Character-start repair return

TASK /root/b3_manager/b4_2_table, Astra/low, resumed Codex harness-native child of WORKING_ITEMS /root/b3_manager. No descendants. Source frozen on base 7b900eaad764603a3c58dd643ebaa34ea569a1d9; parent owns commit and independent backcheck. Exact four maintained paths and SHA256 identities are in SOURCE_FREEZE.json.

## Cause and repair

Native evidence showed q selected immediately after character-start. On unchanged production, the new regression sent individual q then r key events and observed r instead of qr (fail-before.log plus trace). The unconditional input onFocus select() selected the replacement character as though it were existing text. This is an initial-selection defect; no capitalization or OS-policy defect is inferred.

TableEdit now captures initialSelection=all for Enter/double-click and end for printable replacement. EngineeringTable applies that selection once per captured edit token on first focus. Later caret placement/refocus is untouched. No timer, global listener, operation/controller, native or OS preference change. Existing captured target/before/unit/generation/pending, Cancel/Apply/Keep and staging contracts remain.

## Checks

- Actual-key fail-before: one explicitly collected desktop case failed as expected, qr expected versus r observed. Source hashes and original failing trace retained.
- Focused core: 51 tests passed in four files, one worker; explicit direct/review numeric/text first-selection and later-refocus selection assertions included.
- TypeScript frozen check: exit 0.
- Browser: exact 10 collected cases passed with pinned Chromium 148.0.7778.96/revision 1223, source port 5174/shared lock, one worker. Eight new cases cover direct/review × Label/X × both configured profiles; separate ordinary keys, caret movement, action-control refocus, subsequent insertion, pointer Cancel, Apply/Keep, model Undo/Redo versus staging, Enter and double-click select-existing. Two existing connected text cases retain text Undo, model history, filter/focus, Queue/Clear and Save/Open assertions.
- Commands/results: CHECK_COMMANDS.json. Resource release: RESOURCE_RELEASE.json. No native/build/full suite/Git mutation or prior-evidence rewrite occurred. No tolerance or timeout changed. Source stayed frozen during both browser runs.

## Return boundary

Ready for independent affected source backcheck and parent commit. A newly built, identified native process must repeat the character/caret/text-Undo/Cancel/directly affected witness after separate authorization. Prior native observations retain their original failed/partial standing; browser passes do not retroactively change them. Other B4 holds remain.
