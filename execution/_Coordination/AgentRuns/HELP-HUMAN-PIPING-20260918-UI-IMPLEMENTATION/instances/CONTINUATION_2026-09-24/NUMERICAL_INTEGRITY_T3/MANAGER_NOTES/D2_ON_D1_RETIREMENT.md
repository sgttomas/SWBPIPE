# D2's advance note on D1's retirement option (D-4 A)

Recorded by the T3 manager, 2026-09-26, from D2's SendMessage after its return was committed (`e8ab6c87d`). D2 has **not** revised its design; it will revise only to ROOT's rulings after V1. This note is input for V1 and ROOT, not a design change.

If ROOT selects D1's D-4 option A (retire exact-block selection for fresh solves), D2 expects:

- **I-4 answered "no".** The joined identity `load-reference-source-1` stops being fresh. Slice S-E (joined eligibility, the route test and route carriers, the TS port) would qualify an identity that fresh solves no longer publish, so S-E becomes optional. The alternative is to make `load-reference-source-1` historical-only, with a standing reason and removal from the fresh sets, as S-F does for source-blocks-1.
- **S-D (the fallback)** becomes mostly unreachable for fresh solves: harmless, of little value. Once no fresh solve selects exact-block recovery, the `Err` guard is unreachable anyway.
- **S-F folds into D1's retirement.** The source-blocks-1 standing, fresh-set and text changes (S-3, DD-7) still apply. The I-3 gate becomes "D1's method covers the retired domain on every route".
- **Unaffected:** S-A, S-B, S-C and the transport split (DD-8). Residuals R-1a and R-1b then close by retirement.
