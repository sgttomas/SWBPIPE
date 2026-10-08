"""I99 B3-W control: the value route's output for n05, n06 and fields (this probe, main
0b6c5d7362) against the committed physics-source raw fixtures, compared as JSON values.
usage: compare_committed_raw.py <out dir> <P/fixtures/product_preview/physics_source>"""
import json, sys, pathlib
out, ps = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
bad = 0
for name in ["n05", "n06", "fields"]:
    for mode in ["sparse_interactive", "dense_scrutiny"]:
        mine = json.loads((out / f"plain_{name}_{mode}.json").read_bytes())
        committed = json.loads((ps / f"{name}-{mode}.raw.json").read_bytes())
        same = mine == committed
        bad += not same
        print(f"{name} {mode}: equal to the committed {name}-{mode}.raw.json as a JSON value: {same}")
print("TOTAL unequal:", bad)
