# RT-A6 H1 retained-pricing D3: PROVISIONAL, UNADOPTED proposal

Status: an isolated technical proposal for a fresh, independent changed-proof and carrier review. Nothing here is
adopted. It adds no governed section, registration, generated block, law record, witness freeze or run, accepted D3
record, or publication.

## Identities

- **Priced source (immutable):** H1 `925fcf122e6716006bb05d25496a5cda7caa79dd`, core tree
  `197326f74012bfa7967db0186d2b2e75e13e76d1`. Its base is integrated Step0 `e8cfa72d5`.
- **Proposal branch:** `t3/86902ce5` (worktree `/Users/ryan/.t3/worktrees/swbpipe/t3-86902ce5`). Head `a7b6aa180`
  is the merge of origin/main `0df1ec32a` (docs/governance only; it contains Agent 0's `394f4c623`). Its core tree
  equals 925's. Proposal commits: `44ee7027b`, `9f9303345`.
- **Proposal tool cohort:** `a7b6aa180:tools/retained_pricing` (tree `dc37f4b1…`) plus `tools/repository_history.py`,
  extracted by `git archive` into `tool_prop/`.
- **Predecessor tool cohort:** `e8cfa72d5:tools/retained_pricing` (tree `b64f41a8…`; byte-equal at 925), extracted
  into `tool_e8/`.
- **Registered predecessor:** §8.58 (P0/S1 modal resource). Basis line §8.55's. Block 1,713 lines; POINT ATOMS 154 lines.

## Old-basis reproduction (comparison basis only; not final qualification)

`tool_e8 … price.py run e8cfa72d5`: exit 0, O-10 0/0, all four points complete. Line map: 888 remapped, 0 unmapped,
36 prunes, 8 resolutions. D and TAV: L(3,0) 43,783 / 4,714,366,116; L(2,1) 39,373 / 4,103,398,826; L(1,2) 35,239 /
3,578,737,078; E 76,356 / 7,229,420,978. These equal §8.58's values. `generate --check` with the §8.55 basis line
(`basis_855.txt`) is equal (1,713 lines), and `point-atoms --check` is equal (154 lines).

## Unmodified obstacle (reused)

The unmodified tool on 925 stops at exit 4: "B7a formation owner source changed: self_weight.rs". With only the five
full-file pins relaxed (`disc/`, discovery only), the run reaches the points and is incomplete:
- 2 unmapped loop headers and 71 unclassified arguments;
- D +2,130 at every point, from `contents_diag`/`capture` reached at `cases·l`, the by-name `.take(` fan-in and
  `capture`'s false out-edges;
- reach +14.

## Source audit (e8 → 925; PP production files)

| File | Change | D1 reach |
|---|---|---|
| `operating_conditions.rs` | `pub mod contents;`; judge's exact-route refusal spares a contents-only use; `diagnostics.extend(contents::contents_use_findings(model, case))` per case; lower takes `RetainedContents` before the clear and restores it under `Some` | judge and lower are live |
| `operating_conditions/contents.rs` (new) | capture/of_case/of_case_in_place/case_contents_density/binds_contents/contents_use_findings/RetainedContents::take/restore/contents_diag | via judge, lower and inspect |
| `operating_conditions/resolution.rs` | deletes resolve_case's contents refusal push (behind S2-ZR); docs | none new |
| `self_weight.rs` | from_value, the contents-aware generator, contents_evidence_quantity, contents_dependency; generate_operations' contents argument; retained_basis's contents_source check; inspect's of_case_in_place inside the lineage closure | only through the fn_zero set and the closure |
| `modal/tests.rs` | `#[cfg(test)]` | none |

Domain facts (`retained_memory.rs`, unchanged): D1.5 `if case.condition_use.is_some() { return refuse(C::Case,
F::ConditionUse) }`; D1.3 `!m.condition_sets.is_empty()` and any `condition_set_ref`; D1.10 object provenance.

### Multiplicity: heap zero is not call/frame absence

- **judge → contents_use_findings: entered once per case.** Its first statement is a let-else that returns
  `Vec::new()` without allocating. `extend` by an empty Vec reserves nothing. Heap 0; the call and frame stay live.
- **lower → RetainedContents::take: entered once per lower call** (solve and validation). The `filter_map` makes c
  passes and collects zero items. std's `SpecFromIterNested` returns `Vec::new()` at the first `None`, so nothing is
  allocated. `uses.is_empty()` then returns `None` before the sets and pipes collections. Heap 0; the call and frame
  stay live.
- **lower → restore: never entered** (only under `Some`).
- **inspect → of_case_in_place / case_id: never entered on D1.** Every admitted load `continue`s before the lineage
  closure:
  - D1.10 leaves no object provenance, so `method` is `None`; a non-object JSON provenance indexes to Null.
  - D1.7 refuses the deterministic element form and any node load in the generated namespace.
  - A support target is `false`.

  So neither a recognized nor a malformed-legacy record reaches the closure.
- **contents_dependency, contents_evidence_quantity: callers only in the 17 guarded self-weight fn_zero functions**
  (generate_operations, retained_basis).
- **from_value, generate_contents_aware…: no caller in the 15 priced crates.** The callers are `self_weight_wasm` and
  PP's `tests/`. `serde_json::from_value(` is a lowercase module path, so the resolver gives free fns only; this one is
  associated and receiverless. The graph has no in-edge.

### Positive owners

None are added on D1, and no old positive owner or rule moves. The B7a deterministic-identity format stays positively
priced at `cases·l`. Its audit is re-keyed 1303 → 1519, and the text is byte-identical (e8 lines 1298–1308 equal
925 lines 1514–1524).

## Proposed rules (labels in code; `rules/o10_rules.py` "RT-A6 H1" block, from line 6333)

- **Pins.**
  - B7a `self_weight.rs` → `9e7bf2e0…`.
  - S1-PIN and the S1 owners `operating_conditions.rs` → `69041ab7…`.
  - S2-PIN and S2_PINS `resolution.rs` → `97fac9e8…`.
  - H1-PIN `contents.rs` → `8fbf2cc6…`.

  Each comes with an H1 warrant.
- **Premises** (stop on drift independently of the hashes): D1.5, D1.3, cuf-first, cuf-caller, take-sig, take-first,
  lower, tk-names, ns, fv.
- **H1-ZR:** EDGE_ZERO judge→contents_use_findings, lower→take and lower→restore. Each warrant records that the frame
  stays live.
- **H1-NS:** EDGE_ZERO inspect→of_case_in_place and inspect→case_id; SITE_ZERO for the origin `to_string`.
- **H1-L1:** take's `condition_sets` and `conditions` maps, bound 0, each only in `contents.rs:take`.
- **H1-BN/H1-TK (graph truth):**
  - `targets: []` for capture `next`/`is_empty`/`index`, case_contents_density `next` and contents_evidence_quantity
    `is_null`/`get`/`len`.
  - `exclude_targets` (fatal unless witnessed):
    - lower `take` minus `retained_memory.rs`'s free helper;
    - generate_operations `key` minus FK `functionals.rs:key`;
    - 35 by-name `take` callers minus `RetainedContents::take`, each re-checked so that its body never names
      `RetainedContents`.
- **H1-ID:** 36 `contents.rs` audit sites plus two `self_weight.rs` sites.
  - IN128 for ids, keys, set refs and units (the S1-ID precedent).
  - `suffix` = 128+1+14 = 143 B, derived from the source's `at()` and literal suffix inventories, which are guarded.
  - `what` = 14 B; `at("normalize")` = 138 B.
  - Existing by-name classes stay where they lie at or above the true bound: `reason` → error_display, `case` →
    static 2,173 B.
- **H1-FV:** O10_SITE_SIZE 0 for from_value's `other`/`what` rows. A request JSON value's Display has no source bound,
  and o10_check fails the run if the multiplicity is ever positive. This replaces the by-name error_debug class
  (8,192 B), which is not a true bound for a Value.

## Binding evidence (committed cohort `a7b6aa180` on immutable 925)

`proposal_925/`: exit 0, O-10 0/0, all four points complete (no unmapped header, no unclassified argument;
text_complete and D_converged hold).
- **Merged tree vs the e8 reproduction:** identical but for the basis line. Structure equal, 1,602 coefficients equal,
  0 higher, 0 lower. The check is sparse 8,593,926,538 and dense 8,653,057,882 on both (`cmp_925_vs_e8.json`).
- **Points:** each `point.json` equals e8's except the `self_recursive` label `self_weight.rs:928:retained_basis` →
  `:1113:` (same function).
- **Per point, equal to e8:** D, TAV, TEXT requested/moving, D_diagnostics, retained diagnostic bytes,
  sites_with_positive_multiplicity (1,912 / 2,165 / 2,199 / 2,117) and every function multiplicity.
- **Reach:** 4,779 → 4,792 (+13, all at multiplicity 0). TEXT rows 5,053 → 5,118; sites_reached 4,657 → 4,716.
- **Graph:** nodes 5,864 → 5,879; edges 28,052 → 28,111 (+59, −0; `edgediff_final_L30.txt`). 37 H1 exclusions
  witnessed (76 in total).
- **Line map:** old/remapped/unmapped/pruned equal entry for entry; 8 resolutions equal; premise pins and off-build
  equal. Two fn_zero keys re-key by function: dependency_projection 885 → 1070 and stable_mass_per_length 731 → 910.
- **Blocks:** `generate --check` (§8.55 basis line) and `point-atoms --check` against 925's committed
  `retained_memory.rs` and `retained_memory_law_tests.rs` (byte-equal to e8's) are equal at 1,713 and 154 lines.

## Registration / profile / law-build proposal (UNADOPTED)

A conservative carry: no coefficient moves (`a_le_b` and `a_ge_b`).
- Keep the generated block, `REGISTERED_PROFILES` (identity, 9 reader layouts, 12,884,901,888 threshold) and POINT
  ATOMS. Nothing is regenerated.
- All 13 reviewed inputs' digests recomputed at 925 are equal.
- No H1 type is an atom of the block, the point atoms or the reader layouts, and no existing struct or field changed.
  So the in-build atom values and `PINNED_RECORD` are unchanged by source derivation. Ordinary confirmation
  (PROVISIONAL, not AC-004): `cargo test --offline --locked --lib profile_` under `t3_cargo.sh` on head `a7b6aa180`
  (core = 925), with the four flag variables unset. 9 passed, including `profile_in_build_record` (unchanged
  `PINNED_RECORD`) and `profile_point_atoms_in_build` (`law_profile.log`; the build output was deleted afterwards).
- M, the 0.9 margin, R and the TEXT reserve policy are unchanged.
- A governed section number is not assigned here; that reserved choice belongs to Agent 0 or the owner.

## Remaining and not done

- **AC-004 / stack.** The lib-test and challenge binaries compile H1's code, and frames are live (judge/lower/take).
  Coefficient identity does not establish stack depth (§8.39, §8.58). The witnesses are required and not run.
- **Independent review.** A fresh changed-proof and carrier review of this proposal is required.
- **Full tool suite.** Not run in full. The run covered the H1 suite (15), the W15B suite and the affected suites
  (b7a formation, 840, 836, 852, 858, dynamics, c5 callgraph, the root `tests/test_retained_pricing_tool.py`): 198
  passed before the W15B label fix, then 21 passed after it.
- **Tool defect (pre-existing, not changed).** `price.py` `template_inventory` skips any directory whose absolute
  path contains `/bin` or `/tests`. An `--out` named `bind_925` produced an empty inventory; KD5C6 failed closed
  (`failed_inventory_outdir_bin_925/`).
- **Stale README reference.** README's `tests/test_retained_pricing_tool.py` is at the repo root, not under the tool.

## Re-use / re-derive after the real next merged predecessor

1. Price the selected predecessor with its own committed cohort, and reproduce its registered block and point atoms.
2. Merge this branch's three commits onto it. Re-read every pinned file that predecessor changes.
3. A pin mismatch in S1/S2/B7a/H1-PIN, an H1 premise stop or an `exclude_targets` witness failure each means
   re-review: change no value just to pass.
4. Re-run with the committed cohort, out dir named without `/bin` or `/tests`. Compare coefficient-wise with the
   predecessor's tree; check the line map, multiplicities and reach.
5. Re-run the tool tests (wrapped), the law tests and then AC-004 under Agent 0's selection.
