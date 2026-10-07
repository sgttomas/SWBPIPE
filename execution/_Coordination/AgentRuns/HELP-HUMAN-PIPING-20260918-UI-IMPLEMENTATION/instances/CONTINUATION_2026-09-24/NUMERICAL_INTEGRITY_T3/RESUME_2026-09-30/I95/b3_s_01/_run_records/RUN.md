# I95 B3-S: how the pricing was run

Python 3.13 from VENV (each script puts `VENV/bin` first on PATH, as I82's did); Git reads with `GIT_OPTIONAL_LOCKS=0`; no cargo, no native job, no install, no Git write. Scratch: `WT/scratch/i95_b3_s/` (TMPDIR = its `tmp/`). Placeholders as in STUDY.md. Every script takes `I95_WT=<WT>`.

**Order** (each from `WT/scratch/i95_b3_s/`, the scripts copied there as `tools/`):

```sh
I95_WT=WT tools/b3s_snapshot.sh            # git archive of main 2007709549 (core, schemas, fixtures) -> snap/
I95_WT=WT tools/b3s_tree_check.sh > b3s_tree_check.out.txt
I95_WT=WT tools/b3s_text_base.sh           # step 0: u4_g7_06's chain, rules line-mapped Pass A -> bd6b4be2c3 (I82's), run_text_part2.sh -> rr/, base/
VENV/bin/python R/I82/b1_cap_study_01/_run_records/b1_mc_chain.py rr mc_chain    # I82's multi-case chain, unchanged
VENV/bin/python tools/b3s_census.py snap/projects/chirality-piping out/b3s_census.out.json
I95_WT=WT tools/b3s_build_chains.sh        # ur, urc, er, erc from mc_chain (b3s_exact_chain.py)
I95_WT=WT tools/b3s_sweep.sh > out/sweep.log    # mc_chain, ur, urc, er, erc at c = 1, 2, 3 (D1's caps, a = c, L = c*l)
VENV/bin/python tools/b3s_exact_chain.py mc_chain erc_nocensus --credits --exact-only --edges base/edges_base.json
I95_WT=WT CHAIN=erc_nocensus tools/b3s_run.sh c3 '{"l": 128, "c": 3}'          # the c = 3 attribution point
VENV/bin/python tools/b3s_report.py runs R/I82/b1_cap_study_01/_run_records R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt out/b3s_report.out.json
VENV/bin/python tools/b3s_rules_mult.py runs out/b3s_rules_mult.out.json
VENV/bin/python tools/b3s_open_items.py runs out/b3s_open_items.out.json
(for v in ur urc er erc erc_nocensus; do diff -ru mc_chain $v > out/mc_chain_to_$v.diff; done)
```

`base_reproduction.txt` was written from step 0's outputs (`cmp` against I65's u4_g7_01 Pass A `text_g7/` and I82's `profile_trees/`, and I65's `text_row_diff.py` on the four text budgets). `b3s_text_delta.py` is the per-function TEXT delta used while building the rules (no recorded output).

**Inputs (committed, unchanged):** I65's `u4_g7_06/_run_records/chain/` (per-file sum list `8761920d…`), `g7_linemap.py`, `text_row_diff.py`, `premise_pins.json`; I82's `b1_mc_chain.py`, `b1_eval.py`, `profile_trees/`; I72's `law_record.txt`; the snapshot's `fixtures/product_preview/physics_source/`.

**Run times:** step 0 about 20 s; one point 15–70 s; the sweep 3 min 47 s.

**Scratch** (`WT/scratch/i95_b3_s/`: `snap/` 83 MB, `rr/`, `base/`, the chain copies and `runs/`) is disposable; these scripts recreate it.
