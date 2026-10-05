# RV87 confirmation: the U4 G6 pre-registration repair (SF-1 to SF-3, N-1, N-2)

**Reviewer:** RV87, TASK (Type 2), by ROOT's direction. No descendants.
**Candidate:** `R/I65/u4_g6_01/`: RETURN.md Addendum 2, ID_CLASS_AUDIT.md (795 rows, new §2a) and `_run_records_g6r/`. Its 118-entry SHA256SUMS verifies OK.
- The code is `b43378d90a` on the memory branch.
- The text basis is the `1e323058f3` snapshot.

## Verdict: **CONFIRMED**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 3 |

| Item | Result |
|---|---|
| **SF-1** | **Confirmed.**<br>– The five label sites are now priced at 131 B: `lib.rs:1632` 29 → 140, `:1644` 34 → 145, `:1708` 8,241 → 8,352, `:1721` 83 → 194, `:1739` 88 → 199.<br>– `:1763` is 8,243 → 8,354.<br>– The sixth site, `lib.rs:1149`, is `formation_check_evidence_line`'s `global_dof` label (`check.global_dof.map_or_else(.., integrity_dof_label(..))`, multiplicity 2): 175 → 286, the same +111.<br>– The new `integrity_dof_label\(` rule (131) precedes the int rule |
| **SF-2** | **Confirmed.**<br>– `retained_product.rs:3125` (`copy()`'s `push_str(s)`, multiplicity 199) goes 327 → 1,024, +277,406 B on the whole run.<br>– `lex_site_size` is re-keyed from `:2963` to `:3125`.<br>– The three `site_zero` keys that matched no row are removed. They changed no row; `:529`'s target now sits at `:691` and stays priced |
| **SF-3** | **Confirmed.**<br>– A candidate is any expression that names an id, ref, name, key, label, suffix or identity token; a bare text parameter (`str_param`); or one whose first rule is an identifier class. It must be in the table whatever rule prices it.<br>– `stale-key` and `stale-audit-entry` fail the run.<br>– RV87's four failure controls each fail with their own finding (below). RV87's G6 control `lib.rs:299`, the c1 I65 now reports, is in I65's set |
| **N-1** | **Confirmed.** The audit's scope paragraph names the 8 name-filtered copies and the families that price them |
| **N-2** | **Confirmed, with two label residues** (N-2 below). 28 `suffix` entries are IN128. The bytes are unchanged |
| **The stated residual** | **Acceptable for registration at this basis.**<br>– RV87's by-type sweep of every non-candidate finds no identifier alias.<br>– The mechanism is real (control k5), so it must be carried as a re-qualification obligation (N-1 below) |

**Delta and maximum.** I reproduced I65's figures independently:
- **TAV:** +898,784 (whole), +861,146 (W), +845,812 (X). Eight rows change, all upward, and D/D_env are unchanged.
- **The maximum:** every phase of both modes was re-derived from `profile_tree.json` × the pinned record's atoms, and each equals the record.
  - **Each W phase moves by +861,162 B** (TAV_W +861,146, and `s(ThreadPacketOutput)` 1,688 → 1,704 for S-3's `required` field). Each X phase moves by +845,828 B.
  - **W3 dense: 3,595,488,734 B = 0.8929 M,** 28,389,922 B under 0.9 M. Sparse: 0.8881 M, 48,100,370 B under.

## Controls (RV87's own; `_run_records/rv87_g6r_controls.out.json`)

**Baseline.** The repaired TEXT was re-run (whole, W and X), and every row is identical to `text_g6r/`. Each control then changes one input in scratch:

| | Change | Result |
|---|---|---|
| k1 | Remove the TPLLABEL entry at `lib.rs:1149` (the site the new enforcement found) | incomplete, `id-unaudited` |
| k2 | Remove `s` at `retained_product.rs:3152` (STATIC 327; its fallback rule is the non-identifier f64 class, the case that was silent at G6) | incomplete, `id-unaudited` |
| k3 | Move `lib.rs:1721`'s audit entry to `:1722` (a drifted key) | incomplete: `id-unaudited` at :1721, `stale-key` and `stale-audit-entry` at :1722 |
| k4 | Add a `lex_site_size` key at `retained_product.rs:3126`, a line with no lexicon row | incomplete, `stale-key` |
| **k5** | **The residual.** `lib.rs:1721`'s inventory row is given the alias `end`, as a new site written that way would be, with no table entry | **complete, TAV −85,248 B.** The alias is priced at the int rule's 20 B and nothing flags it |

## The residual: why it is acceptable now

**Nothing in the current code hides behind it.**
- RV87 instrumented a scratch copy of the repaired `text_budget.py` (`rv87_text_budget_instr.py.txt`), confirmed it reproduces the run identically, and dumped every positive-multiplicity (site, expression) that the predicate does **not** treat as identifier-bearing. That is 410 pairs (`rv87_g6r_noncandidates.out.json`).
- The ones in large classes (error_display, joined_list, message_int, and so on) cannot underprice.
- RV87 read every string-typed one in the small classes (int, 20, 64, 30, 0, 128, f64_display, 327, static 512, None). Each is one of:
  - a literal or literal table;
  - a `&'static str` constant (policy, schema, profile, contract ids);
  - a numeric;
  - a digest;
  - a unit or dimension symbol;
  - DOF-class text;
  - a join priced by `join_rules`;
  - the `located` path (≤ 9 keys).
- **None is an alias of a result, diagnostic, case, member, support, node, load or source id.**

**The code basis adds no text.** Between the text basis `1e323058f3` and `b43378d90a`, the non-test code adds 1,494 lines and **none** is a `format!`, `to_string`, `to_owned`, `String::from`, `.clone()`, `push_str`, `join`, `replace`, `write!` or `collect::<String>`. So TEXT at `1e323058f3` is the registered code's TEXT.

**The mechanism is still real.** k5 shows that a future site holding an identifier under a token-less local name, priced by a non-identifier rule, would pass silently. That makes it a re-qualification risk, not a present gap. The same-line key drift needs a code edit to arise.

**RV87's recommendation:**
- Register at this basis.
- Bind the residual to the re-qualification obligation: any code change re-runs TEXT, and repeats this non-candidate review or closes it.
- The cheap closure: require an explicit table row (e.g. NOTID or STATIC) for every bare-local placeholder or receiver that a non-identifier class prices. That inverts the default for exactly the k5 shape. Lex keys could also carry the expression as well as the line.

## Notes

| ID | Where | Evidence | Remedy |
|---|---|---|---|
| N-1 | The audit's stated residual; control k5 | **The residual is acceptable at this basis.** A by-type sweep of all 410 non-candidates finds no identifier alias, and the code basis adds no text sites. The mechanism is real (k5: complete, −85,248 B) | Record it as a re-qualification obligation in QUALIFICATION/the D-6 record. Close it at the next re-qualification with the explicit-row rule above |
| N-2 | `retained_product.rs:2332` and `:2351` audit entries | **Two `suffix` rows keep the old label.** The `suffix` there is `stable_suffix(&row.entity_ref)` (`:2223`), an input-derived id, but it is still labelled STATIC. The bound (128) and bytes are unchanged. The reader's `suffix` (`retained_precision.rs:211`, `:213`, `fn error(.., suffix: &str)` with literal codes) is correctly STATIC | Relabel IN128 |
| N-3 | `text_args.g4.json` id_audit key at `lib.rs:1149` | **One audit key embeds a code comment, so editing the comment fails the run.** The key is the inventory's raw placeholder text, including a `//` comment. Editing the comment fails closed (`stale-audit-entry`), which is safe | Optionally strip comments in `audit_key` |

## Execution

**Who.** RV87, TASK under ROOT.

**Memory guard.** PID 5387 was running at start and at seal, and the run wrapper checks it before every run.

**Git reads only,** with `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `diff`, `show`.

**Not run.** No Cargo, install, new tooling, solver, native or DEC-025 job. The text runs are the packet's stdlib scripts, run in scratch.

**Writes.**
- `R/REVIEW_RV87/u4_g6_02/`: this file, SHA256SUMS and `_run_records/`.
- `WT/scratch/rv87_u4_g6_02/`.

**Commands** (from `_run_records/`):
- `python3 rv87_g6r_rowdiff.py <G6 _run_records> <_run_records_g6r>`;
- `python3 rv87_g6r_inbuild.py <G6 _run_records> <_run_records_g6r>`;
- the runs and controls: `rv87_tb.sh.txt` (`tb.sh <text_args> <out> [variant]`, `TB_INV`/`TB_PY` overrides);
- the instrumentation: `rv87_text_budget_instr.py.txt`, run with `RV87_NONC=<out>`.
