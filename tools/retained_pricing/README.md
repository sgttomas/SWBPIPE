# Retained pricing

This tool prices the retained route's memory at a revision. It also writes the registered profile, the GENERATED PROFILE block in `core/product_physics/src/retained_memory.rs`.

It is SQ2's TEXT and cap-priced chain under the O-10 rules, in one Python command (standard library only). The reasons for the rules it adds by name are in DEL-05-02's [retained-pricing-rules.md](../../execution/PKG-05/DEL-05-02/Design/retained-pricing-rules.md). Use this tool, not earlier copies of the chain.

## Use

Run from any directory:

```bash
P=projects/chirality-piping/tools/retained_pricing
python3 $P/price.py run <rev> --out <dir outside the checkout>   # price a revision
python3 $P/price.py compare <tree A> <tree B>                    # coefficient-wise comparison
python3 $P/price.py generate <tree> <retained_memory.rs>         # rewrite the GENERATED PROFILE block
python3 $P/price.py generate <tree> <retained_memory.rs> --check # compare instead of writing
python3 $P/price.py eval <law-test log> <tree>                   # phases at in-build atom values
```

### Reading a profile change

1. **Run the new head.** `run <head>` should complete on all four points with no unmapped loop header and no unclassified argument, and both o10 checks should pass.
2. **Compare.** Use `compare <new tree_merged.json> <registered tree>`.
   - `a_le_b` true (every coefficient no higher) means the registered block still bounds the head, so nothing needs re-registering.
   - Otherwise, regenerate the block with `generate` and re-pin the law tests whose values moved. The law tests are `cargo test --lib profile_` in `core/product_physics`. Their `I65_G5_ATOM` lines feed `eval`.
3. **Check the block.** `generate --check` confirms that a committed block is the generator's output for a tree.

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

All outputs go to `--out`, which must be outside the checkout. The output directory is not a record; delete it when done.

| Path | Content |
|---|---|
| `tree_merged.json` | The profile tree: the coefficient-wise maximum of route L at (3,0) and route E |
| `summary.json` | The line map (remapped, unmapped, pruned), the premise pins, each point's completeness, D, TAV, sparse and dense E+R and TAV_W/TAV_X, the o10 checks and the merge |
| `runs/{L_30,L_21,L_12,E_30}/` | Each point's call graph, lexicon, chain work directory (`work/text_budget.caps.out.json`, `work/profile_tree.json`), `summary.json` and `looplog.json` |
| `map/`, `chain/`, `rcL/`, `rcE/` | The carried rule tables and line-map report, the base chain at the revision, and each route's chain (`SQ2_RULES.json` logs every rule applied and the decision-key map) |
| `o10_check_{L,E}.json`, `logs/` | The o10 checks and step logs |

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Complete |
| 2 | Bad input |
| 3 | An o10 check failed |
| 4 | Stopped before the points: a rule or decision key not carried, a premise pin not as reviewed, or a template inventory or rules step failed |
| 5 | A point is incomplete: unmapped header, unclassified argument, or chain error |
| 6 | Merge or generator failure, or `generate --check` found a difference |

## Pipeline

1. `git archive` the revision's `projects/chirality-piping`, without `execution/`.
2. Carry the base chain's line-keyed rules (`base/*.g4.json`, `base/sens.py`, `base/premise_pins.json`) from the basis to the revision with `lib/linemap_rules.py --prune-deleted`.
   - **Keymap.** `lib/linemap_lib.py` maps each key in order:
     1. **Offset:** by the cumulative offset, checked.
     2. **Function:** by the same ordinal `fn` definition.
     3. **Text:** by the line's text.
     A key that none of these carries stops the run.
   - **Prune.** A key whose function or line the revision deleted is dropped with its rule, including list-form rules, and the drop is reported.
3. Check the premise pins (the warrant lines of dead branches) against the snapshot's text.
4. Regenerate the template inventory on the snapshot.
5. For each route, apply the decisions with `lib/sq2_rules.py` and `rules/sq2_decisions_{L,E}.py`.
   - The decisions include the shared `rules/sq2_decisions_common.py`. Its last step executes `rules/o10_rules.py`.
   - Keys written at the basis are carried by the same map. Keys resolved by name at the snapshot (`rs`, `rs_any`, `fnkey`, `site`) are used as resolved.
   - An uncarried key stops the run unless the decisions' `KEYMAP_ALLOW` gives a reason.
6. Run the points. Route L runs at (lc, z) = (3,0), (2,1) and (1,2); route E runs at 3. Caps are `l = 128` and `c = 3`. Each point writes its loop log (`lib/mk_looplog.py`).
7. Run `lib/o10_check.py`. Each appended loop rule may match only its listed `file:fn` functions, and each multiplicity-0 site must stay at 0.
8. Merge: `lib/merge_trees.py` takes route L at (3,0), SQ2's worst shape, and route E. Route L's other shapes are priced so they can be compared. Their phases at in-build values come from `eval <law-test log> runs/L_21/work/profile_tree.json`, and they should stay below (3,0)'s; the tool does not check this itself.

## Layout and provenance

**`base/`** is SQ2's base chain at the basis `2243be380b`.
- **Source:** I104's chain scripts, at `e85d383b64:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I104/b1_sq_01/_run_records/chain/`.
- **N-5 adapter rows:** applied in `producer_caps.py` and `g5_profile.py` by that record's `g6/tools/sq_n5_chain.py`.
- **Rule tables and keys:** the rule tables, `sens.py`'s keys and I65's premise pins were carried from `57c92a7b33` to `2243be380b` by RV132's line map. This is the base SQ2 was qualified on.
- **Check:** on `f78f52b7e2`, `2c5cf0823b` and `e9cc7e3f5f`, carrying from this base gives rule tables byte-identical to the earlier two-hop carry from `57c92a7b33`.
- **Generator change:** `g5_profile.py` also takes the wrapped basis line described under "The basis line".
- **Not included:** the chain's patch logs and its stale template inventory, because a run regenerates the inventory.

**`rules/`** holds the decision tables: SQ2's route rules and the O-10 name-keyed rules. `i95_new_loops.json` lists I95's B3-S rebinding loops for route E.

**`lib/`** is the machinery.
- `linemap_lib.py`, `linemap_rules.py`, `sq2_rules.py`, `o10_check.py`, `merge_trees.py`, `tree_eval.py` and `census_patch.py` are T3's O-10 tools.
- `mk_looplog.py` is vendored unchanged from the I104 record above (`tools/mk_looplog.py`).

**`data/`** holds two files.
- `basis_text_rows.json` lists the (file, line) of the basis runs' TEXT rows. The line map uses it only to choose among files that a short key could name.
- `census_c3cf.json` is I95's exact-evidence census, used for route E's per-case evidence forms.

## Limits

- **Code coverage.** The rules cover the code they were written for. A new loop, an unclassified argument, or a removed function that a decision names makes the run stop or report incomplete. It does not guess.
- **Basis-relative keys.** Decision keys are written at the basis. A long-moved file can need a key re-expressed by name (`fnkey` or `site` in `o10_rules.py`).
- **Fixed points.** The points and caps are D1's: C_eq ≤ 3 and l ≤ 128. Other shapes need a code change.
- **Atom values.** Values come from the in-build law test. The tree's ASSUMED strides are only the chain's own check.
- **Runtime and CI.** A full run takes about one minute of CPU on one core. It is not in CI; `tests/test_retained_pricing_tool.py` covers the keymap and the prune on a two-commit fixture repository.

## Adding an adjudication rule

Add a rule when a run stops or reports incomplete on a new head.

1. Read the reason in `runs/*/work/text_budget.caps.out.json` (`unmapped_loop_headers`, `unclassified_args`) or in the stop message.
2. Write the argument in DEL-05-02's `Design/retained-pricing-rules.md` under a new label. State the code facts it rests on, the bound or class, and why the bound holds on the D1 graph. A zero needs a premise, not an observation.
3. Add the rule to `rules/o10_rules.py` under the same label, keyed by name: `fnkey(name, file)` or `site(fn, file, text)`, never a bare line number.
   - Guard it with `if fnkey(...)` so that heads without the function add nothing.
   - Use the existing families: `O10_LOOPS_LAST` (exact header, bound, why, the only `file:fn` it may match), `O10_EDGE_ZERO`, `O10_SITE_FROM`, `O10_SITE_SIZE`, `O10_ID_AUDIT` and `O10_ARGS_FIRST`.
   - For a site you claim has multiplicity 0, prefer `O10_SITE_SIZE` 0, which `o10_check` guards, over a zero.
4. If a function the decisions name was renamed or deleted, change only the affected entry and leave its argument unchanged. Use `rs_any` for a rename, or a `KEYMAP_ALLOW` entry with its reason for a deletion.
5. Re-run the head and the registered revision. The registered tree must still reproduce exactly. Then compare against the registered tree as above.
