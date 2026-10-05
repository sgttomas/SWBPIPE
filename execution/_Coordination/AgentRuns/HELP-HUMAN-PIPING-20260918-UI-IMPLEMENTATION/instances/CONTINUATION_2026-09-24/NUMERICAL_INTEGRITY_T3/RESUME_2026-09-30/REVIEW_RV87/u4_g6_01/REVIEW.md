# RV87 confirmation: the U4 G6 identifier-class audit

**Reviewer:** RV87, TASK (Type 2), by ROOT's direction. No descendants.
**Candidate:** `R/I65/u4_g6_01/`, specifically ID_CLASS_AUDIT.md, `_run_records/text_g6/`, `id_audit/`, `per_identity/` and `controls/`. Its 75-entry SHA256SUMS verifies OK.
- The text basis is the `1e323058f3` snapshot.
- The code is `2bb81ec1ea` on the memory branch.

## Verdict: **NOT CONFIRMED**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 3 |
| NOTE | 3 |

| Item | Result |
|---|---|
| **1. RV87's 18 sites and RV89's 5 at the right bound** | **Confirmed.**<br>– Every one of the 18 is priced exactly at RV87's bound in the G6 run: 1,024; 1,053 and 1,054 for the two formats; 872, 292, 173, 158, 142, 140 and 322 for the template-bound ones.<br>– RV89's 5 are a subset of the 18.<br>– The 18 add +32,373,120 B to the whole TAV, equal to RV87's `remaining_total` |
| **2. The class is closed** | **Not confirmed.**<br>– A by-type probe found identifier-bearing copies outside the table, priced by a non-identifier rule: the node-DOF label `integrity_dof_label(..)` at 5 format sites, priced at the 20-B integer class (SF-1), and the adapter's `copy(s)` at 327 B (SF-2).<br>– The added text is +624,268 B on every branch. The rule still holds |
| **3. The enforcement is real** | **Partly.**<br>– Removing an entry whose fallback is an identifier rule does make TEXT incomplete. RV87 reproduced I65's control (`lib.rs:5592`) and added a second one (`retained_product.rs:280`).<br>– Removing an entry whose fallback is a non-identifier rule leaves TEXT complete, with the bytes silently lower (control c2).<br>– By construction, the enforcement cannot see SF-1 and SF-2 (SF-3) |
| **4. The TEXT and maximum deltas are right** | **Confirmed.**<br>– **The run reproduces exactly.** RV87 re-ran G6's TEXT (whole, W and X), and every row is identical: 2,149,902,046 / 1,569,180,716 / 1,439,555,190.<br>– **The TEXT delta reconciles.** +33,295,754 B = the 18 sites' +32,373,120 + 50 other raised rows' +922,634. 68 rows changed, and none was lowered. RV87's own split gives W +4,040,448 from the 18.<br>– **The maximum reconciles.** RV87 re-derived every phase of both modes from `profile_tree.json` forms × the pinned record's atoms, and each equals the record. W3 dense goes from 3,580,540,218 (part 2) by +4,316,002 (TAV_W; only the TAV forms changed) and +9,771,352 (38 atoms, the closed Estimates) to **3,594,627,572 = 0.8927 M**. Sparse is 0.8878 M |

**What it does to the maximum.**
- SF-1 and SF-2 put W3 dense at 3,595,251,840 B = **0.8929 M**, which is 28,626,816 B under 0.9 M. Sparse is 0.8880 M.
- **The 0.9 M rule holds.**

## Findings

| ID | Sev | Where (at `1e323058f3`) | Evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | `PP/lib.rs:1632` (`reaction@{}`), `:1644` (`spring_action@{}`), `:1708`, `:1721`, `:1739`, and the TPL bound at `:1763` | **The node-DOF label is priced at the integer class, and the table misses it.**<br>– **The placeholder.** `integrity_dof_label(model, dof)` returns `"{node.id}:{UX..RZ}"`, at most 131 B (`lib.rs:1067–1072`). The audit prices its own body (`:1071`).<br>– **The rule.** In `text_args.g4.json`, the int rule (`…\|_dof\|…`, 20 B) matches first, ahead of the composite_id rule that names `integrity_dof_label`.<br>– **Not caught.** The sites are not in the table, and int is not an identifier class, so TEXT stays complete.<br>– **A related inconsistency.** The TPL bound at `:1763` (8,241) was built from these sites, so it is low by 111 B. I65's own `:1817` entry and `:1764` class do assume the label at ≥ 131.<br>– **Δ:** +624,268 B on each of the whole, W and X runs, at the run's convention (`rv87_class_probe_gaps.out.json`) | Price the five placeholders at the label's source bound (131, or composite 600) and raise `:1763` to match. Move named identifier functions ahead of the substring int rule |
| **SF-2** | SHOULD-FIX | `PP/retained_product.rs:3125` (`copy()`'s `out.push_str(s)`, multiplicity 199). Callers: `:3640` (`copy(&r.id)`, a result id) and `:563` (`copy(text)`, the basis text). `text_args.g4.json` `lex_site_size` key `retained_product.rs:2963` | **The adapter copy is priced by the variable's name.**<br>– It is priced at 327 B by the f64 rule `^(v\|s\|t\|…)`.<br>– G4's `lex_site_size` entry for this copy (class result_id) is keyed at the `b1f80234dc` line, 2963. At `1e323058f3`, line 2963 is unrelated code, so the entry silently no longer applies.<br>– **By source,** 2 result-id copies and 1 basis-text copy exceed 327: **+3,334 B**. At G4's intended class (result_id for all 199) it would be +277,406 B.<br>– **Other stale keys.** `site_zero` keys `retained_product.rs:529`, `structural/formation_check.rs:524` and `structural/retained/assemble.rs:15` also match no row at this basis | Re-key or audit the `copy()` site. Make TEXT fail when a site-keyed rule (`site_size`, `lex_site_size`, `site_total`, `site_zero`, `site_from`, `site_aggregate`, `id_audit`) matches no row at the basis |
| **SF-3** | SHOULD-FIX | `_run_records/text_budget.py:335–338` and `:561–566` | **The enforcement fires only when an identifier-class rule matches.** That means `ident`, `ident_debug`, `result_id` or `composite_id`, priced through `arg_max`.<br>– Sites priced by `site_size`, `lex_site_size` or `site_aggregate`, and arguments priced by any other rule, are never checked.<br>– So SF-1 and SF-2 pass, and a table entry whose fallback is a non-identifier class can be removed silently.<br>– **Control c2** removed `loads/primitive_loads/src/lib.rs:299` `actual.schema_ref()` (FMTSTATIC 512). The result was complete=True, TAV −370 B.<br>– **15 of the 735 entries are in this position** by RV87's static count. Most are harmless NUM lengths; the 2 FMTSTATIC entries revert from 512 to 327 | Make the table authoritative for every identifier-bearing candidate. Flag a missing (site, expression) whatever rule would price it, including placeholders that call label or id functions |
| N-1 | NOTE | `PP/lib.rs:2779`, `:5561`, `:5564`, `:7050`, `:7117`, `:7127`; `PP/source_receipt/rows.rs:318`; `PP/source_receipt.rs:815` | **Some identifier copies never enter TAV, and are priced elsewhere.**<br>– The lexicon's `TEXT_NAMES` name filter classes these as data clones or non-literal `.into()`, so they are outside TAV, the table and the enforcement.<br>– Examples: `qualified.clone()`, `ids[&row.id].clone()`, `pipe.from/to.clone()`, `input.behavior.clone()`, `entity.into()`, `case.into()`.<br>– Their bytes land in owners that other families price: row text in O (Text(row), with 4 × 1,024 refs) or T25, and diagnostic refs in Text(diag) at 152 B per ref. So this is not a gap, but the audit's scope statement should say so | Add one line to the audit scope |
| N-2 | NOTE | `ID_CLASS_AUDIT.md` §4 rows 44–54 | **Some table labels are wrong, though the bounds are right.** `suffix = stable_suffix(pipe_id)` is labelled STATIC; it is IN128, with the same 128-B bound. `stable_suffix` is `id.replace(':', "-")`, which keeps the length, and RV87's caller tally confirms that every positive caller passes an input id or a literal path | Relabel |
| N-3 | NOTE | RV87's `u4_g4_02` | **RV87's earlier W figure was an over-estimate.** The W upper for its 18 sites (≤ 5,858,688 B) used whole-run multiplicities. The G6 W run gives +4,040,448 B | None |

## Method

**Reproduction.**
- RV87 re-ran `text_budget.py` (whole, W and X) on the `1e323058f3` snapshot, using G6's `text_args.g4.json`, loop bounds and inventory, and part 2's edges and lexicon. Those inputs are byte-identical between part 2 and G6.
- The settings were `G4_CAPS={"l":128}` and `TB_D=14734`.
- The output is identical to `text_g6/` row for row (`rv87_enforcement_controls.out.json`).

**The by-type probe.**
1. **Every positive-multiplicity expression outside the table: 481 (site, expression) pairs.** Each was listed with the rule class that prices it (`rv87_probe_unaudited.out.json`).
   - Those in large classes (error_display 8,192, message_int and so on) cannot underprice.
   - Every string-typed expression in a small class (int, 20, f64_display, 327, static 512, 64, None) was read with its binding. That is how SF-1 and SF-2 were found.
2. **Typed carriers.** Bindings of `ResultItem`, `Diagnostic`, `LocatedQuantity`, `RowTreatment`, `FunctionalRowBinding`, `Projection` and `Derived` in reached functions were listed. Their id and ref copies were checked against the table.
3. **The lexicon's excluded clones and `.into()`** (235 and 42 receiver classes) were read for identifier Strings (N-1).
4. **The table's own claims were spot-checked:**
   - the `stable_suffix` body and its 41 argument forms;
   - every `entity_ref` assignment in PP (input ids or literals);
   - the `idloc`/`station` labels (producer literals);
   - LIMITATIONS (≤ 242 B, under 327).

**Controls.** Three scratch copies of `text_args.g4.json` each had one entry removed:

| Control | Removed | Result |
|---|---|---|
| c1 | `lib.rs:5592` `matched[0].id` | incomplete, `id-unaudited` |
| c3 | `retained_product.rs:280` `record.id` | incomplete, `id-unaudited` |
| c2 | `primitive_loads/src/lib.rs:299` `actual.schema_ref()` | **complete, −370 B** |

**The maximum.** Every phase of both modes was re-evaluated from `profile_tree.json` (part 2 and G6) with the pinned records' in-build atom values. All equal the records (`rv87_g6_deltas.out.json`).

## Execution

**Who.** RV87, TASK under ROOT.

**Memory guard.** PID 5387 was running at start and at seal. The run wrapper checks it before each run.

**Git reads only,** with `GIT_OPTIONAL_LOCKS=0`.

**Not run.** No Cargo, install, new tooling, solver, native or DEC-025 job. The text runs are the packet's own stdlib scripts, run in scratch.

**Writes.**
- `R/REVIEW_RV87/u4_g6_01/`: this file, SHA256SUMS and `_run_records/`.
- `WT/scratch/rv87_u4_g6_01/`: the work copy, control inputs and outputs.

**Commands** (from `_run_records/`):
- `python3 rv87_g6_deltas.py <G6 _run_records> <part2 _run_records> <u4_g4_02 rv87_s2_confirm.out.json>`;
- `python3 rv87_probe_unaudited.py <G6 _run_records> <part2 text_p2>`;
- `python3 rv87_class_probe_gaps.py <G6 _run_records>`;
- the text runs: `rv87_tb.sh.txt` (`tb.sh <text_args> <out> [variant]`).
