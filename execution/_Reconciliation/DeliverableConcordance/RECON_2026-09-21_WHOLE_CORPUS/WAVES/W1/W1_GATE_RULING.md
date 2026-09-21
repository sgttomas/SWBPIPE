# Wave 1 gate ruling

The owner's ruling on `W1_GATE_ASSESSMENT.md` (SHA-256 `0440ef3b8c2b408ce7b01bb6c94ff654f750adc2340dd91a949e706020b1eed7`),
recorded by Agent 0. Stored-transcript custody for session
`efe0b4ff-c1b6-4a97-af70-b04220464651.jsonl`. UTF-8 with no trailing
newline; not original transport bytes. Each text is the host's answer record
and includes the question as Agent 0 posed it.

## Owner act 1 — entry 2577, 2026-09-21T22:14:25.716Z

SHA-256 `1d9297dbdacbc6d6565fe4a8332873bb2bd135b147458c1bee0450b52572c364` (209 bytes).

~~~~
Your questions have been answered: "Wave 1 missed the gate as written, and all five errors have one cause. How should I proceed?"="Adopt fix, second gate wave". You can now continue with these answers in mind.
~~~~

## Owner act 2 — entry 2604, 2026-09-21T22:17:58.948Z

SHA-256 `858bc341574b85f1d53c1b2bbe3abd0e90b4c9dcc07597afd8f8e2f6ebf49175` (361 bytes).

~~~~
Your questions have been answered: "Can an engine that is tested but has no product caller satisfy a claim?"="Judge by the claim's subject (Recommended)", "Which tier applies to a stale record clause inside a requirement that restates a boundary (IP/data, human authority)?"="Tier of the gap wins (Recommended)". You can now continue with these answers in mind.
~~~~

## Effect

1. **Remedy R-1 to R-6 is adopted** as written in `W1_GATE_ASSESSMENT.md`.
   The conventions carry it as Part F (F1–F6).
2. **A second gate wave runs before scale-out** (not the rolling queue). The
   gate conditions are unchanged. Wave 1 is not rerun; its disagreements are
   recorded in `WAVES/W1/RESOLUTIONS.csv`.
3. **Uncalled implementation (Part F, F7).** The claim's subject decides. A
   tested engine or library with no product caller satisfies a claim about
   that engine or library. It does not satisfy a claim about app or runtime
   behaviour; such a row takes `PARTIALLY_IMPLEMENTED`. Every `ALIGNED` row
   whose implementation evidence has no product caller carries
   `PRODUCT_CALLER: NONE` in Notes, for R3 clustering.
4. **Tier of the gap wins (Part F, F8).** It parallels C7, where the cause of
   the remaining gap wins. When a requirement restates a boundary invariant
   and only a record clause in it has gone stale while the boundary holds, the
   row takes the tier of that gap (usually `LOCAL_DESIGN`) and names the
   boundary in Notes. `INVARIANT` applies when the gap touches the boundary's
   subject.
5. **Not ruled here:** the DEC-094 / SCA-009 R-005 landing. It goes to R4 as
   an authority item.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
