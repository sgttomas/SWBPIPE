"""Append reviewer-authored binary64 boundary cases to the SCRATCH copy of the
shared load-reference mutation file (never the reviewed bytes). Expected
outcomes are the reviewer's own IEEE-754 round-to-nearest-even arithmetic.
Integers above 2^53 are refused by the canonical carrier in both languages,
so accepted cases carry the same carrier keys as the committed N2 cases."""
import json, pathlib, sys
from fractions import Fraction
p = pathlib.Path(sys.argv[1])
text = p.read_text()
doc = json.loads(text)
M = "/contract_evidence/load_reference_states/0/members/0/"
def case(cid, op, sel, point, dispatch):
    c = {"id": cid, "source": "pressure-dense", "ops": [
        {"op": "set", "path": M + "operating_temperature_k", "value": op},
        {"op": "set", "path": M + "material_selection_temperature_k", "value": sel},
        {"op": "set", "path": M + "consumed_material_points/0/temperature_k", "value": point}],
        "dispatch": dispatch, "note": "CP4 reviewer scratch case"}
    if dispatch == "accept":
        c["carrier_rust"] = "UNSAFE_JSON_NUMBER"
        c["carrier_python"] = "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE"
    return c
def rne(n):  # reviewer's own round-to-nearest-even of an integer to binary64, via exact rationals
    if n == 0: return 0.0
    e = n.bit_length() - 53
    if e <= 0: return float(n)
    q, r = divmod(n, 1 << e); half = 1 << (e - 1)
    if r > half or (r == half and q & 1): q += 1
    return float(q) * float(1 << e) if q < (1 << 53) else float(q >> 1) * float(1 << (e + 1))
U64 = (1 << 64) - 1
cases = [
    # u64::MAX (i64/u64 path) and u64::MAX+1 (float-parser path) are both 2^64.
    case("R4-u64max-vs-2p64", U64, U64 + 1, float(1 << 64), "accept"),
    # 2^54+2 is a tie between 2^54 and 2^54+4: ties to even gives 2^54.
    case("R4-tie-even-down-2p54", (1 << 54) + 2, (1 << 54) + 2, float(1 << 54), "accept"),
    # 2^54+6 is a tie between 2^54+4 (odd) and 2^54+8 (even): 2^54+8.
    case("R4-tie-even-up-2p54", (1 << 54) + 6, (1 << 54) + 6, float((1 << 54) + 8), "accept"),
    case("R4-tie-even-up-2p54-wrong-neighbour", (1 << 54) + 6, (1 << 54) + 6, float((1 << 54) + 4), "SOURCE_LOAD_REFERENCE_MATERIAL_SELECTION"),
    # Beyond u64 (float-parser path): 2^65+2^12 ties to 2^65; 2^65+3*2^12 ties to 2^65+2^14.
    case("R4-beyond-u64-tie-down", (1 << 65) + (1 << 12), (1 << 65) + (1 << 12), float(1 << 65), "accept"),
    case("R4-beyond-u64-tie-up", (1 << 65) + 3 * (1 << 12), (1 << 65) + 3 * (1 << 12), float((1 << 65) + (1 << 14)), "accept"),
    case("R4-beyond-u64-tie-up-wrong-neighbour", (1 << 65) + 3 * (1 << 12), (1 << 65) + 3 * (1 << 12), float((1 << 65) + (1 << 13)), "SOURCE_LOAD_REFERENCE_MATERIAL_SELECTION"),
    # Mixed integer spellings of one binary64 across the three fields.
    case("R4-three-spellings-one-binary64", (1 << 70) + 1, (1 << 70) - 1, float(1 << 70), "accept"),
]
for c in cases:  # self-check the reviewer's expectations against exact rounding
    ops = [o["value"] for o in c["ops"]]
    vals = [rne(v) if isinstance(v, int) else v for v in ops]
    assert (vals[0] == vals[1] == vals[2]) == (c["dispatch"] == "accept"), (c["id"], vals)
ids = {c["id"] for c in doc["cases"]}
assert not ids & {c["id"] for c in cases}
doc["cases"].extend(cases)
p.write_text(json.dumps(doc, indent=1) + "\n")
print("appended", len(cases))
