# RV87 on U4 G7 Pass A: the TEXT-tool cycle fix, and the §11 discharge

**Reviewer:** RV87, TASK (Type 2), by ROOT's direction. No descendants.
**Candidate:** `R/I65/u4_g7_01/`. Its SHA256SUMS is 78/78 OK; RETURN.md is `074d4b93…`, QUALIFICATION_G7.md `37b83fc1…`.
**The basis:** the merge tree `ba1faa1c858ce3630a22767677310b1902a14b83`. My own copy is a `git archive` of that tree under `WT/rv87_g7/basis`, and it equals I65's extract (`diff -rq` of `core/` is empty).
**Scope:** the cycle fix and §11 only. RV89 owns the inventory, identity, F5's pricing and the Pass B script.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 4 |

| Question | Result |
|---|---|
| **1a.** G6's call graph had no multi-function cycle, so G6's registered numbers were never affected | **Confirmed.** My own iterative Tarjan (`rv87_scc.py`) finds **no multi-function SCC** on the reached graph, with or without the `edge_zero` cuts, on both graphs:<br>– G4's graph: 2,698 functions reached;<br>– G6's graph: part 2's `edges_p2.json`, 2,752 reached. I reproduced G6's TEXT byte for byte on it, so it is G6's effective graph.<br>The same 11 self-recursive text ancestors appear at G6 and G7 |
| **1b.** Cutting the `edge_zero` edges first is sound, and each cut is justified on D1 | **Confirmed.**<br>– **Soundness.** Propagation already skipped `edge_zero` edges, and they carry no call. Removing them before the condensation changes no live path. It only stops a dead edge from fusing live functions into one component, whose non-entry members would then get 0. `anc` still includes the cut edges, which is conservative.<br>– **Each cut checked against the source.** All 6 new cuts sit only inside the `is_retained(source)` branches: `for_source` → `validate`, `preview_physics_retained_contract` and `retained_error` (`semantic_contract.rs:438–443`), and `for_source_metadata` → `validate_transport_metadata`, `preview_physics_retained_contract` and `retained_error` (`:270–274`).<br>– **Why the branches are dead.** On the reached graph, `for_source`'s only caller is `retained_precision::validate` (G7, `:4305`), and it passes `project(source, true)`. `project` sets `producer.semantic_contract_id` to the literal preview-physics-1 id unconditionally (`:4252–4253`), so `is_retained` is false. `for_source_metadata` is reached only from `for_source`'s base branch on that same projection, or from the dead `validate_transport_metadata`.<br>– **The new base-branch checks** (`forbid_retained_rows`, `forbid_retained_member`) allocate only on an error path, which the projection rules out |
| **1c.** The fail-closed check catches every cycle that could reintroduce silent zeroing | **Confirmed for the graph as built.** Silent zeroing needs a multi-function component in the graph, and the run now fails on any such component among text ancestors.<br>**Four cycle controls** (`rv87_g7_text_runs.out.json`):<br>– **q1**, `for_source` → `validate` restored: `scc`, −112,852,224 B;<br>– **q2**, the metadata cut restored: `scc`;<br>– **q3**, a synthetic back edge from `validate_preview_physics_evidence` to `for_source`: `scc` on the G7 tool, while **the G6 tool reports complete=True at −112,852,224 B**.<br>**Not seen by the check:** cycles hidden by dispatch the lexical graph misses. That is the call graph's stated limit (G4 R-1). The U6 delta adds no `dyn`, `Fn` or fn-pointer dispatch: its closures are inline, and `map_err(retained_error)`'s fn item is an edge. Self-recursion is outside the check (N-1) |
| **1d.** The 7 new rows have multiplicity 0 | **Confirmed**, by a line-independent comparison per (file, kind, fn) of the positive-multiplicity multisets (`rv87_rows_multiset*.out.json`). The positive groups are identical to G6's in all four runs. The 7 extra rows, all multiplicity 0, are:<br>– `derivative.rs` `class_disclosure` and `not_covered_message`, which are unreached;<br>– `validate_transport_metadata`'s `into_text`, below a cut;<br>– 4 rows in `verify_preview_physics_retained_table`, below a cut |
| **2.** §11: the 410 are the same set, and U6 adds no alias | **Confirmed.**<br>– My own G7 run's non-candidate dump equals I65's `noncandidates_g7.json` exactly, and equals my G6r list of 410 key for key: (file, fn, expression, kind/spec, class, multiplicity). The only difference is a trailing space where my G6r dump truncated an expression.<br>– There is no new positive-multiplicity non-candidate, so U6 adds no identifier-class alias. Every row is one I read at G6r.<br>– **Deferring the explicit-row rule is acceptable for this merge,** subject to SF-1 |

**Reproduction.** I regenerated the G7 call graph, lexicon and inventory with the G7 chain scripts on my copy.
- **Inputs:** edges `49e74ab2…`, lexicon `767ace8b…` and inventory `d705ed42…`. The first two equal I65's `p/` and `pass_selftest` copies byte for byte, and the inventory equals `pass_a/template_inventory_head.out.json`.
- **The G7 tool on them** reproduces `pass_a/text_g7/` whole, W, X and env, row for row: 2,150,800,830 / 1,570,041,862 / 1,440,401,002 / 642,949,448.
- **The G7 tool on G6's final inputs** reproduces G6r's four outputs row for row.
- **The G6 tool on the G7 inputs** returns complete=True with TAV 2,037,948,606, which is −112,852,224 B (`preview_physics_evidence.rs` −110,776,988; `semantic_contract.rs` −2,073,808; …). That confirms the silent-zeroing defect.

**My independent model.** The G4/G6 composition still holds, because every input it reads is unchanged. Every phase of G7's pinned record re-derives exactly from G7's `profile_tree.json` × the record's in-build atoms (`rv87_g7_inbuild.out.json`).
- Each phase equals G6r's: Δ 0.
- Only `T17_V4`'s form changed (F5). No atom moved.
- V4 is 17,172,765 B, against V2_hash at 1,148,998,726.
- W3 is 0.8881 / 0.8929 M, which is 48,100,370 / 28,389,922 B under 0.9 M.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | `_run_records/g7_pass.sh:62–64` (`set -u` only); QUALIFICATION_G7.md §4.2 ("Pass B … **stops** on any new non-candidate") | **Pass B reports these conditions but does not stop on them.**<br>– `noncand_compare.py` exits 1 on a new non-candidate, but Pass B ignores that and only prints the verdict.<br>– A TEXT run that is incomplete, including an `scc`, likewise appears only in the summary line.<br>– Pass B exits 0 ("read the summary") in both cases.<br>– §11's deferral of the explicit-row rule rests on that comparison being fail-closed | Make Pass B exit non-zero (as with 3 and 4) when `noncand_compare` fails or any TEXT run is incomplete. Or correct §4.2 and require the summary's verdicts before reliance. Coordinate with RV89, who reviews Pass B |
| N-1 | NOTE | `chain/text_budget.py:695–698` (`SELF_RECURSIVE` is informational) | **Self-recursion is not covered by the fail-closed check.**<br>– Single-function recursion zeroes nothing, but a self-recursive text ancestor's sites are counted once per external call. Only `recursion_factor` (2 functions) scales them.<br>– At this basis, the 11 self-recursive text ancestors equal G6's, and their text is priced elsewhere: error paths fire once, `located`'s per-object copies are in T17's OBJ_WALK, and `write_canonical` is in the hash route | Fail the run on a self-recursive text ancestor outside an approved list of 11 (or without a `recursion_factor`), as `scc` does for cycles |
| N-2 | NOTE | `chain/text_budget.py` scc check | **The check's completeness equals the call graph's completeness.** A cycle through dispatch the lexical graph misses cannot be seen. U6 adds none, as checked above | State it in QUALIFICATION_G7 §4.1 |
| N-3 | NOTE | G4–G6 TEXT | **No earlier result is affected.** The G6 tool zeroes silently on any in-graph cycle (control q3). G4's and G6's graphs were cycle-free (1a), so no registered or reviewed number was affected. G7's fix is necessary for any future basis | None |
| N-4 | NOTE | `pass_a/proposal/t17_v4_f5.diff` | **F5 is in the record's tree but not in the committed profile.** G7's `profile_tree.json` carries F5 in `T17_V4`; the committed Rust profile does not (proposal not applied). No phase, record or maximum moves, because V4 is about 1.13 GB below V2_hash | RV89's item. Apply it at Pass B or later |

## Execution

**Who.** RV87, TASK under ROOT.

**Memory guard.** PID 5387 was running at start and at seal, and the run wrapper checks it before every run.

**Git reads only,** with `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `cat-file`, `diff`, `archive` of the basis tree.

**Not run.** No Cargo was needed or run. No installs; no native, solver-at-scale or DEC-025 jobs; nothing in the system temp directory.

**Writes.**
- `WT/rv87_g7/`: the basis copy and chain scripts.
- `WT/scratch/rv87_u4_g7_01/`: the regenerated inputs, run outputs and control inputs.
- `R/REVIEW_RV87/u4_g7_01/`.

**Not touched:** WT/f2a-memory and I65's scratch, which was only read for hash comparison.

**Commands** (from `_run_records/`):
- `python3 rv87_scc.py <edges> <loop_bounds> <inventory> <lexicon>`;
- `python3 rv87_rows_multiset.py <G6r run> <G7 run>`;
- `python3 rv87_noncand_compare.py <u4_g6_02 non-candidates> <G7 dump> <I65 dump>`;
- `python3 rv87_g7_inbuild.py <_run_records_g6r> <pass_a>`;
- the runs: `rv87_tb7.sh.txt` (`tb7.sh <text_budget.py> <text_args> <loop_bounds> <out> [variant]`).
