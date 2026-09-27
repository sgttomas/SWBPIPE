"""RV4: verify GATE/FORMATION_EXCEPTIONS.json at 759dccf35 against its predecessor (759dccf35~1).
Usage: check_emptied.py <previous.json> <current.json>   (standard library)"""
import hashlib, json, sys
p = json.load(open(sys.argv[1])); n = json.load(open(sys.argv[2]))
e = n["emptied"]["pinned_before_s11g"]
print("previous counts", p["counts"], "rows", len(p["rows"]))
print("current counts", n["counts"], "rows", len(n["rows"]))
print("rows_sha256 recomputed", hashlib.sha256(json.dumps(p["rows"]).encode()).hexdigest(), "recorded", e["rows_sha256"])
print("owners equal", p["owners"] == e["owners"], "counts equal", p["counts"] == e["counts"], "source equal", p["source"] == n["source"])
