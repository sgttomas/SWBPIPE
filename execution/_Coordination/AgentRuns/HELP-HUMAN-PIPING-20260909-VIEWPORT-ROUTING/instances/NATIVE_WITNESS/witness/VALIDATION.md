# Native witness validation

Verdict: `PASS`

| Check | Result | Evidence |
|---|---|---|
| Exact source/app/store identity before UI write | PASS | `IDENTITY.json`; ten member hashes matched, PID/executable/plist/identifier exact |
| Native application, no browser substitution | PASS | CUA app identity and screenshots |
| Elevated From node anchors plane | PASS | `node:N-140` at `(7.6,2.4,2.2) m`; UI read `XZ · Y=2.4 m · through node:N-140` |
| Applicable axis control | PASS | X and Z enabled in XZ; selected X; Y disabled |
| Orbit drag does not author | PASS | Native status explicitly rejected movement over 4 CSS pixels; endpoint fields and model count unchanged |
| No-hit pointer guard | PASS | Blank click reported no finite intersection and authored nothing |
| Hover ghost and pointer capture | PASS | Stable visible hover `(5.66,2.4,2.2) m`; unchanged-position capture `(5659.782,2400,2200) mm`; captured ghost unchanged at displayed precision |
| Mixed-unit conversion | PASS | `5659.782 mm = 5.659782 m`; main saved `2928.619 mm = 2.9286190000000003 m` under JSON floating representation; Y/Z exact |
| Add freezes review without model mutation | PASS | 26 native entities and 5/4 saved model before Apply; validated two-operation atomic batch displayed |
| Apply commits one atomic new-end batch | PASS | Native model moved directly from 26 to 28 entities with exactly one node and one pipe |
| Continuation field semantics | PASS | From advanced to new endpoint; selected reusable fields retained; consumed identities/endpoint fields cleared |
| One Save persists exact result | PASS | Store moved from 5/4 to 6/5 only after one Save; exact node/pipe JSON verified read-only |
| Quit/reopen persistence | PASS | Relaunched exact bundle as PID 30116; Open local restored 28 entities, exact IDs, exact coordinates and connectivity |
| Post-probe model/store integrity | PASS | Uncommitted pointer probe canceled; no second Add, Apply, or Save |

The initial finite-hover screenshot and first main capture differ slightly in their displayed X readout because CUA moved the pointer from the prior hover location to the visible endpoint pixel before pointer-down. That attempt remains in the action record. The subsequent stable-pointer probe held pointer and camera position unchanged and passed the required displayed-hover-to-full-field comparison.

Candidate qualification: the parent reported that combined independent source review passed against the same ten-file source cut while this witness was running. This witness remains evidence for the candidate build and does not itself accept source, update lifecycle state, or waive CHANGE/DEC-025 gates.
