---
name: swbpipe-retained-pricing
description: Derive and compare retained-route source pricing and bound profiles.
metadata:
  kind: instruction
  surface: tool
---
# Retained pricing

This tool prices the retained route's memory at a revision and generates the GENERATED PROFILE block in `core/product_physics/src/retained_memory.rs`. Registration separately binds the actual build, reviewed inputs, layouts and selected threshold.

The base chain's `g4_caps.py` keeps the historical illustrative `M = 4_026_531_840`: `tree.M`, `fraction_of_M`, `margin_rule_0.9M` and `fits_M` use illustrative layout strides and are not native budget qualification. `run` exit 0 establishes pricing completeness. `generate` replaces only the generated block and preserves `REGISTERED_PROFILES.threshold_bytes`; it does not select or register M. `eval` (`lib/tree_eval.py`) uses supplied native atoms to evaluate forms/phases without applying a budget. Current qualification uses the source-selected registered threshold and the native law tests' checks of all phases, R, the 0.9 margin and the upward-rounded 5% TAV_W/TAV_X reserve.

It is SQ2's TEXT and cap-priced chain under the O-10 rules, in one Python command (standard library only). The reason for each rule it adds by name is stated with the rule, in `rules/o10_rules.py` and the `base/` models; the § numbers there are the rules' labels (DEL-04-08). Use this tool, not earlier copies of the chain.

## Use

Run from any directory:

```bash
P=tools/retained_pricing
python3 $P/price.py run <rev> --out <new dir outside any checkout> # price a revision
python3 $P/price.py compare <tree A> <tree B>                    # coefficient-wise comparison
python3 $P/price.py generate <tree> <retained_memory.rs>         # rewrite the GENERATED PROFILE block
python3 $P/price.py generate <tree> <retained_memory.rs> --check # compare instead of writing
python3 $P/price.py point-atoms <run dir> <law tests .rs>       # rewrite the GENERATED POINT ATOMS block
python3 $P/price.py point-atoms <run dir> <law tests .rs> --check # compare instead of writing
python3 $P/price.py eval <law-test log> <tree>                   # phases at in-build atom values
```

### Reading a profile change

1. **Run the new head.** `run <head>` should complete on all four points with no unmapped loop header and no unclassified argument, and both o10 checks should pass.
2. **Review the prune.** `summary.json`'s `linemap.pruned` lists every rule the line map dropped, each with its rule. Each one must belong to a function or line the head really deleted. A dropped `site_from`, `site_total`, `edge_per_call` or `loop_total` rule removes a bound, so read the code. The deleted test is per file: a function moved to another file shows up as deleted, and its rules must be re-keyed to the new file rather than dropped. `linemap.resolutions_used` lists the reviewed resolutions that applied.
3. **Carry the hand-kept keys.** Some models key current-source sites by line rather than through the line map (the
   B7a domain audits, owner choices, phase, returning errors and scoped TEXT, the `_X` emitter in `sens.py`, KD5C6's
   offsets). When a pin, an audit preimage or a caller inventory stops on one, map the old line to the new by the
   file's diff, confirm the text is unchanged, and move the key with a note of the lines that precede it. A key whose
   text changed is a re-read, not a carry. The registered tree for the comparison comes from the preceding revision's
   own committed tool (`git archive` it, with `tools/repository_history.py`, into scratch), checked with its own
   `generate --check`.
4. **Compare.** Use `compare <new tree_merged.json> <registered tree>`.
   - `a_le_b` true (every coefficient no higher) means the registered block still bounds the head, so nothing needs re-registering.
   - Otherwise, regenerate the block with `generate` and re-pin the law tests whose values moved. The law tests are `cargo test --lib profile_` in `core/product_physics`. Their `I65_G5_ATOM` lines feed `eval`.
5. **Check the block.** `generate --check` confirms that a committed block is the generator's output for a tree.
6. **Bind the points' atoms.** The separate points L(2,1) and L(1,2) use atoms the merged tree does not (the combination owners). `point-atoms <run dir> core/product_physics/src/retained_memory_law_tests.rs` writes them, bound by `g5_profile.py`'s own bindings, into the law tests' GENERATED POINT ATOMS block, and `profile_point_atoms_in_build` prints them as `I65_G5_ATOM` lines beside the profile's. An unbound atom, or one bound only as an Estimate, stops the generator: a point is never evaluated at an ASSUMED stride. `point-atoms --check` confirms the committed block against a run. The block is test-only; regenerating it changes the lib-test binary, not the profile or its record.

### The basis line

`--basis` replaces the tree's basis line.

- **Plain text:** written after "corrections at", followed by a period.
- **Text starting with ",":** continues the sentence after "corrections," and is wrapped at 100 columns. Give the final period yourself.

For example, main's block at `2c5cf0823b` is the registered tree with:

```
--basis ", re-run as SQ2 G5 at f78f52b7e2: the maximum over routes L (C_eq <= 3, worst shape c = 3, z = 0) and E (c = 3), under SQ2's rules; text at l <= 128."
```

## Inputs

**Repository.** A clone with full history; `tools/repository_history.py` maps
the original basis `2243be380b` to its equivalent extracted commit; a shallow clone is refused. Only the original identities that maintained checks still cite are tabled there; any other `source:<original-sha>`, including the historical replays named below, is refused. To resolve one, read `reference/migration/commit-map.txt` from Git history (it was removed from the working tree on 2026-11-10), or point `--repo` at a worktree of a commit that still holds it. The default is this checkout. `--repo` or `RP_REPO` selects another.

**Revision.** Any revision whose code the rules cover.

**Tools.** Python 3.9 or later and `git`. The chain's scripts call `python3`; the driver puts its own interpreter first on `PATH`, and the driver and the chain run in Python's UTF-8 mode.

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
   Chirality identity), then write its SWBPIPE source tree without `execution/`
   from the raw committed blobs (`ls-tree -r`, `cat-file --batch`); no attribute
   or git setting changes a byte. Both standalone and original monorepo layouts are supported.
2. Carry the base chain's line-keyed rules (`base/*.g4.json`, `base/sens.py`, `base/premise_pins.json`) from the basis to the revision with `lib/linemap_rules.py --prune-deleted`.
   - **Keymap.** `lib/linemap_lib.py` maps each key in order:
     1. **Offset:** by the cumulative offset, checked.
     2. **Function:** by the same ordinal `fn` definition.
     3. **Text:** by the line's text.
     A key that none of these carries stops the run.

     The hunks come from `git diff -U0` with the histogram algorithm and every other output-affecting setting pinned on the command line. Every git read runs through `lib/pinned_git.py`, with no global or system configuration, no output-changing `GIT_*` variable and strict UTF-8 decoding, so the user's git setup cannot change a carry.
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
8. Merge: `lib/merge_trees.py` takes route L at (3,0), SQ2's worst shape, and route E. Route L's other shapes are priced so they can be compared. Their phases at in-build values come from `eval <law-test log> runs/L_21/work/profile_tree.json` (the log of the `profile_` law tests, whose `I65_G5_ATOM` lines include the point atoms, step 6 of "Reading a profile change"), and they should stay at or below the merged tree's in every phase and mode; the tool does not check this itself.

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
  - the same carry, with and without a rename, under adverse repository-local configuration, attributes and replace refs, `GIT_*` variables and global configuration and attributes; UTF-8 reading in the carry, the driver and a chain step under an ASCII locale; the snapshot's raw bytes under adverse attributes and replace refs;
  - the decision-key stop and `KEYMAP_ALLOW`;
  - the `EDGE_ZERO_REMOVE_CALLER` count;
  - the generator's basis line;
  - RT-L2's owner bindings and the typed census's acceptance of the setting (`tests/test_rtl2_pressure_elongation.py`).

## Adding an adjudication rule

Add a rule when a run stops or reports incomplete on a new head.

1. Read the reason in `runs/*/work/text_budget.caps.out.json` (`unmapped_loop_headers`, `unclassified_args`) or in the stop message.
2. Write the argument beside the rule, under a new label, in `rules/o10_rules.py` or the `base/` model that binds its source. State the code facts it rests on, the bound or class, and why the bound holds on the D1 graph. A zero needs a premise, not an observation.
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

### RT-S3 constant-effort fallback

`base/ce_fallback_model.py` binds the six-line CE branch in `solve_prepared_load_case` and its two `lib.rs` tests. The
existing pins do not change value. B7a domain, KD5C4 and W1 reductions compare `lib.rs`, and KD5T2, KD5C6, batch 5
and O4 compare the case-solve body, after `s6_text`/`s6_bytes`/`s6_body` remove exactly that delta at its reviewed
anchors (CEF-PJ). Any other form is not projected, and the pin stops. `rules/o10_rules.py` zeroes only the new
`solve_prepared_load_case → classify_constant_effort_consumption` edge. Its guard, CEF-D1, binds the branch, its
receiver, predicate, classifier and force write, and D1.6. See the governing rules' §8.61.

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

The reader's dispatch has had three shapes, and W16B-RD guards each by its exact text, so earlier registered revisions
still price and any other text stops. In the one-view shape (Way H B2), `validate_pressure_evidence_without_certificate`
holds the dispatch itself. It asks `present` of the received document first, and calls `without_member` only in its
no-combination arm. `present` reads only `exact_combinations` and the result rows, and `without_member` removes only
the certificate member, so the arm taken is the RT-A3 §4 shape's. One per-call edge carries both former premises, W16B-RD's and
RTA3C-WC's: of its three direct-pass calls, the `present` arm's is unreached on D1, and the other two are the arms of one match.

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

### §8.38: RT-A3 reducer 2a bounded concentric pressure-bore steps

The combined source's full-file/prefix pins and scoped `problem` caller keys
follow the reducer guard. On D1, empty components/regions leave its new
classification and geometry paths unentered. Existing U6-J4 reads the longer
identity; resident pressure-table limitation text raises STATICS. The proposed
profile raises each native merged phase by 1,751 B; all 80 native evaluations
fit the unchanged M/margin and upward-rounded TEXT-reserve policy. The protected
profile is owner-approved (2026-11-02, item 1) after clear focused scrutiny. See the
governing rules' §8.38 for the exact source, proposal and remaining checks.

### Modal leaf seam (DEL-04-09 I1, §8.39)

The retained binary64 frame leaves are bound by B7a `LEAF_SEAM` as exact lifts,
with the straight/curved wrapper delegations and single retained implementations
checked alongside the full source pins. W1 reduction owners/lifetimes stay
unchanged; live structural owner keys move by the module declaration's two lines.
The four-point source run at `8e91c3941`, on integrated reducer2a `393a11ee7`,
keeps all 1,602 coefficients and the registered generated block unchanged.
Extra debug wrapper/accessor stack depth is assessed separately through AC-004;
heap coefficient identity alone does not settle that check. See the governed
§8.39 proof and `tests/test_dynamics_d3.py` for mutation refusals.

### §8.40: model document 0.9.0 S2 pressure-free resolution (conservative carry)

On D1, D1.3 and D1.5 leave `resolution::plan` with an empty set map, and every
`resolve_case` returns `Ok(None)` at its first statement. `rules/o10_rules.py`
adds:
- S2-PIN: `resolution.rs`'s bytes and premise checks.
- S2-ZR: zeroes the heap-pricing edge `plan` → `resolve_case`; the call/frame remains live.
- S2-BN: `plan`'s `as_str` has no crate target.
- S2-ID: the resolution's unreached format arguments.
- S2-RK: re-keys the S1 rules that moved from `lower` into `judge`.

`base/s1_owners_model.py` prices S2-JD: `judge`'s exact-size `Vec<Diagnostic>`
for a document its findings refuse (≤ 4z, route L with combinations). PP
`lib.rs` keys move +49 with unchanged text (the closed wind wire precedes them).

Against main `4cf32bfd4`, six TAV coefficients fall by 3,990,536 B, because
§8.36's short-key over-bound is no longer met. No coefficient rises. The native
atom `s(PreviewLoadCase)` grows 672 → 680 B, so the in-build record moves +24/+48 B
at actual native types. Law/challenge expectations derive from the kept block
and those types; the generated block and registered identity/inputs/layouts
remain unchanged. See the governed §8.40 proof and
`tests/test_840_rt_g5_s2.py`.

S2 keeps the registered coefficients under the `a_le_b` carry rule. Reproduce
the kept block using the committed predecessor `4cf32bfd4` tool/source and adopted
§8.38 basis; compare the current source-derived tree against that result, then
evaluate both with actual native atoms. The six-lower S2 generated proposal is
comparison evidence, not the kept block; `generate --check` of that proposal
against the kept block is not an equality claim. See §8.40 for native-layout
changes, the independent review and remaining qualification.

### §8.41: RT-A3 reducer 2b (candidate; profile disposition reserved)

2b's pressure-step branch adds no owner on D1: `pressure_step_run_evidence`,
`reducer_body_members` and the extracted helpers are reached only behind
D1-empty loops. `base/w15_owners_model.py` binds that premise (RTA3R2B-NZ: one
caller each, after the D1-empty loop header), alongside re-read full-file pins,
batch 5's prefix and the scoped `problem` keys (1882, 1959). The run is complete
and O-10 clear, the line map equals main's, and no function multiplicity
changes.

The restated pressure-2 limitation raises `STATICS[1]` by 286 B and the longer
identity raises TEXT by 876 B. Against the kept block, two coefficients are
higher and six lower, so the `a_le_b` carry does not hold. Proposal A, made
with `lib/merge_trees.py` from the kept and candidate trees, keeps every kept
coefficient except +286 B on `STATICS[1]` and its success copy. Proposal B, the
source-derived block, is comparison only. Both rebind only reviewed inputs #8
and #11. All 80 native candidate evaluations fit and lie under the kept merged
phases. Adoption, the law records and the stack witnesses wait for the owner's
disposition. See the governing rules' §8.41 and
`tests/test_841_rt_a3_reducer_2b.py`.

### §8.44: RT-A3 increment 3 constant-effort supports (proposed registration)

D1.6 refuses every hanger and every support family but the five it lists, so no
D1 model holds a constant-effort record; gate 4 runs only inside the zeroed
exact combination append (W16B-CB), and the pressure-2 reader's model binding
of applied-load rows is reached only from `reader_verdict`, outside the retained
graph. `rules/o10_rules.py` adds RTA3CE-ID: `applied_load_row`'s two hanger
references are IN128 (unreached), guarded by their source in
`consuming_constant_effort`. The full-file pins of `exact_admission.rs` and the
two RE hash-constant files follow. Derived on frozen #96 (§8.43). The run is complete and O-10 clear, the line
map and every function multiplicity equal #96's, and 12 coefficients rise
(the restated pressure-2 table in STATICS, +788 B with one node and six values;
the identity's constant-effort clause and the D-2 sentence in TAV, +3,894 B).
The block is regenerated; reviewed inputs #8 and #11 rebind; every phase rises
by 5,610 B. After review, S7 restates a consumed constant-effort support's signed
support action (+67 B, six TAV copies): re-derived on main `6c3cb46d3` with
§8.45's lexer, the same twelve coefficients rise against main's corrected tree
(every phase +6,012 B), the block is regenerated on the corrected tool, and
reviewed input #10 rebinds the pressure-2 result schema (gate 4 in its enum). See the governing
rules' §8.44 and `tests/test_844_rt_a3_ce.py`.

### §8.46: RT-A4 absolute sums, SRSS and nested combinations

`rules/o10_rules.py`'s RT-A4 block bounds the plan, combination and reader loops (RT-A4-L1), zeros what D1.4 leaves
unreached (RT-A4-UZ, RT-A4-WZ, RT-A4-RM; nested re-derivation per call RT-A4-NR) and the historical read's selection on
the retained graph (RT-A4-HS), and classifies the new sites. `base/rta4_owners_model.py` prices the plan (RT-A4-PL),
the combination callee's buffers and published operand rows (RT-A4-CB), the derivation member (RT-A4-DV) and the
preview reader's sets (RT-A4-RD, route L only: RT-A4-RE). Two tool repairs: `_production()` removes each
`#[cfg(test)]` item instead of truncating at the first (RT-A4-PR), and `text_lexicon` reads `token` as text (RT-A4-TK).
See the governing rules' §8.46.

### §8.47: RT-U4 region binding in one pass and the shear operand rule

`base/rtu4_owners_model.py` binds the exact reader's one-pass region binding by its three statements and prices
RTU4-RC in T17 V5: the `(load case, member)` count map, at most C·m keys, push-grown with its previous table
(HashMov of s(PairBorrowed)) plus an 8 B count per slot. `rules/i95_new_loops.json` retires the per-region row scan
and bounds the region's id loop by Pall. `rules/o10_rules.py`'s E3 also covers the counting pass. U4B-RD's
`INDEX_PIN` binds `Index`'s `operands` map (#104), charged as §8.42's map at the row index's stride. Against §8.46,
four coefficients rise (T17_V5 and its success copy) and only W4 moves, +7,918 B at native atoms. The predecessor
`e8a59f1fd` is reproduced by its own tool cohort. See the governing rules' §8.47.

### §8.49: RT-A3 increment 4, nonlinear supports on the exact route

D1.6 refuses every support with a `nonlinear` record, so validate_profile's nonlinear arm, the producer's nonlinear
rows and the reader's case scan are unreached. `rules/o10_rules.py` adds RTA3NL-S6, which corrects W16C-ES: the S6
context's `carries_fields` call is unconditional, so `validate_profile` → `carries_fields` is charged once per call
(`EDGE_PER_CALL`) instead of zeroed (its closure has no owner or TEXT site). It also adds RTA3NL-RD (the reader's two
D1-empty loops), RTA3NL-ID (a support state, at most 8 B) and the re-read pins (RTA3NL-PIN, and RT-A4-HS's
`recorded_tables.rs`). `base/b7a_scoped_text_model.py`'s `problem` parents move +11 (RTA3NL-PP). Against main
`aa794eb68`, eight coefficients rise (STATICS[1] +754 B; TAV +5,862 B) and every phase moves +6,616 B. See the
governing rules' §8.49 and `tests/test_849_rt_a3_nonlinear.py`.

### §8.60: RT-A3 S6, nonlinear supports on load/reference-state documents and beside connectors (proposed)

D1.5 refuses every `analysis_state`, D1.6 every nonlinear support and D1.4 every component, so S6's resolved-case
motion paths are unreached on D1. One owner is reached: `solve_prepared_load_case` builds the loop's consumed motion,
`CaseMotionEvidence::new(..)`, as `capture_nonlinear_state_evidence`'s argument before the capture returns None.
`base/w16_owners_model.py` prices its one non-empty vector (S6-ME: one u64 per spring, B(8s) per case, CasePrefix);
`rules/o10_rules.py` adds RTA3S6-L (the resolved spring-base loop and the moved-ground map, 0 on D1), rewrites
W16-NS's arms (and excludes `CaseMotionEvidence::resolve`, the modal state case's restatement) and reads RT-A4's
`CombinationExpression::resolve` by its impl, since lib.rs now has two `fn resolve`. Against main `394f4c623` (the kept
§8.58 tree) 16 coefficients rise and none fall (O_base +256 B, STATICS +779 B, +17 Node, +78 Value, TAV +2,718 B);
every native phase rises +18,761 B. The regenerated block, reviewed inputs #8 and #11 and the law and challenge records
have cleared numerical/pricing/preservation scrutiny on the final composition; its one current-source AC-004 attempt
passed all seven once-only tests with verified closure at `1a07aec6b`. Exact-head hosted checks, publication
consultation and integration remain pending. S6 was drafted as §8.59; its source and tool comments
still cite that number, which now means §8.60 (the modal support operations are §8.59). On the final composition
with §8.59's predecessor (`da0afeabb`) the tool prices S6's tree unchanged. See the governing rules' §8.60.

### RT-L2: pressure elongation per load case and the cancellation screen's exception

**The change.** Model document 0.9.0 gains `model_settings.pressure_elongation`, a closed record holding one list of
operating load-case ids, refused at validation without a pressure contract, empty, duplicated or naming no case. The
exact route reads it per case: the straight Poisson coefficient is 2ν or 1, the arc's free-strain push and recovery are
skipped when a case does not include it, the certified wall increment follows, and the formulation basis carries one of
three fixed statements. The ordinary semantics move from pressure-2 to pressure-3; readers refuse a stored pressure-2
result, and a successor whose base is one, by name. `finish_source_groups` gains the cancellation screen's exception:
when the original screen fails, `cancellation_exception` runs the eligibility predicate, the family and the record; the
warning is formed only with a record, and `blocked_envelope` removes it from a blocked document in place. D1.4 gains one
clause: it declines a document whose `operating_load_cases` is longer than its load cases, from the two lengths.

**The list's two bounds (RTL2-K, `base/rtl2_owners_model.py`).** The retained route runs, in order: the closed-field
check and `CapturedInvocation::parse` (the typed list is allocated there), then `admit`'s censuses, then
`domain_clauses` (with the clause), then `law_order`, and only with a permit the permitted run, where validation runs
(once in the ordinary run and once more in a republication). So:
- at parse time the list is bounded by the raw census only (k ≤ RawValues = 16,384; its strings ≤ RawStringCapacity =
  131,072 B). RTL2-TS charges the typed list at that bound in T_resident and in T25.1a (`requested()`'s re-derivation;
  it follows the clause, but its string bytes have no per-string capacity cap below RawStringCapacity, so it is kept at
  the census as an over-bound);
- validation sees k ≤ c ≤ C = 3 (`caps::LOAD_CASES`). RTL2-VS (validation's refusal working set, BuildScratch) and
  RTL2-VL (`rules/o10_rules.py`: the refusal loops, k each, and the diagnostic loop, at most max(2, k + 1)) are priced
  at k = C, the diagnostic loop as C + 1 = 4.
The guard stops the run if the clause is removed or changed, leaves `family_clauses` or loses the load-case cap beside
it, or if the chain that puts it before validation changes (family_clauses ← domain_clauses ← admit before law_order;
law_order's `domain?` before its permit; admit before `permitted_dispatch`; no validation in retained_memory.rs,
source_receipt.rs or model_fields.rs). RTL2-FS (the formulation statement's ≤ 3 named cases) and RTL2-RD (the pressure
reader's key list and statements, T17 V5) do not depend on k.

**Other rules.** Each argument stands beside its rule.
- RTL2-XZ: on D1 every retained case's factor groups are empty (formed only inside the region loop, D1.5's empty E
  regions; L never reaches the producer), so the screen is 0, the failed-screen branch is never entered and
  `cancellation_exception` (the predicate, `family_nodes`, `cap_remainder_record`) and `cap_remainder_warning` are not
  reached; borrowing the empty group map through the loop and dropping it after the screen holds no bytes. The two
  edges from `finish_source_groups` and the four inside `cancellation_exception` (`exception_chains`, the predicate,
  the family, the record) are zero; RTL2-XZ-M zeroes the warning's per-case formatted message
  (`cap_remainder_warning_message`), formed only inside `cap_remainder_warning`, itself reached only with a record. The guard binds the call
  structure (one production call of each, the exception inside the failed-screen branch, the warning inside
  `if let Some(..) = exception`, B5L1's borrowed group loop), not message, code or token text.
- RTL2-RX: the reader's record checks (`assembled_is_group_sum`, `exception_consistency`, `bore_group_value`) and the
  warning check are zero edges: no record and no warning exist on D1; `exception_model_binding` is reached only from
  `reader_verdict`. RTL2-FB: `consistency_bound_ratio`'s and `cap_remainder_record`'s f64 `.min`/`.max` are not
  FallbackError's. RTL2-FV: `keep_finding`'s `finding()` is its closure parameter. RTL2-ID: the texts' argument classes.
- `blocked_envelope`'s `retain` is `Vec::retain` in place (no crate target in the call graph, no allocation); it walks
  the blocked document's diagnostics once.
- B7a scoped TEXT: `problem` delegates to `finding`, which owns the diag sites; `finding`'s count equals `problem`'s on
  D1. B7b-B: the route bases {pressure-3, preview-physics-2} select the B7b route-E rules. M08-VZ reads
  `ExactContract::of` by its impl.

**Carried obligations.** Full-file, prefix and body pins re-read on their diffs and re-bound (B7a domain, its admission
prefix and `family_clauses`, W1 reductions and member loads, batch 5's prefix and `family_clauses`, KD5C4/C5, O4, TIM,
RT-A4-HS); hand-kept current-source line keys carried by diff mapping with unchanged text (B7a domain audits, owner
choices, phase, returning errors, scoped TEXT and its `problem` parents, the `_X` emitter). Batch 5's pressure pin now binds
the whole production text of pressure_runtime.rs, each `#[cfg(test)]` item removed (B5-PR): the text before the first
`#[cfg(test)]`, its former scope, would end at the exception's test probe inside `finish_source_groups` and leave the
production after it unpinned there.

**Outside D1 (not priced by the retained profile).** A pressure region's elongation token and flag, the arc push and
recovery choice, the exception's predicate (its members and their formed directions), family, record and warning, and
the reader's record and model checks all run only on a case with a pressure region.

**Derivation and native checks.** On `19eab0355` (the final source with the third and fourth scrutinies' responses) the four points complete with O-10 0/0; the prune (36) and the
resolutions (8) are main's. Against main `4d29420a0`'s kept tree (reproduced by its own tool), 1,468 of 1,606
coefficients are equal, 138 higher and none lower: D +8 at every point (two validation calls, at most max(2, k + 1) ≤ C + 1 = 4
diagnostics each), STATICS[1] +4,497 B (the pressure-3 table and schema, with the exception record's `remainder_force_n` and its
description; the schema is resident, so its description bytes are priced), O_base +131,212 B with +16,384 s(String), +20 s(&str) and
+6 s((&str,String)) (the typed list at the census; the working set at C), TAV +174,654 B; s((&str,String)) joins the
profile (four coefficients from 0). The block is regenerated with this revision's basis line (1,718 lines) and the
point atoms (150 lines); both `--check`s are equal. At the in-build atoms every phase rises in both modes by W1/W2
+781,195 B, W3 +957,715 B, W4 +937,643 B, W5 +858,755 B, X1 +1,268,703 B, X2 +723,247 B, X3 +694,903 B; W3 stays the
maximum (8,804,212,689 B dense, without R). Every phase of both modes fits the unchanged M, the 0.9 margin and the
upward-rounded 5% TEXT reserve; the minimum headroom is 2,477,313,452 B (dense W3; sparse 2,536,444,796 B, W3), and
L(2,1) and L(1,2) stay below the merged tree in every phase and mode. The law tests' pins move with it (the record,
D_env 28,779 with push capacity 32,768, Text(diag_env) 273,233,168 B = 273,191,600 + 4 · 10,392, reviewed inputs #1
and #10). This tool, with the twelve re-pinned base files restored from main and main's three o10 pin values, every new
rule kept, reproduces main `4d29420a0`'s 1,602 coefficients, its 1,713-line block and its 154-line point atoms exactly.
The source work law takes ΔE = 93 B (the setting's member) in the three mixed requests: each selected case and the
composite publication charge 24E, so each moves by 2,232.

### §8.62: PR-A, the integrity-diagnostic repair (DEL-04-07 *Blocked document*, *Published displacement*)

The tool now prices the reviewed product behaviour of PR-A. No product behaviour or threshold changes. Each rule binds
exact source text and names. Each rule is a positive bound, and none is a zero premise. The arguments stand beside the
rules in `rules/o10_rules.py` and in `base/correction_failure_model.py`, `base/pra_guards.py` and
`base/u4b_u5b_owners_model.py`. `tests/test_862_pr_a.py` holds the mutation tests.

**PRA-IO, the blocked rewrite.** `blocked_envelope` rewrites each integrity diagnostic whose code is CHECKS_PASSED or
SENSITIVE.
- **Loop bound.** The loop's allocating body is bounded by `2*cases`. Only `append_integrity_report`'s one push makes
  such a diagnostic, and it has two calls in the case solve. A product-crate census of every integrity id, code
  literal and rewrite call binds this.
- **Message.** The new message is site-audited: two constants, the prior code (33 B), and the integrity message
  (inherited `message_int`). The old message is charged at its own site, so both strings are counted while they
  coexist.

**PRA-CF, the correction failure.** The failure texts are charged in every case solve that forms a correction. No
premise excludes the failure.
- **Debug text.** It is bounded from the kernel's own types and literals. `RecoveryFailure` is a `#[derive(Debug)]`
  enum over `&'static str`, f64, u32 and fieldless enums. Its longest variant is charged at the longest frame-kernel
  literal, Debug-escaped. It is formed at most once per call, because collect into Result stops at the first Err.
- **Option chain.** It runs once per case solve. The finite scan inside it allocates nothing.
- **Other owned texts.** The cloned failure, the held reason, the sentence and the warning (id, message, one ref) are
  priced at their sites.
- **Sentence check.** `composite_text.py` checks that the sentence stays within the integrity sentence term `S_len`.

**PRA-TR and PRA-NL.**
- PRA-TR binds the trigger's prior read and its two calls, whose one String is priced at its own site.
- PRA-NL is route E's direct `require`. Its message is charged positively at the longest literal.

**U5B-PD-RN, a rename with an explicit correspondence.** `published_nodal_displacements` became
`corrected_nodal_displacements`. Both shapes are pinned by body and exact calls, and exactly one may be present. The
charge `ordinary_published_displacements(N)` is unchanged: at most one N-vector of the owner is live at a time.

**The fail-closed class: OWN-PRES and O10-GATE.**
- **OWN-PRES.** `base/owner_presence.py` registers every presence-gated owner model by epoch. `price.py` stops (exit 4)
  when an owner of the snapshot's latest epoch, or an earlier one, is absent. An older source that holds none of an
  epoch's owners still adds nothing. A model that stops during the census counts as present there: it stops again,
  with its own message, at its pricing step.
- **O10-GATE.** `rules/o10_rules.py` records every `fnkey` lookup. It stops on any absent target that
  `O10_ABSENT_REVIEWED` does not explain by a source fact that holds:
  - a reviewed alternative function is present;
  - the absent function's own distinctive text is gone from its file, so a rename that keeps the code stops;
  - or the file itself is absent (a source before the module).

  No allowance stands on absence alone.
- **FN-VIS.** `fnkey` and the span helpers accept any `pub(...)` scope. The earlier `pub([a-z]+)` missed
  `pub(in crate::structural) fn skew_spring_matrix`, so SKW-D1's zero edge into it was never added. It is added now.
- **Limits.** These cover the owner models' readings and o10's gated blocks. They do not cover arbitrary text
  predicates inside other models; those models' pins stop on change instead.

**CFG-PROD, the production extraction.** `base/cfg_test_items.py` is the one extractor. It removes each exact
`#[cfg(test)]` item, statement or expression-position element, and only that.
- **Why.** The earlier helper (o10's `_production` and batch 5's copy) counted a `{` inside parentheses as the
  element's block. A test statement with a closure (lib.rs's `tests::KD5_RECOVERY_PROBE.with(|probe| {..});`)
  swallowed the production after it, hiding it from the guards that read that text.
- **Same class.** RTL2's and U6's helpers truncated at the first `#[cfg(test)]`; they use the extractor now. Their
  readings were re-checked equal on main 5e71c4d05 and PR-A's source.
- **Prefix pins kept.** KD5's prefix pins keep their registered scope. The production they leave out of
  certified_recovery.rs is bound by B7a's and W1's full-file pins of that file.
- **The scanner, same class.** The scanner that builds the TEXT inventory, loops and call graph
  (`loopscan.blank_test_items`) ended such a statement correctly at its depth-0 `;`. It ran a test-only struct
  field on to the next depth-0 `;`, though. In retained_product.rs, after `ProductCapture`'s `#[cfg(test)]
  trace_fault` field, it blanked the struct's later fields, its `}` and the `REQUESTED_MODE_KIND` constant. Test
  attributes now end where the extractor ends them. Off-feature attributes keep their own rule, which stops at an
  expression position.
  - What the repair moves (both trees): only those field and constant declarations become visible. They hold no
    call, macro, loop, closure or text site. The field sizes are priced by `size_of` probes (T11), the constant's
    literal by the static-literal pass (`rust_literals.production_literals`, its own lexer), and its uses by raw-text
    constant reads.
  - What the visibility moves: `callgraph_g5` reads struct fields from this text to type `self.<field>` receivers.
    With ProductCapture's fields hidden, its impls' field calls (`self.adapter.reserve(..)`, `self.parked.len()`, ..)
    were unknown receivers fanned out by name to every scanned method of that name. Now they resolve to the field's
    type (418 edges fewer, none added). The fan-out reached lib.rs's `ReservedNotice::reserve`, a diagnostic literal,
    from 21 functions; it now has its one real caller. D and the diagnostic and TAV text fall on main and PR-A's
    source alike (an over-approximation removed, not a charge dropped).
  - Test-only code is blanked in addition: the `else` tails of `let x = if .. {..} else {..};` and `use` semicolons.
- **Regressions.** These are in `tests/test_862_pr_a.py`: nested test modules, a test statement inside a function,
  `cfg(not(test))`, attribute stacks, fields, arms, literals, and the real lib.rs case.

### §8.63: the modal completed-set boundary (DEL-04-09, PR 390; candidate)

**What moved.** PR 390 bounds the dense modal entry's typed outcome rows and shapes with fallible storage. The entry's
body moves again, its text unchanged apart from storage, into `run_modal_dense_within` (the caller's completed-set
limits); `run_modal_dense_with_budget` becomes its one-line delegation, as `run_modal_dense` is to it. The three
Sensitive and cut-off texts are now `format_args!(..)` arguments of `ModalCompletedSetStorage::modal_completed_set_format`,
which counts the length by one formatting pass and reserves exactly before writing. resource.rs gains the boundary
(about 480 lines, 27 TEXT rows); the frame kernel gains `RestraintBasis::lift_into`; the modal eigen crate, which is
not a priced crate, gains the `_with` storage variants. No function the retained graph reaches changes: at all four
points the function multiplicities and the reached functions of the changed files are main's, by name.

**What this establishes.** Two tiers, and only these.

*Tier one, the ground for the coefficients.* At all four pricing points, in the graph the tool built, every inventoried
site (MODAL-CS-M0's keys) has its required row count at multiplicity zero, and `lift_into` is unreached from every
retained root (MODAL-CS-UR). If the computed graph reaches a guarded site, the run stops. The four points share one
source discovery and one call resolution, so their agreement shows the result reproduces; it is no independent
evidence about a file or a call edge that all four missed. This tier does not see: a call the graph resolved to a
wrong, non-empty set of targets; a call upstream that the graph missed under another name; and source compiled in by
a route the directory walk does not follow (see **Limits**).

*Tier two, the inventories.* Each inventory below (ST, RR, LI, ID, M0, TM and the include inventory) records what its
scan reads and stops the run when a record differs from the reviewed one. None is a proof about the build: each
says what it records and what it does not, against the table of forms below as its model.

*The model, by effect.* What the checks are for is a later source change that would cost memory on the retained route
without anyone seeing it. Renaming an import or moving a module is ordinary maintenance; for each check the entry
below says whether such a change is made visible (a record changes, so the run stops naming it), is walked (the
graph reads it), or stops the run outright.

**Rules** (`rules/o10_rules.py`, label MODAL-CS; `tests/test_863_modal_completed_set.py`). None adds an owner.
- MODAL-CS-RK: the three dense-entry audits are keyed on the function that holds the body, on their `format_args!(`
  rows (the inventory's macros include it). The run stops unless `run_modal_dense_with_budget` is exactly its one-line
  delegation with `None` limits. §8.58's delegation check of `run_modal_dense` stands. Maintenance: renaming either
  function or editing the delegation stops the run.
- MODAL-ID's whole-file pin of resource.rs is re-pinned. Its diff is the boundary's module docs, the two widened `use`
  lines and the appended boundary; the P0 text is byte-identical, so §8.58's audits stand. Any edit stops the run.
- MODAL-CS-ID: the boundary's arguments the base leaves unclassified, each at a source bound: usize and u64 decimals
  20 B; an expression String 54 B and a bound String 82 B, the longest of the pinned callers' own format strings; the
  refusal's limit String 107 B. Where the base's by-name class already covers the source bound (`actual`, `value`,
  `count`, `symbol`, `error`, `name`, the dimension's text) no audit is added. It reads the pinned resource.rs: it
  records the eleven formats and the two limit arms by their text, and stops when one is gone.
- The censuses (TM, ST, RR, LI) compare exact reviewed inventories, `rules/modal_cs_inventory.json`, each a multiset of
  (file, enclosing function, stripped source text). A mention is a spelling of a tracked name in code (literals and
  comments blanked); a change that adds, alters or removes such a spelling in a file read changes a record and stops
  the run. A use that does not spell the name (an alias's later uses, a glob import, a macro's expansion) records
  nothing. No file is excluded: test code is in the inventories, marked by review. The files read are the `.rs` files
  under the driver's own crate selection (`price.py` `chain_crates`, compiled from its source, so the correct_norm crate
  is included), the modal eigen crate for ST, and the files `#[path]` brings in. A change to `modal_cs_inventory.json`
  requires re-reading the source it describes.
- MODAL-CS-TM: each reviewed test module (`tests_document`, `tests_resource`, modal_quantities' inline `tests`) is declared
  exactly once in its parent as `#[cfg(test)] mod`, and every test entry of an inline module lies inside it. Every
  `#[path]` in the files read, and in the files those attributes bring in, is found by token (any spacing), must be a
  plain string literal that does not name a reviewed test file (compared by file name), and is resolved against its
  declaring file's directory. A target that is not a `.rs` file of the walked set is a reviewed entry (`path_outside`:
  declaring file, literal and its ground) or the run stops. Today there are 88 declarations of 47 files, the crates'
  `tests/` helpers and one benchmark source. The censuses read every such target, with what it brings in, where the
  snapshot has it. Each entry carries the ground it has in this run, so a declaration that moves from one ground to
  another changes its record and stops:
  - test-file (51): the graph's own file filter leaves the declaring file out (a tests, benches, bin or target
    directory, or a file name holding `test`; the filter's two lines in `base/callgraph_g5.py` are pinned).
  - test-module (3): the declaring file is a module with exactly one declaration in the files read, a `#[path]` item
    whose own attributes include `#[cfg(test)]`, in a parent that spells `mod <stem>` once. These are j7_rf_census.rs
    and j7_t4_consequences.rs, both declared so in source_recovery.rs. This is TM's own check, exact by token. J3 and J2
    check the same two declarations by their text and stop first when either loses its `#[cfg(test)]`.
  - test-item (34): the `#[path]` item's own attributes include exactly `#[cfg(test)]`. Any other gate
    (`cfg(any(..))`, `cfg_attr`, a feature) is not this ground.
  - hash (0 today): none of the above, so the entry carries the SHA-256 of its target, and a changed or missing target
    stops.

  The first three grounds rest on the registered build not compiling test code, and on the censuses reading the file.
  They do not bind the target's bytes: an edit to a test helper is walked by the censuses and stops only if it changes
  a record. A `cfg_attr` that sets a path stops. Maintenance: a new `#[path]` module inside the walked crates is walked
  (main `e0e3e3c7`'s two certificate modules are); a declaration that loses its `#[cfg(test)]`, or a new ungated one
  to a target outside the walked set, stops.
- The include inventory (TM): every `include!`, found by token in code in the `.rs` files under the scanned crates'
  directories (src, build scripts, tests, benches; not `target`) and in the files `#[path]` brings in, is a reviewed
  entry (file, enclosing function, line, target). Today there is one: product physics' build.rs line 17, including
  `src/build_identity.rs`. The ground for the entry is that its target is walked: the run stops unless the target is a
  plain literal naming a `.rs` file of the walked set (`src/build_identity.rs` is, and retained_memory.rs also declares
  it as a module by `#[path]`). Any other `include!` stops. `include_str!` and `include_bytes!` read data and are not
  recorded.
- MODAL-CS-ST: a refused request's `structure` is sized 35 B. The inventory records each spelling, in code, of the seven
  storage request functions (definition, call or other mention, with a call's argument at its position) and each
  `ShapeStorageError::` or `Self::` variant construction or pattern, in the product crate, the modal eigen crate and
  what `#[path]` brings in: 85 entries, 62 production, 10 test, 13 of support.rs's own, unrelated `requested_bytes` and
  `SupportResourceError`. A variant imported under another name, or constructed bare, is not recorded. Each production
  call or construction must pass a constant whose name, looked up by spelling in the files read, has exactly one
  all-literal `&str` declaration, or forward its enclosing function's own `structure` parameter; the eight forwarders
  are body-pinned. The 35 B is that reading: of the pinned files whole (modal_quantities.rs, where the requests and
  constants originate, and resource.rs) and of the inventoried request sites elsewhere (outcome.rs, skew_pencil.rs). It
  is not a resolution of Rust names. Maintenance: renaming a constant, a request or a forwarder changes a record or a
  pin and stops; any edit to modal_quantities.rs stops at its pin.
- MODAL-CS-RR: a premise read from the source, not an observation. A response-row `name` has no length in the source.
  The inventories record each spelling of `run_modal_dense_within` in the files read (its definition, the one
  delegation passing `None`, exact by MODAL-CS-RK, and the test callers) and each spelling of
  `ModalCompletedSetLimits`, `ModalResponseRowProduct`, `response_rows` and `ResponseRows` outside the pinned
  resource.rs: outcome.rs's import and parameter type, and test code. Read so, no production spelling passes limits,
  the name is empty there, and the by-name 128 B covers the dimension's text. A value built through an alias's later
  uses or a struct update (`..other`) is not recorded. Maintenance: an import renamed with `as` spells the original name
  and changes a record; moving a caller to another file changes its record; both stop.
- MODAL-CS-LI: `lift_into` has no TEXT site. The inventory records each spelling of `lift_into` and of skew_pencil's
  `modal_lift_within` in the files read: the definitions, `lift_into`'s one production call inside `modal_lift_within`,
  that function's one production call in `run_modal_dense_within`, and the test mentions. `modal_lift_within`'s body is
  pinned. Maintenance: a new caller or a moved one changes a record and stops; a call through a function value bound
  under another name is not recorded, and is left to tier one.
- MODAL-CS-UR (a checker change, `lib/o10_check.py`): `O10_UNREACHED_FNS` names `lift_into` by file and name. At each
  point the checker reads only `edges.json`, the rows of `audit.json` and the roots in `work/text_budget.caps.out.json`,
  and returns pass or stop. It stops if a path in the graph from a retained root reaches the function, if a
  call-shaped token naming it produced no edge (an `audit.json` row, matched by name), if the function is missing or
  ambiguous in the graph, if its crate has no function in the graph, or if a point's graph or roots are missing. A
  false same-name edge into it from a reached function stops the run; it is corrected in the call graph's rules, not
  passed. The checker does not see a token the graph resolved to a wrong, non-empty set of targets, or a call upstream
  under another name; MODAL-CS-LI separately records every spelling of the name in the source it reads. A source
  before the boundary has no `lift_into`; that state is admitted only by its reviewed absence (no `lift_into` text in
  restraint_basis.rs), and the checker then requires that the graph has none.
- MODAL-CS-M0: the boundary's 25 TEXT lines, the dense body's five TEXT rows (the three moved sites and the two cluster
  refusals) and the five audited test rows must stay at multiplicity 0 and present (`o10_check`). Each key also binds its
  exact row count (`O10_MULT0_ROWS`; resource.rs 1402 and 1422 hold two each). The rows counted are the template
  scanner's own (`base/template_inventory.py`, whose rows the pricing reads), run on resource.rs and outcome.rs, so the
  rows bound and the rows priced are one extraction; the dense body's total and its five distinct lines are bound in
  the rules. J3's test-code pattern sizes 0 the four test Debug sites of a whole `ModalOutcome`, which have no source
  bound. Maintenance: any added, moved or removed row in the dense body or the boundary stops.
- The emitter: `lib/rule_ops.o10_check_rules` extracts the existing multiplicity-zero site-list construction from
  `sq2_rules.py` (same order: `O10_SITE_SIZE` keys, then `O10_MULT0_SITES` keys) and adds two inputs, `o10_mult0_rows`
  and `o10_unreached_fns`; the tests drive the same code.
- O10-GATE: three reviewed absences (`run_modal_dense_within`, `modal_completed_set_judge`, `lift_into`) are keyed on the
  boundary's own text. Each is also checked where the function is looked up, so a rename that keeps the text stops there.

**Re-reading modal_quantities.rs when its pin stops the run.** The 35 B bound and the ST inventory rest on these facts
about the file. Re-check each before re-pinning; if any no longer holds, the bound or the inventory needs review, not
only a new digest.
1. Each production storage request (`modal_shape_reserve` outside the `#[cfg(test)]` module; today six) passes one of
   the file's `pub const …: &str = "…"` structure names, spelled directly, with no local binding of that name in scope.
2. Those constants are all-literal, declared once each, and the longest is `CUMULATIVE_FRACTIONS`, `cumulative
   effective-mass fractions`, 35 B (it sizes the cumulative fractions); a longer one raises the bound.
3. `ShapeStorageError` has two variants, each with one `structure: &'static str`; nothing imports a variant under
   another name, glob-imports the enum, or constructs a variant bare, outside the test module.
4. The forwarders `FallibleStorage::modal_shape_reserve`, `reserve_exactly` and `requested_bytes` pass their own
   `structure` parameter unchanged and bind no other `structure` (their body pins are in `_CS_FWD_PINS`).
5. The caller-owned entries are unchanged: the `_with` variants take the caller's `ShapeStorage`, and the test module's
   `CountingStorage` and its entries stay inside `#[cfg(test)] mod tests` (MODAL-CS-TM).
6. No macro, `include!` or `#[path]` brings other source into the file.

**The model the checks were read against.** Rust forms, whether each check records them, and the deciding line of
`rules/o10_rules.py` (at this head).

| Rust form | RR, LI | ST | TM, include | Deciding line |
| --- | --- | --- | --- | --- |
| Method, path or fully qualified call | Yes, by spelling | Yes for the seven requests; the argument by position | n/a | 5376, 5614 |
| Bare function value | Yes (a mention) | Yes (a mention) | n/a | 5376, 5639 |
| `use`, `use … as`, `pub use` | Yes where the original name is spelled; the alias's later uses no | Request names yes; a variant import no (modal_quantities.rs: stops at its pin) | n/a | 5376, 5643, 5600 |
| Type alias or type position | Named types yes; alias-only no | n/a | n/a | 5376 |
| Struct expression: named, alias, `Self` | Named yes; alias and `Self` no (resource.rs pinned) | `ShapeStorageError::`/`Self::` variant yes; alias or bare no (pinned files stop) | n/a | 5376, 5643, 5201, 5600 |
| Local constant shadowing a module constant | n/a | Not resolved; looked up by spelling (pinned files stop) | n/a | 5609, 5600 |
| `name: value`, shorthand, `..other` | Explicit and shorthand yes; `..other` no | `structure` of a recorded request or variant only | n/a | 5376, 5643 |
| Glob import `use m::*` | No | No (pinned files stop) | n/a | 5376, 5600 |
| Macro: code tokens; expansion; string; comment | Code tokens yes; expansion, strings and comments no | Same | n/a | 5376 |
| `#[path]` (any spacing, plain literal) | Target read if walked or reviewed | Same | Followed; an outside target is a reviewed entry with its ground (test-file, test-module or test-item: bytes not bound; hash: bytes bound), else stops; a non-plain literal stops | 5509, 5520, 5528–5544 |
| `#[cfg_attr(…, path = …)]` | n/a | n/a | Stops | 5503 |
| `#[path]` inside an inline module | Resolved against the file's directory (nesting not modelled) | Same | Same | 5509 |
| `include!` | Target must be walked | Same | Reviewed entry, target a walked `.rs`, else stops | 5567, 5569 |
| `include_str!`, `include_bytes!` | Not read (data) | Same | Not recorded | — |
| `#[cfg(...)]`-gated files under src | Walked | Walked | Walked | 5376 |
| A `.rs` file no walk, `#[path]` or `include!` reaches | Not read | Not read | Not read | — |

**Untested stops.** Two MODAL-CS-M0 predicates have no negative test of their own, because another stop fires first
on any source that would reach them: `run_modal_dense_within` being one bodied function (MODAL-CS-RR's census stops on
a second definition first), and the three moved keys being among the dense rows (MODAL-CS-RK's site lookups stop
first). There is no hash-ground test because no entry has that ground today.

**Limits.** The modal eigen crate is in neither the call graph nor the template inventory, so its two
`ShapeStorageError` Display rows have no inherited class. They would need adjudication if that crate ever entered the
inventory. The tool's graph and these inventories read `.rs` files found by walking the crates' directories (and, for
the inventories, what `#[path]` and `include!` bring in); source compiled in by another route is read by nothing here.

**Derivation.** No coefficient moves, so nothing is registered. On `6f6dadbc2` (PR 390's reviewed head `6c6fc70e5`
merged with main `3c8fc26a6`) the four points complete with O-10 0/0. Every guarded row is present at multiplicity 0
with its exact row count, and `lift_into` is found once in each point's graph and is reached by no retained root. The
prune (36), the resolutions (8) and the remapped keys (889) are main's, and no hand-kept key moves. Main `e0e3e3c7`
differs from `3c8fc26a6` in core only by the frame kernel's inverse-norm certificate (two new files and a declaration in
arc_certificate.rs), which nothing here reads as a boundary mention, and in none of the 13 reviewed inputs. Against
main `e0e3e3c7`'s tree, priced by its own tool, all 1,606 coefficients are equal: none is higher and none is lower.
That tree is `3c8fc26a6`'s apart from the basis line. `generate --check` with the registered basis line (1,717 lines)
and `point-atoms --check` (150 lines) are equal on the head and on main. This tool replays main `e0e3e3c7` byte for
byte with one restoration, resource.rs's pin set back to main's value; the reviewed absences admit the rest.
