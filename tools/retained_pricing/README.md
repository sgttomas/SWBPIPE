# Retained pricing

This tool prices the retained route's memory at a revision and generates the GENERATED PROFILE block in `core/product_physics/src/retained_memory.rs`. Registration separately binds the actual build, reviewed inputs, layouts and selected threshold.

The base chain's `g4_caps.py` keeps the historical illustrative `M = 4_026_531_840`: `tree.M`, `fraction_of_M`, `margin_rule_0.9M` and `fits_M` use illustrative layout strides and are not native budget qualification. `run` exit 0 establishes pricing completeness. `generate` replaces only the generated block and preserves `REGISTERED_PROFILES.threshold_bytes`; it does not select or register M. `eval` (`lib/tree_eval.py`) uses supplied native atoms to evaluate forms/phases without applying a budget. Current qualification uses the source-selected registered threshold and the native law tests' checks of all phases, R, the 0.9 margin and the upward-rounded 5% TAV_W/TAV_X reserve.

It is SQ2's TEXT and cap-priced chain under the O-10 rules, in one Python command (standard library only). The reasons for the rules it adds by name are in DEL-04-08's [retained-pricing-rules.md](../../execution/PKG-04/DEL-04-08/Work/retained-pricing-rules.md). Use this tool, not earlier copies of the chain.

## Use

Run from any directory:

```bash
P=tools/retained_pricing
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

**Repository.** A clone with full history and `reference/migration/commit-map.txt`, which maps
the original basis `2243be380b` to its equivalent extracted commit; a shallow clone is refused. The default is this checkout. `--repo` or `RP_REPO` selects another.

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

1. Resolve the revision (use `source:<original-sha>` for an explicit original
   Chirality identity), then `git archive` its SWBPIPE source tree without
   `execution/`. Both standalone and original monorepo layouts are supported.
2. Carry the base chain's line-keyed rules (`base/*.g4.json`, `base/sens.py`, `base/premise_pins.json`) from the basis to the revision with `lib/linemap_rules.py --prune-deleted`.
   - **Keymap.** `lib/linemap_lib.py` maps each key in order:
     1. **Offset:** by the cumulative offset, checked.
     2. **Function:** by the same ordinal `fn` definition.
     3. **Text:** by the line's text.
     A key that none of these carries stops the run.
   - **Prune.** A key is pruned only when the revision deleted its line, or deleted its function so that no `fn name` is left in the file. The key's rule is dropped, including list-form rules, and the drop is reported. Dictionary rules are discarded by their original key identity before insertion at mapped keys, so a deleted old key cannot overwrite or prune an unrelated live rule remapped to that spelling.
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
- `linemap_resolutions.json` holds the reviewed resolutions. One `fewer_definitions` entry is the frame kernel's `UserStiffnessElement::orientation`, deleted by T4-U3; at `666cb30520` it reproduces O-10's tree. One site entry is `load_reference_source.rs:47`, inside the private `fn code` B7b deleted with the v2 readers. Five site entries are `preview_physics.rs:1093`-`1095`, `1119` and `1122`, inside the pre-parse `fn imposed_displacement_refusal` W-16 slice a deleted.

**`lib/`** is the machinery.
- `linemap_lib.py`, `linemap_rules.py`, `sq2_rules.py`, `rule_ops.py`, `o10_check.py`, `merge_trees.py`, `tree_eval.py` and `census_patch.py` are T3's O-10 tools.
- `census_exact.py` takes route E's census by I95's method on the pressure-source-1 producer fixtures (U6-RP), counting each source-selected extremum at its largest `locations` shape (U6-RL): `python3 census_exact.py <projects/chirality-piping dir> <out json>`.
- `mk_looplog.py` is vendored unchanged from the I104 record above (`tools/mk_looplog.py`).

**`data/`** holds three files.
- `basis_text_rows.json` lists the (file, line) of the basis runs' TEXT rows. The line map uses it only to choose among files that a short key could name.
- `census_c3cf.json` is I95's exact-evidence census, used for route E's per-case evidence forms before D-5.
- `census_d5_79bf8656dc.json` is the D-5 census (72 rows per member and the per-basis evidence) on the committed pressure-source-1 fixtures as regenerated at `79bf8656dc`, used on D-5 source. `census_exact.py` reproduces it byte for byte (`tests/test_census_exact.py`).

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
2. Write the argument in DEL-04-08's `Work/retained-pricing-rules.md` under a new label. State the code facts it rests on, the bound or class, and why the bound holds on the D1 graph. A zero needs a premise, not an observation.
3. Add the rule to `rules/o10_rules.py` under the same label, keyed by name: `fnkey(name, file)` or `site(fn, file, text)`, never a bare line number.
   - Guard it with `if fnkey(...)` so that heads without the function add nothing.
   - Use the existing families: `O10_LOOPS_LAST` (exact header, bound, why, the only `file:fn` it may match), `O10_LOOPS_FIRST` (U6-LF: the same, placed before the earlier broad rules for an exact header one of them would otherwise capture), `O10_EDGE_ZERO`, `O10_SITE_FROM`, `O10_SITE_SIZE`, `O10_ID_AUDIT`, `O10_ARGS_FIRST`, `O10_DIAG_EXCLUSIVE` (B7b-DX: diagnostic sites of one function of which a call pushes at most one; `text_budget.py` counts the group once per call at its largest member and leaves the run incomplete unless every member is an unlooped site of that function with its multiplicity), `O10_MULT0_SITES` (B7b-JR: a site whose own multiplicity must stay 0, with no size or zero rule; `o10_check` fails the run otherwise) and `O10_TYPED_PATCHES` (U6-RB: a route's exact-text substitution in one chain file, with its reason).
   - For a site you claim has multiplicity 0, prefer `O10_SITE_SIZE` 0, which `o10_check` guards, over a zero.
4. If a function the decisions name was renamed or deleted, change only the affected entry and leave its argument unchanged. Use `rs_any` for a rename, or a `KEYMAP_ALLOW` entry `("deleted", why)` for a deletion.
5. Re-run the head and the preceding registered revision. The preceding tree must still reproduce exactly. The KD5 source pins in `base/kd5_text_model.py` and `base/kd5_first_error_model.py`, and the source-bound closure pin in `rules/o10_rules.py`, hold one current value each, so the committed tool stops (exit 4) on an earlier revision once a pin moves; reproduce it with a scratch copy of the tool whose pins are restored to that revision's values, with the new rules kept. Restore the original case consumer in place of the batch 5 wrapper/helper pair when reproducing that earlier source. Then compare against its registered tree as above. The B7a source models' full-file pins (`base/b7a_*.py`, `lib/b7a_retired_carry.py`, `base/batch5_model.py`) and C7's NumericCase pins (`base/prescribed_key_model.py`) work the same way.

   At RF qualification the coefficient basis was `677f11b674`, registered in `ffe51350ab`. Using the historical RF tool, the preceding T4-U6/U8 basis `588de796a4` was reproduced by retaining RF rules in a scratch copy, restoring the `SOURCE_FILES` assignments in `base/b7a_domain_model.py`, `base/b7a_formation_model.py` and `lib/b7a_retired_carry.py` from `4971d62dc9`, and restoring `base/b7a_owner_choices.py` and `base/b7a_phase_model.py` from that revision. This audited the +41/+2 source-key offsets and reproduced all 1,456 coefficients and the original 1,544-line block. That recipe names the historical RF tool; the current W1 tool fails closed outside its reviewed cohort and cannot replay RF by restoring only pins/keys. For the earlier B7b reproduction at `837a04b770`, see retained-pricing-rules.md, "T4-U6/U8: actual-source D3".

**W1 reductions (before member-load implementation).** `base/w1_reductions_model.py`
explicitly pins the RF and reduced producer/consumer cohorts. The reduced cohort
borrows typed primitives (L1a), keeps one closure-comparison transient (L1b),
charges scaled-force children only in their call-local nonzero-scale evaluation
(L1c), bounds uniform-constructor/attachment staging by one sequential call/term
(L2), moves formation Values (L3), and guards the combination index and its
callee by `combining` (L5). Stage-1 formed/success counts remain unchanged at
this checkpoint. No stage-2 estimated owner is included.

U6-RB uses the same pinned production guard and test-only shadow as L5. Missing,
former or altered reduced-source guard text stops with exit 4; route E never
silently regains the index clone. Route L at `z >= 1` separately prices the
callee's sorted `base_ids` and cloned `case_rows`. Source/lifetime mutation tests
and the actual driver exit-code controls are in `tests/test_w1_reductions.py`.
They establish source/ownership warrants, not runtime allocation observations.

The older B7a/KD5 bindings now name this reduced source and fail closed outside
that reviewed cohort. Reproduce the RF registration with its historical
committed tool and source revision `03f39bb49d9`; the RF baseline comparison was
performed with that governing tool before the W1 rule changes. Merely restoring
pins/keys in the new tool is insufficient: the current formation traversal
warrant also names Cow's `.iter()` spelling. The explicit RF branch in the new
reduction model preserves its local historical formulas, but does not establish
whole-tool replay of an older registration. Helper absence alone earns no
reduction.

### W1 stage 2 actual-source owners

`base/w1_member_load_model.py` binds the complete member-load producer,
declared ledger, recovery, maximum and independent-reader call chains. It adds
positive member-only sidecars, the full formed ledger beside the existing
nodal ledger, recovery/E and two-lane curvature vectors, loaded search and PP
completion owners. All native strides come from the generated build bindings.
The successful formation count is now all authored `lc` cases; stage 1's
`lc−1` preparation-refusal credit is retired. Ordinary/fallback owners remain.

The new reader cap is an additional 1 MiB per case plus an equal moving
allowance, 6,291,456 B at three cases, in every phase. Existing `G8_VALUES`
and `G8_BYTES` remain separate. The source inventory bounds complete encoding
reallocation, independent integer recomputation, hashing and stack staging;
native and real-wasm requested-allocation controls check its consuming path.
The loaded search similarly retains a separate 1 MiB source-bound workspace,
with returned hulls, diagnostics, PP spans and evidence charged independently.
See the W1 section of the governing retained-pricing rules for the inventories.

### Epoch 4 (T4-U4b/U5b) actual-source owners

`base/u4b_u5b_owners_model.py` binds, by function-body sha256, the epoch's heap owners outside the TEXT inventory
(U5B-PD, U4B-ST, U5B-AB, U4B-W1, U4B-RD, U4B-NQ) and prices each positively in `ordinary_caps.py`,
`producer_caps.py`, `g4_caps.py` and `t25_g4.py`; a snapshot without them adds nothing and a changed body stops.
Route L's P_final takes the producer's `FAILED_MEMBER_ROWS` (U4B-RL) and route E's the U4b census (U4B-RP). See
the governing rules' §8.8.

T4-S6's streamed publication hashes (S6-HS) are read by `u6_owners_model.py` from each site's exact text and the
canonical writer's pinned production code, and priced by its `stream_route` in `g4_caps.py` and `t25_g4.py`; the
former U6-DH/U6-SH forms keep their prices. The same section holds T4-S6's U4B-W1, U4B-NQ, U5B-CG and failed Preview
formulation corrections.

### O4 actual-source owners

`base/o4_owners_model.py` binds, by function-body and span sha256 and by each exact allocating statement, the heap owners
the O4 source (Timoshenko documents and distinct section bases on route E and the pressure-source route) adds outside the
TEXT inventory, and prices them positively at the caps with every member Timoshenko and every member's bases distinct:
O4-PS/PA (member sidecars and preparation records), O4-ID (the K4SRC/K4STF Timoshenko block in E_src and E_stf),
O4-RS/NC/PP/KS (the reduced section set, maps, operand tables, maxima, payload and kernel staging), O4-TR (trace
premises), O4-WR (receipt keys, class entries and reduced preparation records), O4-RD (reader sets and block bytes),
O4-SR (the second composite maximum), O4-SP (source payload keys) and O4-EV (Timoshenko evidence entries). A snapshot
without them adds nothing and a changed body stops. See the governing rules' §8.9.

### W1 coverage repair actual-source owners and the off feature

`base/w1r_owners_model.py` binds, by function-body and span sha256 and by its exact statements, the certified product
proof's `final_scales` (the gate's coupled, floored per-body scales, now held with the proof to serialization) and prices
the backing and the proof's Vec header in each holder per attempted case (W1R-FS). `base/w1_member_load_model.py`'s
loaded-search inventory adds the driven search's frame endpoints (W1R-LM), and `rules/o10_rules.py` bounds selection's
body-scale loop by the bodies (W1R-L1).

The census probe (`census-probe` feature) is compiled out of the registered build, which has no features (W1R-FG).
`base/loopscan.py` blanks items and statements under `#[cfg(feature = "census-probe")]` or
`#[cfg(all(test, feature = "census-probe"))]` as it blanks `#[cfg(test)]` items, and the call graph, the template
inventory and JN1's inventory skip the two files the build includes only through compiled-out module declarations
(`census_probe.rs`, `w1_census_driver.rs`). The inclusion and manifest guards (`off_build_files`, `off_feature_audit`)
stop the run (exit 4) on any other inclusion, a plain module, a new off-build file or a manifest line that could enable
the feature. Corner 1 adds the off feature `loaded-maximum-hooks` (the kernel's capacity hook for the genuine-exhaustion
control), under the same blanking and manifest guard. See the governing rules' §8.11 and §8.12.

### R-A actual-source owners

`base/ra_owners_model.py` binds the heap owners that the source route's work reduction (R-A) adds outside the TEXT
inventory. It binds them by function-body and span sha256, by its exact statements and by `mod shortcuts`' count of
allocating constructs. `t07_repair.py` and `t25_g4.py` price them positively:
- RA-HS: the kernel's B1 hint vectors, one `Option<u32>` per descriptor (the new in-build atom `s(Option<u32>)`). The
  plan's is in T07 E3 and the replay's in T25.1f.
- RA-SC: `evaluate`'s Shortcuts buffers, `zero`, `window`, `products`, `touched` and `source_ok`. The live
  evaluation's are in T07 F3 and the replay's in T25.1f.

A snapshot without them adds nothing, and a changed or partly present owner stops the run. `rules/o10_rules.py`
zeroes one false by-name value edge, from `certify_rows`' `row` binding to R-A's row-trace recorder (RA-EDGE). The
row trace and the long reference entry sit under `exact-operation-count`, which the scanners blank as test code. PP's
dev-dependency compiles that feature into every PP test build, but the trace allocates only after `take_row_paths`,
which only the gating tests call. See the governing rules' §8.13 for the arguments.

### W-8 topology warnings actual-source owners

`base/w8_topology_model.py` binds topology.rs (full-file sha256), validation's `validate_components` and
`branch_topology_invalid` bodies, the exact allocating statements and topology.rs's inventory of owning constructs. It
prices positively, in `ordinary_caps.py`, the topology checks' transient working set once per ordinary run (W8-TW: the
coordinates, element edges, a second Bodies, the part map and sets, the cell grid, a per-node candidate buffer of
at most n-1 entries and at most six named pairs per warning) and validation's node-degree map (W8-ND, <= 2m ids). `rules/o10_rules.py` prices the warnings'
texts (W8-L1/L2, W8-NI, W8-ID, W8-JN, W8-RF, W8-CG) and guards the branch-topology warning at multiplicity 0 (W8-BT);
`lib/sq2_rules.py` places the `O10_REFS_FIRST` affected-refs rules before the inherited ones. A snapshot without
topology.rs adds nothing; a changed or partly present owner stops the run. See the governing rules' §8.16. The absolute
node-proximity tolerance (§8.18) adds W8-BF (the branch warning's statement of the published factor sides, added after publication over the model's
components, at multiplicity 0 by W8-BT's premise) and NP-MS
and NP-PT (validation's refusal of a malformed `model_settings.node_proximity_tolerance` and the accessor's texts).

### Silent-drop guard actual-source owners

`base/sd_owners_model.py` binds, by their exact statements in their functions, the owners the pressure-free input
closure adds outside the TEXT inventory and prices them positively in `ordinary_caps.py`: SD-WH (the combination
callee's withheld names, the warning's cloned list and a support group's rows, route L with combinations), SD-HS
(validation's duplicate-term set) and SD-PB (the closed-record scan's probe error). `rules/o10_rules.py` zeroes the
scan's finding and refusal sites by its place before admission (SD-NF, guarded), bounds its loops (SD-L1) and texts
(SD-ID, SD-WT, SD-DT) and the duplicate loops (SD-DL) and removes one by-name false edge (SD-EV); `rules/sq2_decisions_common.py` re-keys two replaced
combination audit keys by name (SD-RK). A snapshot without the guard adds nothing. See the governing rules' §8.19.

### W-7 global end rows actual-source owners

`base/w7_owners_model.py` binds, by their exact statements in their functions, the owners W-7's member end actions in
global coordinates add outside the TEXT inventory and prices them positively: W7-NC (`prepared_not_covered`'s per-member
end-action table, `producer_caps.py` T14.7, per attempted result set), W7-QF (`qualify_source_case_rows`' filtered
references, `ordinary_caps.py`) and W7-RD (the readers' coverage maps, `g4_caps.py` T17 V5, with four BTree node atoms
bound in `g5_profile.py`). The rows themselves are inside P_final: route L takes `FAILED_MEMBER_ROWS` = 78 (W7-RL) and route
E the census's 102 plus the twelve global end rows its pressure-source-1 fixtures omit (W7-RP, `lib/census_patch.py`).
`rules/o10_rules.py` bounds the new loops (W7-L1) and texts (W7-ID), zeroes one `Option::replace` site (W7-RZ) and re-keys
the stress-free notice's audit by its new text (W7-SF). A snapshot without W-7 adds nothing; a partly present or changed
owner stops the run. See the governing rules' §8.20.

### W-15 rigid-component scaling actual-source owners

`base/w15_owners_model.py` binds, by the sha256 of W-15's two new modules (PP `rigid_component.rs`, RE
`component_body.rs`), their exact allocating statements and the readers' call sites, the positive owners W-15 adds on
D1, where no component is admitted (P2) and so no member is scaled: W15-PM (the eligibility predicate's and
mapping-conflict check's member maps, two exact-size `(&str,&T)` tables of m entries, in `ordinary_caps.py`
BuildScratch) and W15-RB (the readers' component-body check on the projected successor: the case and entry lists and
one cloned extremum and coverage entry, in `g4_caps.py` T17 V5). `rules/o10_rules.py` zeroes the scaled-member path's
sites by P2 under a source guard (W15-NZ), bounds the new loops (W15-L1) and texts (W15-ID); U4B-NQ's guard excludes
the record's `STRESS_KINDS` list, a reader of the derived kinds (W15-SK). A snapshot without the modules adds nothing;
a changed or partly present owner stops the run. See the governing rules' §8.21 and `tests/test_w15_rigid_scaling.py`.

### W-15 slice b component weight

Slice b's component-weight generator adds no owner on D1. Its lineage functions are reached only after
`inspect_applied_self_weight`'s `continue`, which every admitted load takes because D1.10 refuses object provenance
and D1.7 refuses the deterministic element form and, since slice b, a node load in the generated self-weight
namespace. `rules/o10_rules.py` zeroes the edge into `generation_basis` and the wrench-set key site under a source
guard (W15B-NS). The tree is coefficient-identical to §8.21's. See the governing rules' §8.22 and
`tests/test_w15b_component_weight.py`.

### W-16 slice a imposed displacement on the pressure-free route

`base/w16_owners_model.py` binds, by the sha256 of W-16's new module (PP `imposed_motion.rs`) and its exact statements,
the positive owners it adds on D1, where D1.7 refuses every support-targeted primitive: W16-NI (`case_prescribed`'s node
index, one `(&str,usize)` table of n entries per case solved without a load state, on both routes, `ordinary_caps.py`
CasePrefix) and W16-VF (`validate`'s findings vector, at most one per primitive load, BuildScratch). `rules/o10_rules.py`
zeroes the support-target branch's sites and edges under a source guard (W16-NS), bounds `validate_pressure_free`'s calls
of `refuse` by cases·l (`EDGE_PER_CALL`), and audits the reachable node/element refusal's texts (W16-ID);
`rules/sq2_decisions_E.py` zeroes `validate` → `validate_pressure_free` on route E (EXACT). See the governing rules'
§8.23 and `tests/test_w16_imposed_motion.py`.

### W-16 slices b and c exact combinations and equivalent-static generation

Slices b and c add no owner on D1, which admits neither feature on a retained branch: route E's D1.4 exact clause
refuses every combination, route L has no exact contract, and D1.5 refuses every case with `equivalent_static` on both
routes. `rules/o10_rules.py` zeroes, under source guards, the exact combination append and operand standing (W16B-CB),
the pressure-2 reader's combination pass behind `present()` (W16B-RD, whose direct pass is counted once per call by
`EDGE_PER_CALL`), the generator's new calls, the equivalent-static record and its two readers (W16C-ES), and the
by-name edges into the two new modules' private functions (W16BC-BN); W16BC-ID classifies the new modules' unreached
format arguments. `rules/sq2_decisions_E.py`'s B7b-LR zero follows the direct pass. See the governing rules' §8.27 and
`tests/test_w16bc_combinations_equivalent_static.py`.

### W-16 stage 2 imposed displacement with nonlinear supports and on spring bases

`base/w16_owners_model.py` adds W16S-SG: each case's spring-base vectors (`case_spring_ground`'s `ground` and `moved`,
s entries each, `ordinary_caps.py` CasePrefix). `rules/o10_rules.py`'s W16-NS follows the support branch into its new
helpers, `target_acting` and the two new resolvers; W16S-SG zeroes `spring_base_motion`'s formed terms behind
`g == 0.0`; W16S-L1 bounds the relative-displacement loop at 0 on D1 and zeroes its clause; W16S-ID classifies the
helpers' unreached format arguments. See the governing rules' §8.29 and `tests/test_w16_imposed_motion.py`.

### §8.30: the three corrections and W-16t partial-span wind on Timoshenko members

`rules/o10_rules.py` adds the following rules; none adds an owner on D1:
- C30-JR: the shared-terminal junction refusal's node map is empty on D1 (D1.5), so its node loop is bounded at 0.
- C30-ID: the junction message's argument classes.
- C30-DG: the nonlinear dense-memory guard runs only with a nonlinear support (D1.6). Its edge and diagnostic sites are zero; NI's two Display forms stay priced.
- W16T-SP: a Timoshenko proper span arises only from wind generation (D1.5, W16C-ES), so SP's partial-span bound is zero.

The hanger-load warning inherits the basis's hanger zero. `base/batch5_model.py`'s pressure prefix and `base/b7a_scoped_text_model.py`'s `problem` parents follow the junction refusal. See the governing rules' §8.30 and `tests/test_830_corrections.py`.

### §8.31: nonlinear pressure-free combination operand standing (slice 1a)

Route L's current projected base is preview-physics-2, so the new reader is
reached through `semantic_contract::for_source`; R1's original preview-physics-1
wording is historical. `rules/o10_rules.py` adds OP1A-PF (empty nonlinear-only
producer tails under D1.6), OP1A-RD (empty record/notice loops, with the reader
scans retained), OP1A-BN (four private-helper name collisions), OP1A-ER (one
returning error per reader call), OP1A-ID (site-specific literal/list classes)
and OP1A-OWN (positive borrowed-gate-vector backing beside V5, including its
previous push backing). Raw producer/reader pins and mutation tests in
`tests/test_op1a_operand_standing.py` guard the premises.

The merged tree remains §8.30's 1,586 coefficients; use §8.30's registered basis
line with `generate --check`. The two unmerged combination points additionally
use native `Node(String,BTreeMap)` = 648 B; §8.31 gives its collector expression
and the atom-log supplement needed by `eval`. To replay the preceding registration while keeping
the new rules, recover main `9ef6c4c469`'s complete tool cohort into scratch,
replace only its `rules/o10_rules.py` with this turn's, and run main
`9ef6c4c469` with `--repo` pointing at the repository. OP1A's branch is inactive
on that source. See the governing rules' §8.31 and §10.

### §8.33: tees and implicit pressure junctions on the exact route (RT-A3 increment 1)

`rules/o10_rules.py` adds the following rules; none adds an owner on D1:
- RTA3-JW: the junction warnings' node map is filled only from a case's pressure regions, explicitly empty on E (D1.5) and absent on L, so its node loop and `ending` maps are bounded at 0.
- RTA3-ID: the warnings' argument classes, site-specific.
- RTA3-BB: the branch rows' basis loop (at most two bases) sits inside the association loop that D1.4's empty components bound at 0.

`base/template_inventory.py` reads Rust's `\u{HEX}` escape as its one character. The registration and its basis are §8.33's (`7e7fef2d4`); main `d290563ea` reproduces §8.32's block and is its comparison basis. See the governing rules' §8.33 and `tests/test_833_rt_a3.py`.

### §8.36: model document 0.9.0 (S1), the interim retained boundary and the typed-string census (candidate)

D1.3 declines non-empty condition sets and spectra and any pipe's `condition_set_ref`, so the 0.9.0 lowering and
withholding judge empty members on D1. `base/s1_owners_model.py` prices their remaining owners (S1-VW: the lowering's
views, findings and operand resolution; S1-WH: the withholding pass), bound to the three new modules' bytes and the two
calls. `base/typed_strings_model.py` replaces T03's basis count of a pipe's strings (12) by the census of the actual
records (13), and adds route E's three contract strings and the node-proximity tolerance's unit (S1-TS), in
`ordinary_caps.py` and `t25_g4.py`. `rules/o10_rules.py` adds S1-PIN, S1-BN (exact call targets for by-name false edges),
S1-NR (SD-NR's guards), S1-L1, S1-ZR, S1-ZS, S1-ID and S1-MS; `rules/sq2_decisions_common.py` re-keys the factor
diagnostic's two audits by name (S1-RK); `base/sd_owners_model.py` prices SD-HS's `(bool, &str)` set (S1-HS). The tree is
a candidate: its generated block is not adopted. See the governing rules' §8.36.

The current source pins fail closed outside the reviewed stage-2 cohort.
Reproduce either RF or the pre-stage-2 reductions using its committed tool and
source revision. A complete `run` is a pricing-completeness result; actual
registration additionally requires native atoms, all 80 budget evaluations,
reviewed inputs/layouts and independent source/pricing scrutiny.

### §8.37: RT-A3 increment 1b published-row lookup index

`base/u4b_u5b_owners_model.py` binds the case-level `PublishedScope` and the
member-level `MemberRows`. RTA3B-PS prices the entity hash table against the
existing guarded P_final row cap, including resize overlap; removals do not
require a grow-only premise. RTA3B-MR prices the thirty station/kind vectors
against the guarded per-member row allowance. Both owners are charged per case
beside U4B-ST in `ordinary_caps.py`. Partial or changed index ownership fails
closed; `tests/test_837_rt_a3_1b.py` exercises the charges and guards. The
combined-source registration follows model document 0.9.0's §8.36; the old
pre-S1 1b draft is not a registration basis.
