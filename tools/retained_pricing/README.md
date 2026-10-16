# Retained pricing

This tool prices the retained route's memory at a revision and generates the GENERATED PROFILE block in `core/product_physics/src/retained_memory.rs`. Registration separately binds the actual build, reviewed inputs, layouts and selected threshold.

The base chain's `g4_caps.py` keeps the historical illustrative `M = 4_026_531_840`: `tree.M`, `fraction_of_M`, `margin_rule_0.9M` and `fits_M` use illustrative layout strides and are not native budget qualification. `run` exit 0 establishes pricing completeness. `generate` replaces only the generated block and preserves `REGISTERED_PROFILES.threshold_bytes`; it does not select or register M. `eval` (`lib/tree_eval.py`) uses supplied native atoms to evaluate forms/phases without applying a budget. Current qualification uses the source-selected registered threshold and the native law tests' checks of all phases, R, the 0.9 margin and the upward-rounded 5% TAV_W/TAV_X reserve.

It is SQ2's TEXT and cap-priced chain under the O-10 rules, in one Python command (standard library only). The reasons for the rules it adds by name are in DEL-05-02's [retained-pricing-rules.md](../../execution/PKG-05/DEL-05-02/Design/retained-pricing-rules.md). Use this tool, not earlier copies of the chain.

## Use

Run from any directory:

```bash
P=projects/chirality-piping/tools/retained_pricing
python3 $P/price.py run <rev> --out <new dir outside any checkout> # price a revision
python3 $P/price.py compare <tree A> <tree B>                    # coefficient-wise comparison
python3 $P/price.py generate <tree> <retained_memory.rs>         # rewrite the GENERATED PROFILE block
python3 $P/price.py generate <tree> <retained_memory.rs> --check # compare instead of writing
python3 $P/price.py eval <law-test log> <tree>                   # phases at in-build atom values
```

### Reading a profile change

1. **Run the new head.** `run <head>` should complete on all four points with no unmapped loop header and no unclassified argument, and both o10 checks should pass.
2. **Review the prune.** `summary.json`'s `linemap.pruned` lists every rule the line map dropped, each with its rule. Each one must belong to a function or line the head really deleted. A dropped `site_from`, `site_total`, `edge_per_call` or `loop_total` rule removes a bound, so read the code. The deleted test is per file: a function moved to another file shows up as deleted, and its rules must be re-keyed to the new file rather than dropped. `linemap.resolutions_used` lists the reviewed resolutions that applied.
3. **Compare.** Use `compare <new tree_merged.json> <registered tree>`.
   - `a_le_b` true (every coefficient no higher) means the registered block still bounds the head, so nothing needs re-registering.
   - Otherwise, regenerate the block with `generate` and re-pin the law tests whose values moved. The law tests are `cargo test --lib profile_` in `core/product_physics`. Their `I65_G5_ATOM` lines feed `eval`.
4. **Check the block.** `generate --check` confirms that a committed block is the generator's output for a tree.

### The basis line

`--basis` replaces the tree's basis line.

- **Plain text:** written after "corrections at", followed by a period.
- **Text starting with ",":** continues the sentence after "corrections," and is wrapped at 100 columns. Give the final period yourself.

For example, main's block at `2c5cf0823b` is the registered tree with:

```
--basis ", re-run as SQ2 G5 at f78f52b7e2: the maximum over routes L (C_eq <= 3, worst shape c = 3, z = 0) and E (c = 3), under SQ2's rules; text at l <= 128."
```

## Inputs

**Repository.** A clone with full history that contains the basis `2243be380b`; a shallow clone is refused. The default is this checkout. `--repo` or `RP_REPO` selects another.

**Revision.** Any revision whose code the rules cover.

**Tools.** Python 3.9 or later, `git` and `tar`. The chain's scripts call `python3`; the driver puts its own interpreter first on `PATH`.

**Optional environment.**
- `RP_OUT`: the default for `--out`.
- `RP_JOB_WRAPPER`: a command that each point runs under (split like shell words), such as a host's job-slot or memory-guard wrapper. Nothing host-specific is built in.

## Outputs

All outputs go to `--out`.
- It must not lie inside any git work tree; the tool looks for a `.git` entry in it and every parent.
- It must be new, empty, or a directory this tool made before (it holds the marker `.retained_pricing_out`). Before a run the tool removes only its own entries, those listed in the table below.
- The output directory is not a record; delete it when done.

| Path | Content |
|---|---|
| `tree_merged.json` | The profile tree: the coefficient-wise maximum of route L at (3,0) and route E. Written only when every point passes and both o10 checks hold |
| `summary.json` | The line map (remapped, unmapped, pruned), the premise pins, each point's completeness, D, TAV, sparse and dense E+R and TAV_W/TAV_X, the o10 checks and the merge |
| `runs/{L_30,L_21,L_12,E_30}/` | Each point's call graph, lexicon, chain work directory (`work/text_budget.caps.out.json`, `work/profile_tree.json`), `summary.json` and `looplog.json` |
| `map/`, `chain/`, `rcL/`, `rcE/` | The carried rule tables and line-map report, the base chain at the revision, and each route's chain (`SQ2_RULES.json` logs every rule applied and the decision-key map) |
| `o10_check_{L,E}.json`, `logs/` | The o10 checks and step logs |

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Complete |
| 2 | Bad input |
| 3 | An o10 check failed; no tree is written |
| 4 | Stopped before the points: a rule or decision key not carried, a premise pin not as reviewed, or a template inventory or rules step failed |
| 5 | A point failed its gate and no tree is written. A point passes only when `sens.py` ran clean, `complete` and `text_complete` (all four TEXT variants) hold, `D_converged` holds, and it has no unmapped header and no unclassified argument |
| 6 | Merge or generator failure, or `generate --check` found a difference |

## Pipeline

1. `git archive` the revision's `projects/chirality-piping`, without `execution/`.
2. Carry the base chain's line-keyed rules (`base/*.g4.json`, `base/sens.py`, `base/premise_pins.json`) from the basis to the revision with `lib/linemap_rules.py --prune-deleted`.
   - **Keymap.** `lib/linemap_lib.py` maps each key in order:
     1. **Offset:** by the cumulative offset, checked.
     2. **Function:** by the same ordinal `fn` definition.
     3. **Text:** by the line's text.
     A key that none of these carries stops the run.
   - **Prune.** A key is pruned only when the revision deleted its line, or deleted its function so that no `fn name` is left in the file. The key's rule is dropped, including list-form rules, and the drop is reported.
   - **Fewer definitions.** Sometimes a name's definition count falls but stays above zero, as when one of two `fn f` is removed. The count alone does not say which definition went, so the key is `fewer_definitions` and the run stops. A reviewed entry in `rules/linemap_resolutions.json` resolves it, keyed as written at the basis:
     - `{"as": "deleted", "why": …}` prunes it;
     - `{"as": n, "why": …}` maps it to the head's n-th definition.

     The decision keys use the same resolutions.
   - **Replaced site in a deleted function.** A line key whose line a hunk replaced stops the run. A reviewed entry `{"as": "deleted", "fn": name, "why": …}` prunes it only when `name` is the key's enclosing function at the basis and the revision's file no longer defines `fn name`; a moved function still stops.
3. Check the premise pins (the warrant lines of dead branches) against the snapshot's text.
4. Regenerate the template inventory on the snapshot.
5. For each route, apply the decisions with `lib/sq2_rules.py` and `rules/sq2_decisions_{L,E}.py`.
   - The decisions include the shared `rules/sq2_decisions_common.py`. Its last step executes `rules/o10_rules.py`.
   - Keys written at the basis are carried by the same map. Keys resolved by name at the snapshot (`rs`, `rs_any`, `fnkey`, `site`) are used as resolved.
   - An uncarried key stops the run unless the decisions' `KEYMAP_ALLOW` lists it as `("deleted", why)` and the map reported it deleted. A replaced key is never allowed (`lib/rule_ops.py`).
6. Run the points. Route L runs at (lc, z) = (3,0), (2,1) and (1,2); route E runs at 3. Caps are `l = 128` and `c = 3`. Each point writes its loop log (`lib/mk_looplog.py`).
7. Run `lib/o10_check.py`. Each appended loop rule may match only its listed `file:fn` functions, and each multiplicity-0 site must stay at 0. If a check fails, the run ends with exit 3 and no tree.
8. Merge: `lib/merge_trees.py` takes route L at (3,0), SQ2's worst shape, and route E. Route L's other shapes are priced so they can be compared. Their phases at in-build values come from `eval <law-test log> runs/L_21/work/profile_tree.json`, and they should stay below (3,0)'s; the tool does not check this itself.

## Layout and provenance

**`base/`** is SQ2's base chain at the basis `2243be380b`.
- **Source:** I104's chain scripts, at `e85d383b64:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I104/b1_sq_01/_run_records/chain/`.
- **N-5 adapter rows:** applied in `producer_caps.py` and `g5_profile.py` by that record's `g6/tools/sq_n5_chain.py`.
- **Rule tables and keys:** the rule tables, `sens.py`'s keys and I65's premise pins were carried from `57c92a7b33` to `2243be380b` by RV132's line map. This is the base SQ2 was qualified on.
- **Historical carry check:** before K-D5's name-key migrations, carrying this base at `f78f52b7e2`, `2c5cf0823b` and `e9cc7e3f5f` gave rule tables byte-identical to the earlier two-hop carry from `57c92a7b33`. K-D5 moves reviewed changed sites and the condition-estimator call into name-keyed rules; the registered batch-3 tree still reproduces with the same 630 coefficients.
- **Current-source model:** K-D5's owned-layout and unified-text additions apply only when the snapshot defines `MemberBoundInputs`. Older priced revisions keep their original formulas and illustrative-layout metadata. The positive owners and domain-grounded empty children are explained in the governed pricing rules.
- **Batch 5 source binding:** `batch5_model.py` pins the pressure production prefix, preparation/solve consumers and actual D1 refusal bodies before adding positive carrier and staging charges. Its inactive branch requires the exact registered `e0ec460bf3` pressure prefix and case body; helper-name absence alone never selects historical pricing. Batch 5's six illustrative atom entries are added only on its source branch, preserving the old registered tree's bytes as well as all 689 coefficients. Actual native layouts, rather than illustrative values, qualify the new profile.
- **Generator change:** `g5_profile.py` also takes the wrapped basis line described under "The basis line".
- **T4-U6/U8 owners:** `u6_owners_model.py` finds, by their exact source text, the two hash-wrapper forms (U6-DH, U6-SH) and the D-5/D-6 resident owners that `ordinary_caps.py`, `t25_g4.py` and `g4_caps.py` then price (U6-ES, U6-WS, U6-RS, U8-FM). Older sources add nothing.
- **Not included:** the chain's patch logs and its stale template inventory, because a run regenerates the inventory.

**`rules/`** holds the decision tables: SQ2's route rules and the O-10 name-keyed rules.
- `i95_new_loops.json` lists I95's B3-S rebinding loops for route E.
- `linemap_resolutions.json` holds the reviewed resolutions. One `fewer_definitions` entry is the frame kernel's `UserStiffnessElement::orientation`, deleted by T4-U3; at `666cb30520` it reproduces O-10's tree. One site entry is `load_reference_source.rs:47`, inside the private `fn code` B7b deleted with the v2 readers.

**`lib/`** is the machinery.
- `linemap_lib.py`, `linemap_rules.py`, `sq2_rules.py`, `rule_ops.py`, `o10_check.py`, `merge_trees.py`, `tree_eval.py` and `census_patch.py` are T3's O-10 tools.
- `census_exact.py` takes route E's census by I95's method on the pressure-source-1 producer fixtures (U6-RP): `python3 census_exact.py <projects/chirality-piping dir> <out json>`.
- `mk_looplog.py` is vendored unchanged from the I104 record above (`tools/mk_looplog.py`).

**`data/`** holds three files.
- `basis_text_rows.json` lists the (file, line) of the basis runs' TEXT rows. The line map uses it only to choose among files that a short key could name.
- `census_c3cf.json` is I95's exact-evidence census, used for route E's per-case evidence forms before D-5.
- `census_d5_20c6126a16.json` is the D-5 census (72 rows per member and the per-basis evidence), used on D-5 source.

## Limits

- **Code coverage.** The rules cover the code they were written for. A new loop, an unclassified argument, or a removed function that a decision names makes the run stop or report incomplete. It does not guess.
- **Basis-relative keys.** Decision keys are written at the basis. A long-moved file can need a key re-expressed by name (`fnkey` or `site` in `o10_rules.py`).
- **Fixed points.** The points and caps are D1's: C_eq ≤ 3 and l ≤ 128. Other shapes need a code change.
- **Atom values.** Values come from the in-build law test. The tree's ASSUMED strides are only the chain's own check.
- **Runtime and CI.** A full run takes about one minute of CPU on one core. The full run is not in CI. `tests/test_retained_pricing_tool.py` runs for any change under this directory and covers:
  - the key carry, the prune, and the `fewer_definitions` stop and its resolutions, on two-commit fixture repositories;
  - the decision-key stop and `KEYMAP_ALLOW`;
  - the `EDGE_ZERO_REMOVE_CALLER` count;
  - the generator's basis line.

## Adding an adjudication rule

Add a rule when a run stops or reports incomplete on a new head.

1. Read the reason in `runs/*/work/text_budget.caps.out.json` (`unmapped_loop_headers`, `unclassified_args`) or in the stop message.
2. Write the argument in DEL-05-02's `Design/retained-pricing-rules.md` under a new label. State the code facts it rests on, the bound or class, and why the bound holds on the D1 graph. A zero needs a premise, not an observation.
3. Add the rule to `rules/o10_rules.py` under the same label, keyed by name: `fnkey(name, file)` or `site(fn, file, text)`, never a bare line number.
   - Guard it with `if fnkey(...)` so that heads without the function add nothing.
   - Use the existing families: `O10_LOOPS_LAST` (exact header, bound, why, the only `file:fn` it may match), `O10_LOOPS_FIRST` (U6-LF: the same, placed before the earlier broad rules for an exact header one of them would otherwise capture), `O10_EDGE_ZERO`, `O10_SITE_FROM`, `O10_SITE_SIZE`, `O10_ID_AUDIT`, `O10_ARGS_FIRST`, `O10_DIAG_EXCLUSIVE` (B7b-DX: diagnostic sites of one function of which a call pushes at most one; `text_budget.py` counts the group once per call at its largest member and leaves the run incomplete unless every member is an unlooped site of that function with its multiplicity), `O10_MULT0_SITES` (B7b-JR: a site whose own multiplicity must stay 0, with no size or zero rule; `o10_check` fails the run otherwise) and `O10_TYPED_PATCHES` (U6-RB: a route's exact-text substitution in one chain file, with its reason).
   - For a site you claim has multiplicity 0, prefer `O10_SITE_SIZE` 0, which `o10_check` guards, over a zero.
4. If a function the decisions name was renamed or deleted, change only the affected entry and leave its argument unchanged. Use `rs_any` for a rename, or a `KEYMAP_ALLOW` entry `("deleted", why)` for a deletion.
5. Re-run the head and the registered revision. The registered tree must still reproduce exactly. The KD5 source pins in `base/kd5_text_model.py` and `base/kd5_first_error_model.py`, and the source-bound closure pin in `rules/o10_rules.py`, hold one current value each, so the committed tool stops (exit 4) on the registered revision once a pin moves; reproduce it with a scratch copy of the tool whose pins are restored to the registered revision's values, with the new rules kept. Restore the original case consumer in place of the batch 5 wrapper/helper pair when reproducing that earlier source. Then compare against the registered tree as above. The B7a source models' full-file pins (`base/b7a_*.py`, `lib/b7a_retired_carry.py`, `base/batch5_model.py`) and C7's NumericCase pins (`base/prescribed_key_model.py`) work the same way. The committed tool's pins hold T4-U6/U8's priced source (`433fc1f1e4`); the registered tree, B7b's at `837a04b770`, reproduces with those files restored from the tool at `d8f98fe60c` and the two embedded pins (KD5's `evaluate`, C7's) set back (retained-pricing-rules.md, "T4-U6/U8: actual-source D3").
