"""I68 U8-0 probe: the Python reader's classification of the L = 0 successor's body-1 rows."""
import json, sys
from copy import deepcopy
sys.path.insert(0, sys.argv[1])
from core.analysis_runs import retained_precision as rp
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.load(open(f"{sys.argv[2]}/l0_isolated_node_{mode}.json"))
    r = rp.validate_retained_precision(deepcopy(doc["source"]), deepcopy(doc["invocation"]))
    for c in r["classifications"]:
        if "N2" in c["result_id"]:
            print(mode, c["result_id"], c["class"], "normalized", c.get("normalized_bits"), "scale", c.get("scale_bits"), "bound", c.get("bound_bits"))
