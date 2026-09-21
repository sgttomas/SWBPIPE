# Editor ownership defects

Two deterministic regressions failed in ownership-repro-01.txt before repair. Expected invalid A survives click B then Enter/character; actual startEdit replaced A because its guard only excluded busy/pending, not an existing invalid owner. Expected external C focus and selection survive delayed A-to-B navigation; actual focusCell(next, false) skipped DOM focus but still invoked onSelect(B).

High-confidence source causal chains: (1) failed Apply returns with edit A; B still receives keyboard event; non-editing branch starts fresh capture B. (2) completion tests focus ownership only for DOM focus argument, not the entire selection action. Bounded repair: prevent replacing any owned editor until accepted/Cancel; redirect eligible B keyboard edit attempt to A; gate the entire deferred navigation on continued focus ownership. Additional containment correction: blur exemptions recognize only this table root's own cell/action elements, so another table's controls do not suppress blur Apply.

Affected: direct table interaction only. Existing controller operations/history remain unchanged. Regressions retained unchanged; browser/native ordering still needs its separately granted witness. Software-defect-diagnosis skill is the same hash recorded in FOCUS_DEFECT_DIAGNOSIS.md; existing task authority permits core repairs.
