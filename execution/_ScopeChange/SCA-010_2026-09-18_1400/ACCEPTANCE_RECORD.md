# SCA-010 Acceptance Record

**Accepted by:** Ryan Tufts, owner, in session on 2026-09-18.
**Owner's words:** "I accept SCA-010." (stored transcript timestamp 2026-09-18T13:55:47.429Z, 70 bytes, SHA-256 of the extracted bytes `5cf6071cf15d7ccf43b91197e7a712019a2bb15c414ba6f777d65bfc48193bc8`; an in-session extraction from the host's stored transcript, not original transport bytes; the whole message is reproduced in `execution/_Coordination/_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md`.)
**What is accepted:** the amendment as the bundle states it: actions A001 to A006 of `Amendment_Actions.csv`, the PRD name change to SWBPIPE with the header changes, the three template occurrences, and nothing else.
**Precedent and form:** `execution/_ScopeChange/SCA-007_2026-07-16_2026/ACCEPTANCE_RECORD.md`.

## Correction after acceptance

Independent review REVIEW-02, reading the bundle as it stood before acceptance, found that it cleared all 24 occurrences of the former name for replacement, when one of them (PRD line 15) is part of the path to a preserved historical file. The bundle's own non-scope already said ruled history keeps the former name, so the correction narrows the actions to what the accepted brief intended: 23 occurrences replaced, one excepted. The owner accepted the bundle before the correction; the correction removes an edit and adds none.

## Execution: accepted, not yet executed

The actions move from PROPOSED_PENDING_ACCEPTANCE to ACCEPTED_PENDING_EXECUTION. **No PRD or template text has changed.** The design program does not execute them, for two reasons recorded here: its assignment is design only, and the bundle's own impact assessment names one ordering risk, a build in which the PRD's notice and the report renderer's emitted notice differ in their first word, which is removed by executing the PRD text, the template and the renderer together in the implementation tranche that executes `DEC-101`. One declared departure from `Brief.md`, which said the latest-scope-change pointer moves at acceptance: `execution/_ScopeChange/_LATEST.md` moves when the actions are executed, so that the pointer never names an amendment whose text is not yet in the PRD. The executing loop is whichever the owner names when authorizing that tranche.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
