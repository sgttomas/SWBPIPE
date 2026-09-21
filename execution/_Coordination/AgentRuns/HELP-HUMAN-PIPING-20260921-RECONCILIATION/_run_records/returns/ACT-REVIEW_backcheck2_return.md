VERDICT: PASS

Scope: the delta `dcbfda55b..20192a20c` (commits `8f4153a76` and `20192a20c`), 3 files, all under `projects/chirality-piping/`. HEAD is `20192a20c`.

**(a) N-1 is closed.** Lines 18–19 of `D-73_RULING_2026-09-21.md` now read "reproduces from the packet as committed at `5438c1c98` by replacing the body of its §6 with the single line `*(Awaiting ruling.)*`". Doing that recomputes to `0241867d9c0cac7d54b9c7aa1ca0434b8c05ae7a560f6616543ecd0384a1a879`, which matches the stated hash. Nothing else in the ruling record changed.

**(b) The backcheck-1 return is complete and matches.** `returns/ACT-REVIEW_backcheck1_return.md` at `20192a20c` is 6169 bytes. It runs from `VERDICT: FINDINGS` to `END-OF-RETURN`, with no trailing newline. I read it section by section against my backcheck-1 return and it matches, with nothing missing: the opening scope paragraph, sections (a) to (e), N-1, the 10-row hash table, the validator outputs and the claim fence. As before, I compared it by reading, not with a byte diff. At HEAD, this complete copy has replaced the truncated one from `8f4153a76`.

**(c) Every hash in `WORK_GRAPH.json` matches its file at `20192a20c`,** and the file parses as valid JSON.

| Record | Stated | Recomputed | Match |
|---|---|---|---|
| `OWNER_DIRECTIONS.md` | 03738706…0b6b | 03738706…0b6b (6752 B) | yes |
| `instances/ROOT/ENTRY_BRIEF_2026-09-21.md` | 3767db25…7823 | 3767db25…7823 (8784 B) | yes |
| `PLAN.md` | 18d39600…6381 | 18d39600…6381 (13178 B) | yes |
| `briefs/ACT-REVIEW_brief.md` | 17a385b4…c796 | 17a385b4…c796 (4413 B) | yes |
| `returns/ACT-REVIEW_return.md` | 23bf3049…b3f | 23bf3049…b3f (10232 B) | yes |
| `returns/ACT-REVIEW_backcheck1_return.md` | 22f829c1…3469 | 22f829c1…3469 (6169 B) | yes |
| Profile as merged (ruling record) | 271bd0d0…da73 | 271bd0d0…da73 (file unchanged in this delta) | yes |

**(d) No new defect.**
- The `ACT-REVIEW` node's new status and `returnCustody` fields describe the record accurately.
- The profile, packet, register and `DEC-110` are untouched in this delta.
- The only blemish is cosmetic: the repaired sentence leaves `` `5438c1c98` by replacing`` as a short wrapped line in the ruling record.

**Validators** (`PYTHONDONTWRITEBYTECODE=1`, HEAD `20192a20c`)
- `python3 tools/validation/validate_claims_language.py` printed "VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied" (exit 0).
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` printed "VALID …/projects/chirality-piping/loop/LOOP_RECEIPTS.md: frozen through Receipt-44; versioned receipt contract satisfied" (exit 0).
- `git status --porcelain` is empty, so this backcheck left the working tree unchanged.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN