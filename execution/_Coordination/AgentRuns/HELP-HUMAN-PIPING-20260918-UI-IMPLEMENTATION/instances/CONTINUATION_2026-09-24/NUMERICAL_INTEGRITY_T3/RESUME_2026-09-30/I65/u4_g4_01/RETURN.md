# I65 U4 G4: return

**G4 is complete.** It covers:
- T16–T19;
- the U1 text part of T08;
- the T24 composition under the restated admission law (S-5);
- every routed repair;
- RV82-N5;
- the D-6 re-pin.

**The margin rule trips. ROOT must choose.**

**The headline.** At the D1 caps, with ε = 2 and illustrative strides, the admission maximum is **0.9115 M (sparse) and 0.9164 M (dense)**, at phase **W3**: publication on branch W. W4, the precommit reader, is within 10 MB of it.
- **Against M:** it fits.
- **Against the 0.9 M rule:** it is over by 46.5 MB (sparse) and 66.2 MB (dense).
- **Branch X:** 0.855 / 0.860 M. It passes.
- **At the milestone's own facts:** 0.084 M.

## Decisions for ROOT

1. **The margin option** (COMPOSITION_G4.md §4–§6; sensitivity in `_run_records/sensitivity.out.jsonl`).
   - **Single-cap reductions that pass:** m ≤ 30, n ≤ 28, g ≤ 24, r ≤ 96, l ≤ 160, or the text cap ≤ 64. For a comfortable margin: **m ≤ 28 (0.860 M) or l ≤ 128 (0.856 M)**.

   | Option | Cost | Risk |
   |---|---|---|
   | **Tighter caps** | a D1 cap edit and a minutes-long re-run | narrower coverage. A thin cap such as m = 30 could re-trip at G5's in-build strides |
   | **Lifetime-aware text model** (a separate grant) | about 1–2 days plus review | a harder-to-review model; the largest gain, over 1 GB |
   | **Per-invocation bound at the census facts** | a G5 evaluator plus tests | derived atoms (D, D_env, text classes) stay cap-priced; the permit becomes input-dependent |
   | **Phase-aware ordinary span** (not on ROOT's list; §5) | about 1 h of records | low to moderate; a per-family "dead after G-C" argument. Alone it gives 0.856 / 0.860 M |

   - **RV84's levers:** N-6 acts only on X, which already passes. S-7 is already applied.
   - **My recommendation:** caps plus the phase-aware span. Failing that, m ≤ 28 or l ≤ 128 alone.
2. **The D-6 reviewed-lock record** (NOTES_G4.md §3) should also bind the reader's 13 `include_str!` static hashes, and G5 should add layout witnesses for the reader types T17 uses. Accept or decline.

## Deliverables

| Item | File | Result (caps, ε = 2) |
|---|---|---|
| T16 publication over U1 (staging, hashing, canonical serialization) | PUBLICATION_READER.md §1–2 | **1,389,829,540 B** (P2: members + body copy + `hash(publication)`). Successor output 190.6 MB, moved. Milestone cross-check: every bound component ≥ measured |
| T17 precommit reader | PUBLICATION_READER.md §3 | **1,275,668,333 B** (V2: G1's whole-successor clone and the publication hash) + 5,397,696 B of statics. `integral_receipt` is borrowed. The schema walk is ≤ 36 levels |
| U1 text part of T08 | PUBLICATION_READER.md §4 | serializer 0.47 MB; capture 593 MB; U3 dispatch 11.9 MB; reader 276 MB (TAV) |
| T18: fallback copy (D-b), N1 reserve, transfer | TRANSFER_COMPLETION.md §1–4 | staged copy 105,778,121 B (D-b's ~68–69 KB experiment copy, priced at the caps; 4.9 MB at the milestone's facts); N1 757,746 B; transfer moves only |
| U3 budgets B-1 to B-10, including the no-fallible-allocation-after-first-mutation property and **"exactly one ordinary run per invocation" (S-7)** | TRANSFER_COMPLETION.md §3–4, **§7** | all met at grant 1. **Grants 1b and 1c (`a634ac8b53`, merged during G4) were checked: B-1, B-3, B-6, B-7 and B-10 are met** |
| T19 Direct completion | TRANSFER_COMPLETION.md §5 | 10,240 B illustrative. s(output) was corrected from 1,024 to 2,048: the inline admission report. The maximum is unchanged |
| T24 per caller and mode; the margin rule; sensitivity; options | COMPOSITION_G4.md | above. Direct only, because `admit` refuses Headless (F-5, G5) |
| D1.10 and D1.11 refusal kinds | NOTES_G4.md §1 | confirmed: D1.10 → `Family`/`source_family`; D1.11 → `Cap(control_bytes)`/`resource_admission`. Both already in the schema enum |
| RV82-N5 | NOTES_G4.md §2 | recorded. The producer's conservation leaves build flags (reader :1089–1168) and execution order (:710) to the reader at precommit |
| D-6 lock re-pin | NOTES_G4.md §3 | lock `4f494db6…475b` (37 packages, 22 registry). U3 did not change it. The result_export closure is already pinned. Add reader layout witnesses and the static hashes |
| G5 carry list | NOTES_G4.md §5 | ROOT's four notes and seven raised in G4 |

## Each routed item

| Item | Where | Disposition |
|---|---|---|
| RV83 **R-1**: the call graph | G3_REPAIRS.md §1 | **Repaired.** R1a–R1l cover generic, trait and alias receivers, blanket impls, chained calls (METHOD now used), turbofish and `<X as Tr>::f`, the lexer, `:` lookbehind, array-type signatures (194 functions had been dropped), and test items blanked in place (U3's mid-file test module would have dropped 10k lines).<br>**Audit:** no unmatched call token.<br>**RV83's evidence:** 64/64 reached; 33/36, with the other 3 explained.<br>"Never under-counts" is replaced by a stated method and six known limits. TEXT and STACK were rerun |
| RV83 **R-2**: `validate_profile` | §6 | the exclusion is dropped and the path priced: pressure_runtime is 25.6 MB of TAV at the packet's byte classes. No D1 clause |
| RV83 **R-3**: the O-N provenance parse | §7 | added at 48,000 B (non-object Value ≤ 128 B, push-built) |
| RV84 **S-1**: the call after `:` | §1 (R1h) | repaired. It exposes T25's commitment text (71.2 MB TAV) |
| RV84 **S-2**: the broad zero rules | §9 | anchored. Specific rules come first for the six cited loops. A self-check (`zero_matched_headers`) found a seventh, `ambiguous_supports`. +97.3 MB TAV |
| RV84 **S-3**: the per-string temporary | §4 | added to the hash route at each Value's L_max, plus the parser scratch. `parsed()` now uses 6 × slots. T16 and T17 use the repaired route |
| RV84 **S-4**: T25's coefficients | §5 | the identity string, the N² aggregate-bits matrix and the factor objects are added. T25 = 1,475,185,857 B |
| RV84 **S-5**: the admission law | COMPOSITION_G4.md §1 | applied: seven phases X1–W5, the maximum over both branches through completion |
| RV84 **S-6**: API.md | API_G4.md | exact hook signatures, and the facts each gate reads. G-B gets no rows, diagnostics or errors. `source_cases` is dropped. The late capture is measured at G-C. The label is now 2·Text(diag_env) = 157,720,102. The hook edits are I61's (D-5) |
| RV84 **S-7**: the double count | §10; TRANSFER B-1 | the Direct root only, with `fn_cap` 1 on the observed run. It is recorded as U3 budget B-1, met by structure |
| RV84 **N-2**: `?` into `CaptureError` | §8 | the claim is corrected, and lexicon kind `from_literal` added: 85 rows, 7.8 MB |
| RV84 **N-3** with RV83 R-1: no mutual recursion | §3 | **re-established** on the repaired explicit graph: 22 self-loops, 19 reachable. RV84's 17 candidate cycles were name fan-out, adjudicated by cited rules (including its traced `WorkTotal::add` collision). 18 implicit-call candidates: 16 false by type structure, 2 genuine and depth-bounded. R and k are unchanged |
| RV84 **N-7, N-11, N-13** | §11; COMPOSITION_G4.md §1 | wording corrected. The moving candidates are enumerated, and G3's undocumented 2 MiB is removed |

## Execution record

**Who.** TASK I65 (Type 2) under ROOT. No descendants.

**When.** Granted at 00:00 MDT on 2026-10-04 and returned at about 02:00 MDT: about 2 h of wall-clock time against a 6–9 h budget. The context was compacted twice; the work continued from the records.

**Basis.**
- NUM moved during G4:
  - `3260d7809e` → `b1f80234dc` (U3 grant 1);
  - → `a634ac8b53` (grants 1b and 1c);
  - head at return: `61a474dd4e`.
- G4's arithmetic is pinned at `b1f80234dc`, read from a `git archive` snapshot under WT/scratch/i65_u4_g4_01/.
- G3's rules were carried by checked line mapping.
- Grants 1b and 1c were read through `git show` for TRANSFER_COMPLETION.md §7.
- No uncommitted code was read. I61's working tree and grant 1d were not opened.

**Memory guard.** `memguard.sh`, PID 5387, was running throughout and at the seal.

**Not run.** No Cargo, rustc, solver, native or DEC-025 job. No installs and no new host tooling. Git reads only (`show`, `archive`, `log`, `diff`, `rev-parse`, `grep`, `status`), all with `GIT_OPTIONAL_LOCKS=0`.

**Run.** Stdlib Python and shell scripts in `_run_records/` (listed in COMPOSITION_G4.md and G3_REPAIRS.md).
- At the seal, the caps chain (`sens.py`) and the milestone text runs were rerun from the packet's scripts, and they reproduced the packet's outputs **byte for byte**.
- `g4_caps.*` was then regenerated after the s(output) stride correction.

**Writes.** Only R/I65/u4_g4_01/. Scratch was under WT/scratch/i65_u4_g4_01/, never the system temp directory. A search found no machine path in the packet.

**Host-record read.** After a compaction, I read this session's own transcript once to recover the exact pipeline command used for the outputs.

**Not touched.** R/REVIEW_RV83/ and R/REVIEW_RV84/ were only read; RV85, RV86 and I66 were not opened.

**Stops.** None was reached:
- no contract reading went beyond the rulings;
- every term has a closing route;
- no write was needed outside the folder.

**Integrity.** `SHA256SUMS` covers every file in the packet except itself.
