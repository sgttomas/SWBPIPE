# REVIEW-02 — return (first pass)

Retained by ROOT on 2026-09-18 from the reviewer's final message under the brief [`../../briefs/REVIEW-02_ruling_records_review.md`](../../briefs/REVIEW-02_ruling_records_review.md). An in-session transcription by ROOT, condensed to the verdict, every finding with its evidence, what passed and what was not checked; the reviewer had no write target. ROOT's disposition follows each finding. The backcheck is appended when it returns.

Verdict: **FINDINGS**. Reviewed SHA `fe0b86e12319265155aa87c9d0e293d13036e913` against `origin/main` at `0d0021db71022a615f6ac162f273c03695c7beab`. The owner's block extracted from the ruling record: 2,515 bytes, SHA-256 `711ca05ef7cb9bcaf6b7f0e6812ad002be58d8f9793aa68a08d0f19bceaf7944`, both as stated. Every quoted owner line in both packets is byte-identical to the block; `DEC` row quotes differ only by collapsed whitespace; no word altered.

## Major

- **M-1. SCA-010 cleared a rename that would rewrite a live path to a preserved historical record.** The impact assessment said none of the body occurrences is a quotation of a historical record; PRD line 15 carries the name only inside the path `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, a file that exists, in a sentence that says ruled history keeps its tokens. Action A003 and the preview applied uniformly. The counts (24 in the PRD; 3 in the template at lines 16, 41, 68; notice copies at 1249 and 1786 identical) are sound. *ROOT: verified and corrected in the impact assessment, actions, preview and brief: 23 replaced, one excepted; the acceptance record, written after the owner accepted the bundle, records the correction as removing an edit and adding none.*

## Minor

- **M-2. `DEC-100` to `DEC-102` deny effects their own text requires** ("No engine, schema, test … effect" beside moved test assertions and a schema namespace rename); the register's D-71 scope column likewise. *Corrected: the rows and the register now say the row itself changes nothing and the act does when a tranche executes it.*
- **M-3. The lint's embedded copy of `BS-IP`** (`tools/validation/validate_claims_language.py`, registered-texts list) is not named as a `DEC-101` obligation. The anchor claims check out. *Corrected: named in `DEC-101`, with the note that the root tool edit needs its own instruction-surface authorization.*
- **M-4. The D-72 ruling's item 2 understated what was accepted** (omitted the pixel-ratio-1 and Dark observations that S-3 would drop). *Corrected in the ruling record; the addendum, written after the owner declined S-3, states them too.*
- **M-5. S-2's "by owner act" read as asserting a set-aside not yet made**, and nothing said which decision would carry the change to D-70 effect 6. *The owner has since ruled S-2; the addendum now says it is the record that carries the change and that the D-70 record is not edited. The supplement's wording is left as the owner read it.*
- **M-6. The Q-20 amendment was generalised without the "ROOT's reading" label.** *Corrected in the ruling record and the working state.*
- **M-7. The supplement's "six against thirty" was not like for like**; with the packet's declared observations it was thirty-six. *The supplement is left as the owner read it; the addendum states the correction and the governing figures (ten runs in the ordinary case now that S-3 is declined).*

## Trivial

8. A lowercased initial in one embedded quotation and one collapsed double space. *Corrected.* 9. The D-72 packet's blockquote rendered as one paragraph. *Blank quote lines added; the owner's lines unchanged.* 10. A repo-root tool path among project-relative targets in the actions file. *Marked.* 11. A stale clause in two work-graph notes. *Corrected.* 12. Whether the `BS-VALID` short variant survives on the export surface was unstated. *Stated as ROOT's reading in the ruling record: it stays.*

## What passed

Item 7 and item 5 were recorded as not ruled in every place they appeared; "Technical Preview" handled consistently and labelled as ROOT's reading; the acceptance-sentence boundary bounded in five labelled readings, reading (b) verified against PRD §19.3; the clearance statement not inflated into closing `PB-TBD-004`. No registry, PRD, template, product, schema or packaging change; SCA-010 proposed with no acceptance record and the pointer unmoved at the reviewed SHA; every "authoriz" sentence denies authorization. Both packets append-only; all bound hashes recomputed. `DEC-099` to `DEC-103` next free, correct column count, counts match RESEARCH-F. The 80 % line uses no score of either product; trace spans, presentation feedback and scanout kept apart. The observation brief and notice annotated, not rewritten; nothing asks the piping session to act. No absolute path; links resolve; work graph parses (29 nodes, 39 edges); claim fences present.

## Not checked

The sealed design brief and its index lines (excluded); uncommitted working-tree changes; the owner message's provenance beyond the extracted block; the earlier owner quote and its hash (no counterpart in the repository); the statement that ROOT put item 7's implications to the owner in session (no custody record in the diff); no build, test, lint or benchmark run; earlier index hashes and the PR #796 statements; whether setting aside part of D-70 effect 6 through D-72 is procedurally admissible.

## Backcheck

Retained by ROOT on 2026-09-18 from the same reviewer's second final message; in-session transcription, condensed. Reviewed SHA `48d3d3af68d54cd92ac70281035e92db1d49140e`. Verdict FINDINGS: four minor, three trivial, no major; all twelve first-pass findings corrected or honestly disposed of; this file judged a faithful condensation. The owner's two later messages recomputed from their strings: 70 bytes `5cf6071cf15d7ccf43b91197e7a712019a2bb15c414ba6f777d65bfc48193bc8` and 82 bytes `5c01428f3a49a383e95b1a5a6871fd5f8ad05c031f148e057f886b11101d0f78`, quoted byte-identically wherever reproduced. Item 7's option A matches the packet; S-3 declined leaves items 2 and 3 as first accepted; SCA-010 accepted and not executed, with no change anywhere outside `projects/chirality-piping/execution/` in the whole branch; `DEC-105` supersedes `DEC-099` without rewriting it, labels its reading of "all instances", claims no stage or release change, and cites the lint at the right lines (86; 104 to 106). Packets append-only; `DEC-104` and `DEC-105` next free with the table's column count; hygiene holds; work graph parses.

Findings, each corrected by ROOT in the following commit: F-1 the D-72 ruling record had no pointer to its addendum and still said item 5 was not ruled (section appended); F-2 the implementation prerequisites omitted `DEC-105` (added); F-3 the SCA-010 brief still said "None yet" under acceptance and two lines still said the pointer moves at acceptance (marked); F-4 the work graph had not received the last commit (notes corrected); F-5 lower-cased timestamp and "sha-256" in the D-72 addendum; F-6 message A's blockquote rendered as one paragraph; F-7 a doubled full stop after a restored quotation.

Not checked by the reviewer: the sealed design brief and the index paragraphs about it, including the record of the message to the running design child; uncommitted changes; the provenance of the two messages beyond the supplied strings and their timestamps; any build, test, lint run or CI; whether the owner should re-confirm SCA-010 after the post-acceptance narrowing, which the reviewer names as the owner's question.


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
