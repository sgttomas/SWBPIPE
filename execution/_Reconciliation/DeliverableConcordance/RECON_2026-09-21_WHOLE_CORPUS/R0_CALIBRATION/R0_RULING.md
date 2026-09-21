# R0 ruling — conventions, grain and scale-out

The owner's ruling on `R0_OWNER_PACKAGE.md` (SHA-256 `d890a65aa2f2acfb5d9b55ca9839e8024097437fc9a71713da4241909a63f253`), recorded by
Agent 0. Stored-transcript custody for session
`efe0b4ff-c1b6-4a97-af70-b04220464651.jsonl`, entry 1305,
2026-09-21T18:53:48.237Z. UTF-8 with no trailing newline; not original
transport bytes. The text is the host's answer record and includes the
questions as Agent 0 posed them.

SHA-256 `0b2502d90d1a874782d268a6236c3a4302fa2198333b5a67db34f638d652d532` (1131 bytes).

~~~~
Your questions have been answered: "Adopt the R0 amendment set? It covers: A3 split into authority vs evidence (with a git-grep discovery step), the C6 rules (including no 'aligned by construction'), GATE: evidence token, new reverse answers, new/narrowed cause tags, extractor v2 re-keying, an Agent-0 canonical situation table (reviewed, shown to you at the first-wave checkpoint), a whole-tree R1 inventory, and a 12-deliverable first wave that scales only if firm false alignment ≤5%."="Adopt as proposed (Recommended)", "May a verbatim, hash-bound owner decision recorded in an AgentRuns owner-direction/adoption record (e.g. DEL-11-01's R18 guide-edit exception) support ACCEPTED_DIVERGENCE?"="Yes, flagged for R4 confirmation (Recommended)", "Does DEC-101's 'other governance documents take the name at their next amendment' reach the ~100 deliverable Scope of Work files?"="No — residue is a finding", "ISSUED DEL-01-01 still shows maintainer roster, quorum, release authority and signing as TBD, which DEC-027 and DEC-057/089 later ruled. How should that be recorded?"="One lifecycle-reassessment group (Recommended)".
~~~~

## Effect

1. **The amendment set is adopted as proposed.** It comprises:
   - the convention dispositions and exact texts in `R0_REVIEW.md` §4;
   - the cause-tag changes in `R0_REVIEW.md` §4 (C7);
   - extractor v2 and re-issued claim keys, per `R0_REVIEW.md` §5;
   - the Agent 0 canonical situation table, independently reviewed and
     shown to the owner at the first-wave checkpoint;
   - the whole-tree R1 reverse inventory, per `R0_REVIEW.md` §6;
   - the scale-out gate in `R0_REVIEW.md` §7: a 12-deliverable first wave at
     double verification sampling, with scale-out only if firm false
     alignment is at most 5% and no shared-situation conflicts remain.
2. **A3a is adopted.** A verbatim, hash-bound owner decision in an
   owner-direction or adoption record that names the exact divergence may
   support `ACCEPTED_DIVERGENCE`. Such a row writes
   `OWNER_DIRECTION_RECORD:<path>` in Notes and goes to the owner at R4 for
   confirmation.
3. **DEC-101 does not reach deliverable Scope of Work files.** Rename residue
   in a deliverable is a finding, cause `RENAME_OR_IDENTITY`. It is not an
   accepted divergence. C6(e) still applies to other rulings that schedule a
   catch-up, but never to DEC-101 for deliverable files. R3 clusters these
   findings as one cause class for a single R4 ruling.
4. **ISSUED text overtaken by later rulings** (DEL-01-01: maintainer roster,
   quorum, release authority and signing as TBD, later ruled by DEC-027 and
   DEC-057/DEC-089) is one `LIFECYCLE_REASSESSMENT_REQUIRED` finding group
   (C6(d)). It goes to R4 on the ISSUED change path.

The bound conventions are in `../CONVENTIONS.md`. Calibration ledgers remain
calibration evidence. All eight pilots are re-encoded in the waves with the
named repairs. No substantive finding is decided by this ruling.

## Addendum — owner items after the R0 PR review (2026-09-21)

The independent review of the R0 PR routed two items to the owner. The owner's
answers, as the host recorded them (transcript entry 1618,
2026-09-21T19:34:05.750Z; stored-transcript custody). The record includes the
questions as Agent 0 posed them. SHA-256 `53862dca088906793d0426abf68456c3f57067df8f374a2c043f762a63716c28`
(1209 bytes).

~~~~
The user answered: "The R0 grain text said to issue every table row as its own required key, and also promised ~20–25% fewer rows. On this corpus those conflict: mandatory table rows would add 5,597 rows (~77% more work). I made table-row keys optional: fixed, deterministic keys, but a row is needed only when a worker splits a block, and then all of that block's rows are required. That gives a ~22% reduction and coverage stays provable. Confirm?"="Confirm optional rows (Recommended)", "Rename residue (a deliverable still saying OpenPipeStress) is a finding under your ruling. The canonical row now sets it to route to you (AuthorityNeeded=OWNER), clustered as one class for a single R4 ruling. The four hard identifiers (.opsproj, openpipestress_jcs_ijson_v1, openpipestress_result_semantics_v0_2, openpipestress-projects.sqlite3) are treated as NOT residue, based on your hash-bound selection 'Keep all four as they are' recorded on 2026-09-18 in the UI run's activation record. Is that right?"="Those look like residue in run records or as references to things in the past.  If they are meant to identify active parts of the code then we need to revise them to be compatible with the current name.".
~~~~

Effect:

1. **Optional `.rNN` rows are confirmed.** Table-row keys are fixed and
   deterministic. A row is required only when a worker splits the block, and
   then every row of that block is required. This replaces the literal reading
   of `R0_REVIEW.md` §5 item 2.
2. **Active code identifiers that carry the former name are rename residue.**
   The owner's words: "Those look like residue in run records or as references
   to things in the past.  If they are meant to identify active parts of the
   code then we need to revise them to be compatible with the current name."
   - Agent 0's application: the four identifiers kept on 2026-09-18
     (`.opsproj`, `openpipestress_jcs_ijson_v1`,
     `openpipestress_result_semantics_v0_2`, `openpipestress-projects.sqlite3`)
     identify active parts of the code. So they are findings in the rename
     class, not exceptions. The 2026-09-18 selection is recorded as context.
   - The two hash and result-semantics identifiers name frozen contracts, so
     renaming them is a code change touching a protected baseline. They go to
     R4 as candidate code-change items (`BaselineClass=FROZEN_CONTRACT`). This
     run changes no code.
   - Mentions in run records, history, or references to past states are not
     residue.
   - This application is Agent 0's reading of the owner's conditional words.
     The owner may correct it.

3. **Consistency enforcement replaces one canonical row per shared hash**
   (`R0_REVIEW.md` §5 item 5). The owner confirmed this separately (transcript
   entry 1655, 2026-09-21T19:37:43.657Z; SHA-256 `7b51877bd30b4f30bd24c6292d6ca97b9b9fc5cda2663c8ea42265b230e1b41b`, 651 bytes):

   ~~~~
   Your questions have been answered: "One more departure the reviewer says needs you. The adopted grain text asked for one pre-decided canonical row per shared-text hash. For 18 shared texts that can't be pre-decided without reading each deliverable's evidence (e.g. identical Remaining items across PKG-07, some Architecture Basis variants, PKG-00 status histories), I instead enforce consistency: same-package deliverables go to the same manager/worker, the validator flags any two rows with the same text that are judged differently unless justified, and verifiers review 100% of those rows. Confirm?"="Confirm consistency enforcement (Recommended)".
   ~~~~

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
