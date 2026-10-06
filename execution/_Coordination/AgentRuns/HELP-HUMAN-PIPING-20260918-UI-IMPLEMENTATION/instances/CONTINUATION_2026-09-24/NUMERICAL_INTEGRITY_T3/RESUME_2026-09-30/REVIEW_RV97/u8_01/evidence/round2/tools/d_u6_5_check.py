"""RV97 round 2: D-U6-5 for the embedded bases, as ruled (value equality with the pinned fixture).
A strict parallel walk (key order, types, -0.0, float bits) of each corpus base's source and invocation
against the fixture file's, plus the textual spelling differences of the numbers. Usage: d_u6_5_check.py P"""
import hashlib, json, math, re, struct, sys
from pathlib import Path
P = Path(sys.argv[1])
c = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
text_corpus = (P / "fixtures/results/retained_precision_cases.json").read_text()
bits = lambda x: struct.pack(">d", x).hex()
def walk(a, b, path, diffs):
    if type(a) is not type(b):
        diffs.append((path, "type", type(a).__name__, type(b).__name__)); return
    if isinstance(a, dict):
        if list(a) != list(b): diffs.append((path, "keys/order")); return
        for k in a: walk(a[k], b[k], path + [k], diffs)
    elif isinstance(a, list):
        if len(a) != len(b): diffs.append((path, "length")); return
        for i, (x, y) in enumerate(zip(a, b)): walk(x, y, path + [i], diffs)
    elif isinstance(a, float):
        if bits(a) != bits(b): diffs.append((path, "float bits", bits(a), bits(b)))
    elif a != b: diffs.append((path, "value", a, b))
num = re.compile(r'(?<=[\[:,\s])-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?(?=[\],}\s])')
for i in (15, 16):
    base = c["cases"][i]
    raw = (P / base["provenance"]["fixture"]).read_bytes()
    fx = json.loads(raw)
    diffs = []
    for key in ("source", "invocation"):
        walk(base[key], fx[key], [key], diffs)
    # Spelling: the fixture's own text against the same values written in the corpus's format.
    fx_tokens = num.findall(raw.decode())
    corpus_like = json.dumps({"id": fx["id"], "source": fx["source"], "invocation": fx["invocation"]}, indent=2)
    # Compare number tokens in document order (the fixture's key order is id, invocation, source; re-dump in the fixture's order).
    corpus_like = json.dumps({k: fx[k] for k in fx}, indent=2)
    c_tokens = num.findall(corpus_like)
    spell = [(a, b) for a, b in zip(fx_tokens, c_tokens) if a != b]
    same_values = all(float(a) == float(b) for a, b in spell)
    print(json.dumps({"case": base["id"], "fixture_sha256": hashlib.sha256(raw).hexdigest(), "pinned": base["provenance"]["fixture_sha256"],
        "strict_walk_differences": diffs, "number_tokens": [len(fx_tokens), len(c_tokens)], "spelling_differences": len(spell),
        "examples": spell[:4], "all_same_binary64": same_values, "id_equal": fx["id"] == base["id"]}))
