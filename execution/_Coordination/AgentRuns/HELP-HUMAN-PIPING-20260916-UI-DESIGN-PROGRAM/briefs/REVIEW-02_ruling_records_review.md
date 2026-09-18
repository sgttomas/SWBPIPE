# REVIEW-02 — independent read-only review of the D-71 and D-72 ruling records

Sealed brief. Role: TASK (Type 2), fresh read-only reviewer, working alone; you do not delegate and have no write target: your return is your final message. Parent: ROOT of run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Model requested: Opus. Its precedent and form are `REVIEW-01_successor_records_review.md` beside this file.

## Candidate

Resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`. Review `git diff origin/main..fe0b86e12` in full **except** `briefs/DESIGN-SYSTEM-03_revision.md` and the `briefs/_INDEX.md` lines about it, which are a sealed design brief outside this review. Ignore uncommitted changes in the working tree: another child is writing under `instances/DESIGN-SYSTEM/`. Read-only git only; no build, test or benchmark.

## The test that matters most

The owner's complete message is the fenced block in `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md`. Extract the block's bytes (between the fence lines, no trailing newline), confirm 2,515 bytes and SHA-256 `711ca05ef7cb9bcaf6b7f0e6812ad002be58d8f9793aa68a08d0f19bceaf7944`. Treat that block as the only evidence of what the owner said. Then, for every statement in the diff about what the owner ruled, decide whether the block supports it:

1. **Verbatim quotes.** Every quoted owner line in the two packets' Human Ruling sections, the ruling records, the `DEC-099` to `DEC-103` rows and the notice is a faithful quotation of the block (whitespace runs may be collapsed inside table rows; nothing else may differ). Report any altered word.
2. **No ruling beyond the words.** D-71 item 7 and D-72 item 5 are recorded as NOT ruled everywhere. Where the owner amended beyond the packet's options (D-71 items 2, 3, 9), the recorded effect is either in the owner's words or is labelled as ROOT's reading. Report any place where a reading is presented as the owner's ruling, where an effect is wider than the words support, or where an effect the words require is missing. Give particular attention to: whether "Do not call it a Technical Preview" is handled consistently with item 1's ruling; whether the acceptance-sentence removal is bounded honestly (what it touches and does not); whether D-72 items 2 and 3 are recorded as accepted as recommended with the supplement's S-2 and S-3 clearly still unruled; whether the owner's clearance statement is recorded without being inflated into closing `PB-TBD-004`.
3. **Nothing executed that needed more authority.** `docs/claims_registry.md`, `docs/PRD.md`, `docs/report_notice_template.md`, product source, schemas and packaging are unchanged in the diff. SCA-010 is marked proposed, has no acceptance record, and `execution/_ScopeChange/_LATEST.md` is unchanged. No text says implementation is authorized.
4. **Packets append-only.** Both packets differ from `origin/main` only by added lines; the hashes the ruling records bind for the pre-append packets match `git show origin/main:<path>`.
5. **DEC rows.** `DEC-099` to `DEC-103` are the next free identifiers, have the table's column count, match the ruling record, and the counts they cite (53 placements, 21 files, seven assertions) match `instances/RESEARCH/F_packet_bindings.md`.
6. **SCA-010 arithmetic and facts.** 24 occurrences of the former name in `docs/PRD.md` (title, working title, two notice copies, 20 others); lines 1249 and 1786; three template lines 16, 41, 68; the lint-anchor claim against `tools/validation/validate_claims_language.py`.
7. **D-72 supplement.** Internally consistent arithmetic (six runs against thirty); no score of either product is used to justify the 80 % line; trace spans, presentation feedback and scanout stay distinguished; S-2 honestly states that it sets aside part of D-70 effect 6 and that this needs the owner's act.
8. **Withdrawal.** The observation brief and its notice are annotated, not rewritten, and nothing still asks the piping session to act.
9. **Hygiene.** No absolute machine path; relative links in changed Markdown resolve; governed records end with the standard claim fence line; `WORK_GRAPH.json` parses and its edges name existing nodes; Canadian English; the barred control words are not used for a product control.

## Return

Verdict `PASS` or `FINDINGS`, the reviewed SHA, findings by severity with file, line, what is wrong and the evidence, and what you did not check. No praise and no summary of content.
