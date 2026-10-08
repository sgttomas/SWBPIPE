#!/bin/bash
# I90: the c = 1 successor documents written by the pin tests' own output variables, base (I1 archive) and head (WT/b1-r).
WT=WT
S=$WT/scratch/i90_b1_sr_rs; P=$WT/b1-r/projects/chirality-piping/fixtures/results
cd $S/pins && shasum -a 256 base/* cand/* > pins.sha256 && cat pins.sha256 | sed "s#^\([0-9a-f]*\)  #\1  #"
for f in base/*; do c=cand/${f#base/}; if cmp -s $f $c; then echo "identical ${f#base/}"; else echo "DIFFERENT ${f#base/}"; fi; done
ls cand | sort > cand.list; ls base | sort > base.list; cmp -s cand.list base.list && echo "same file set ($(wc -l < base.list | tr -d ' ') files)"
for m in sparse_interactive dense_scrutiny; do
  cmp -s cand/u3g2_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal"
  cmp -s cand/retained_precision_l0_successor_$m.json $P/retained_precision_l0_successor_$m.json && echo "fixture l0 $m equal"
done
