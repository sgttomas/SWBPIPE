#!/usr/bin/env python3
"""I69 U8-2: stage snapshot 07l = 07k plus the two producer-solved L = 0 bases and their entries.

Usage: build_07l.py P_ROOT OUT_CORPUS
- Bases (appended after the 15 existing cases, so no index moves): `u8_l0_isolated_node_<mode>`, whose
  `source` and `invocation` are the PP-pinned live successor fixtures'
  (`fixtures/results/retained_precision_l0_successor_<mode>.json`, sha256-checked: D-U6-5), with
  case-level provenance naming the producer, the entry, the registered build identity and the U8 head
  (decision 6 of ROOT's "I61's U8 plan ruled ..." ruling).
- `expected_classifications`: the accepted Python reader's output on the unedited base, accepted only
  after it is checked against (a) the producer's own claims in the receipt (the absolute_verified roster
  with its bounds, the empty not_covered roster, the input-derived DOFs), (b) I68's three-reader
  observation (25/78/9/1 sparse, 25/78/9/2 dense) and (c) the pinned milestone successor's rows, which
  body 0 must equal row for row (I68: body 0 is bit-identical to the milestone), with body 1 = 6
  input-derived plus 9 exact-zero absolute rows.
- `expected`: eligible with the invocation (U7; I68's probe: all three readers PASS eligible).
- Entries: `entries_07l.py` (SNAPSHOT_05_PLAN s1.2 plus I69's discriminating stop pair), both modes;
  mutations appended after mutation 278 and must-pass entries after entry 24.
- The top-level provenance claim becomes decision 6's text; everything else is byte-identical
  (asserted), in the file's own format (json indent=2 plus a newline).
"""
import collections, hashlib, json, sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P)); sys.path.insert(0, str(Path(__file__).parent))
from core.analysis_runs import retained_precision as rp
import entries_07l as E

CORPUS_07K = "482449bfee58e3193937deaed618ff50245895713be6c4a47772671dce0a6167"
U8_HEAD = "d44909708529c6277fc1fd3b22997218296c8dd3"
ENTRY = "run_linear_static_preview_value_with_retained_direct"
IDENTITY = ("v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;"
            "rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;"
            "target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;"
            "pkg=open_pipe_stress_product_physics@0.2.0")
PINS = {  # I68 U8-1 (R/I68/u8_witnesses_01/RETURN.md s2; PP retained_facade_tests.rs at d449097085)
    "sparse_interactive": ("93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876",
                           "c00cbe76954e5188c63b0ef69738cd40a8db15d303b1113a86524e6c3119dd72", [25, 78, 9, 1]),
    "dense_scrutiny": ("dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88",
                       "0b4250c8139ba25ab9d35fc2d443a01d85de943a8e3d0eb9193f5dd5061ce994", [25, 78, 9, 2]),
}
CLAIM = "synthetic controls plus listed producer-solved bases; no native Current evidence"
KINDS = ("relative_verified", "absolute_verified", "input_derived", "non_quantity")

corpus_path = P / "fixtures/results/retained_precision_cases.json"
raw = corpus_path.read_bytes(); data = json.loads(raw); before = deepcopy(data)
assert hashlib.sha256(raw).hexdigest() == CORPUS_07K, "07k"
assert json.dumps(data, indent=2).encode() + b"\n" == raw, "format"
assert (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])) == (15, 278, 24), "07k counts"
assert data["provenance"] == {"kind": "synthetic_control", "claim": "Arithmetic and reader controls only; no producer execution or native Current evidence."}

new_cases, new_mut, new_mp, extra_mut, extra_mp = [], [], [], [], []
for mode, (file_sha, receipt_sha, counts) in PINS.items():
    rel = f"fixtures/results/retained_precision_l0_successor_{mode}.json"
    fraw = (P / rel).read_bytes()
    assert hashlib.sha256(fraw).hexdigest() == file_sha, rel
    doc = json.loads(fraw)
    assert set(doc) == {"id", "source", "invocation"} and doc["id"] == f"u8_l0_isolated_node_{mode}"
    assert doc["source"]["retained_precision"]["receipt_sha256"] == receipt_sha
    assert doc["invocation"]["solver_mode"] == mode
    source, invocation = doc["source"], doc["invocation"]
    v = rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
    assert rp._validate_draft(deepcopy(source), deepcopy(invocation)) == v
    expected = {k: v[k] for k in ("invocation_bound", "numerical_eligible", "standing")}
    assert expected == E.ELIGIBLE, expected
    classes = v["classifications"]
    # (a) the producer's own claims
    sel = source["retained_precision"]["body"]["cases"][0]["selection"]
    assert [{"result_id": c["result_id"], "bound": c["bound_bits"]} for c in classes if c["class"] == "absolute_verified"] == sel["absolute_verified"]
    assert sel["not_covered"] == [] and not any(c["class"] == "not_covered" for c in classes)
    assert [c["result_id"] for c in classes] == [r["id"] for r in source["results"]]
    dofs = {(d["node_id"], d["component"]) for d in sel["input_derived_dofs"]}
    disp = {f"result:disp:{n}:{c.lower()}" for n, c in dofs}
    assert {c["result_id"] for c in classes if c["class"] == "input_derived"} == disp
    # (b) I68's three-reader observation
    assert [sum(c["class"] == k for c in classes) for k in KINDS] == counts, mode
    # (c) the pinned milestone successor, row for row; body 1 = 6 input-derived + 9 exact zeros
    mdoc = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    mclasses = {c["result_id"]: c for c in rp.validate_retained_precision(deepcopy(mdoc["source"]), deepcopy(mdoc["invocation"]))["classifications"]}
    body0 = [c for c in classes if c["result_id"] in mclasses]
    body1 = [c for c in classes if c["result_id"] not in mclasses]
    assert len(body0) == len(mclasses) and all(c == mclasses[c["result_id"]] for c in body0)
    assert len(body1) == 15 and all(":N2" in c["result_id"] for c in body1)
    assert collections.Counter(c["class"] for c in body1) == {"input_derived": 6, "absolute_verified": 9}
    assert all(c["normalized_bits"] == E.ZERO and c["bound_bits"] in (None, E.ZERO) and c["scale_bits"] in (None, E.ZERO) for c in body1)
    base_id = doc["id"]
    new_cases.append({
        "id": base_id,
        "provenance": {
            "kind": "producer_solved",
            "producer": deepcopy(source["producer"]),
            "entry": ENTRY,
            "build_identity": IDENTITY,
            "u8_head": U8_HEAD,
            "solver_mode": mode,
            "fixture": rel,
            "fixture_sha256": file_sha,
            "receipt_sha256": receipt_sha,
            "pinned_by": ["core/product_physics/src/retained_facade_tests.rs u8_l0_isolated_node_publishes_pinned_successor",
                          "core/product_physics/src/retained_facade_tests.rs u8_d_u6_5_l0_fixtures_are_the_live_successors"],
        },
        "source": deepcopy(source),
        "invocation": deepcopy(invocation),
        "expected_classifications": classes,
        "expected": expected,
        "qualification": ("PRODUCER-SOLVED (U8, L = 0): the PP-pinned live successor of the milestone request "
                          "(fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json) plus node N2 at (3, 0, 0), "
                          "referenced by no member, with support rigid:N2 restraining UX, UY, UZ, RX, RY and RZ: body 1 is one "
                          "memberless, fully restrained node (extent 0; coverage stop [F,F,F,F], has_data false). Published by "
                          "the actual Direct entry in the registered dev/test build; source and invocation are D-U6-5 copies of "
                          f"{rel}. Not native Current evidence; the public reader accepts it, eligible with its invocation (U7)."),
    })
    m, mp, xm, xmp = E.entries(base_id, mode, source)
    new_mut += m; new_mp += mp; extra_mut += xm; extra_mp += xmp

data["cases"] += new_cases
data["mutations"] += new_mut + extra_mut
data["must_pass"] += new_mp + extra_mp
data["provenance"] = {"kind": before["provenance"]["kind"], "claim": CLAIM}
# Unchanged: every existing entry, in place; only the claim text and appended entries differ.
assert data["cases"][:15] == before["cases"] and data["mutations"][:278] == before["mutations"] and data["must_pass"][:24] == before["must_pass"]
for key in data:
    if key not in ("provenance", "cases", "mutations", "must_pass"):
        assert data[key] == before[key], key
assert len({e["id"] for e in data["mutations"] + data["must_pass"]}) == len(data["mutations"]) + len(data["must_pass"])
assert len({c["id"] for c in data["cases"]}) == len(data["cases"])
text = json.dumps(data, indent=2) + "\n"
# The appended bases' JSON values are exactly the fixtures' (key order, types, signs and float bits).
for case in new_cases:
    fdoc = json.loads((P / case["provenance"]["fixture"]).read_bytes())
    for k in ("source", "invocation"):
        assert json.dumps(json.loads(text)["cases"][[c["id"] for c in data["cases"]].index(case["id"])][k]) == json.dumps(fdoc[k])
# The old bytes are a prefix-preserving edit: only the claim line and the appended entries change.
OUT.write_text(text)
print("staged", OUT, (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])),
      "k =", len(data["mutations"]) - 278, "j =", len(data["must_pass"]) - 24, "sha256", hashlib.sha256(text.encode()).hexdigest())
