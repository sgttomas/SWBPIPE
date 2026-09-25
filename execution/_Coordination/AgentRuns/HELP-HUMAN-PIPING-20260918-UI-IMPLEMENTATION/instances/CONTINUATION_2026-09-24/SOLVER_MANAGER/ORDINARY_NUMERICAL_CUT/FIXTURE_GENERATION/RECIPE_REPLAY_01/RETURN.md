# Maintained fixture recipe replay

Both actual product modes completed successfully through the maintained recipe. The sparse 830-row and dense 832-row fixtures remained byte-identical to their original actual-producer captures, as did the historical fixture. Source/input/lock inventories matched before and after; the new generation record binds the applied recipe and verified source closure. Prior generation-record bytes are retained in `_run_records`.

The recipe does not synthesize headers, quality or values. These are example transport fixtures, not independent physical oracles or broad method qualification. R1/R2 static backcheck is clear; separate transaction fault controls remain pending. No consumer repaired-test pass is implied.

See `_run_records/RESULT.json`, `VERIFIED.json`, stdout/stderr and the executed driver for exact bindings. The accompanying maintained documentation now names the recipe and its bounded atomicity/recovery claims; that documentation delta needs review backcheck.
