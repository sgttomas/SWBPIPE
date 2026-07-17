# SCA-007 Handoff State

**Upstream accepted basis:** D-47 O-A / `DEC-080` on `main` (PR #255,
`edaee30ec`), over the post-PR-#253 tree (`886bdea18`).
**Derivative-package status:** this snapshot folder is the SCA-007 derivative
record; `_LATEST.md` points here. `SOFTWARE_DECOMP.md` is at revision v0.10.
**Closure verdict:** SCA-007 amendment actions A001–A026 fully executed and
verified; scope change CLOSED at the execution PR merge. No rerun required
unless the execution PR fails to land.

## Named residuals (open, for future governed work — not blockers)

1. **Validation-manual v0.1 token re-key (deferred at fan-in):**
   `docs/validation_manual/index.md:62` and
   `docs/validation_manual/cases/generate_validation_case_pages.py:4,219`
   (plus one generated case page) cite v0.1 "PRD section 16.2/16.5"
   numbering. Deferred because it touches an executable generator (DEC-025
   sweep territory) and regenerated pages — outside this pre-accepted action
   set. Candidate DEL-09-04 hardening item for ordinary loop selection; the
   citations remain resolvable via the v0.3 citation-resolution note
   meanwhile.
2. **DEL-09-04 reproduction run:** now unparked and agent-executable
   (actor-neutral criterion, ruled bundle home
   `validation/evidence/reproduction/<run-id>/`) — ordinary loop work, not
   part of SCA-007.
3. **External-prover activation:** remains a future owner-gated register row
   (CAEPIPE procurement); `DEC-080` changes posture only.
4. Pre-existing, noted in passing (not SCA-007 work): the
   `SOFTWARE_DECOMP.md` revision paragraphs are listed out of order (v0.9
   before v0.8); left as found.

## Rerun triggers

Changes to the adopted PRD text, a successor scope change amending §22/§24,
or a ruling that reverses the relocation would require a new SCA with its own
packet — never an edit to this snapshot.
