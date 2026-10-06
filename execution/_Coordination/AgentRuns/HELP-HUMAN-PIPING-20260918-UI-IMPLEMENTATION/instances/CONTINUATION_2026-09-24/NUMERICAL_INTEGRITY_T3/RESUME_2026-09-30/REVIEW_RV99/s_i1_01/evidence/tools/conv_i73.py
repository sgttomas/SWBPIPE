"""Convert I73's shared case file into RV99's harness format (formula numbers as
hex, RV99's own samples over each input enclosure), so the same 83 cases are
re-checked by RV99's oracle and parity checker, independent of I73's expected
values."""
import json, struct, sys
sys.path.insert(0, __import__("os").path.dirname(__file__))
from gen_rv99 import samples_for, product, to_hex_tree, table_args
from rv99_oracle import my_enclosure
src, out = sys.argv[1:3]
d = json.load(open(src))
f = lambda h: struct.unpack(">d", int(h, 16).to_bytes(8, "big"))[0]
def floats(n):
    if isinstance(n, dict):
        return {k: (float(v) if k in ("value", "argument", "result") and isinstance(v, (int, float)) and not isinstance(v, bool) else floats(v)) for k, v in n.items()}
    if isinstance(n, list):
        return [floats(v) for v in n]
    return n
cases = []
for c in d["cases"]:
    formula = floats(c["formula"])
    ins = c["inputs"]
    encs = [my_enclosure(f(i["value_bits"]), f(i["bound_bits"])) or (f(i["value_bits"]),) * 2 for i in ins]
    lists = [samples_for(e, extra=table_args(formula, []) + [f(i["value_bits"])]) for e, i in zip(encs, ins)]
    tuples = product(lists) if lists else [[]]
    cases.append({"id": "i73_" + c["case_id"], "formula": to_hex_tree(formula),
                  "inputs": [{"id": i["variable_id"], "value": i["value_bits"], "bound": i["bound_bits"],
                              "dimension": i["dimension"], "unit_ref": i["unit_ref"]} for i in ins],
                  "samples": [["0x" + struct.pack(">d", x).hex() for x in t] for t in tuples],
                  "bisect": len(ins) == 1})
json.dump({"eval": cases, "runner": []}, open(out, "w"))
print(len(cases), "converted;", sum(len(c["samples"]) for c in cases), "samples")
