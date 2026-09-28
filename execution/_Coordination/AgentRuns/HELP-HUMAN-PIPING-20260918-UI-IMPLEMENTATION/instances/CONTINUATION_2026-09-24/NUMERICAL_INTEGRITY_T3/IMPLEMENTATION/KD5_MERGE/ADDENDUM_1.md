# K-D5 merge record, addendum 1 (2026-09-27)

`RECORD.md` in this folder is committed as written and is not edited. This addendum supersedes one statement in it, and in PR #1017's body.

- **Superseded:** "An S11-G performance finding (not K-D5's): roughly +15–20% on dense 1000-member solves, from non-interleaved runs." (`RECORD.md`, Evidence, Timing.)
- **Why:** the interleaved comparison of pre-S11-G main `72d5ff864` against `b24b3d536` (S11-G merged), on RF-LARGE-CHAIN-n01000-AX and RF-LARGE-TREE-n01000-ROT, dense and captured, shows S11-G's cost within noise: −0.4% (CHAIN-AX) and +0.9% (TREE-ROT), against a same-binary spread of up to 1.8%. It used probes built fresh from `git archive` copies and alternating runs on a quiet host. The records are in `IMPLEMENTATION/S11G_TIMING/`. The earlier figure compared the S11-G probes against I3's non-interleaved gate runs, which had different builds and host conditions.
- **Unchanged:** K-D5's own cost finding (no measurable cost), which rested on interleaved runs from the start.
- **Method lesson (ROOT):** performance claims come only from interleaved runs of probes built fresh from archives.
