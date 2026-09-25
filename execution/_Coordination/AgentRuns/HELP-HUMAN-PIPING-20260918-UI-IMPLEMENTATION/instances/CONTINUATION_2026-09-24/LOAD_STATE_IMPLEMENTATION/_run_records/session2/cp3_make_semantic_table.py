"""Derive the load-reference-1 semantics table from the pinned physics-1 table.

Rows, vocabulary and hash vectors are carried unchanged: every signature the
0.4.0 route emits is already a physics-1 signature (checked by
check_emitted_signatures below against the frozen raw fixtures). Only the
contract identity, inheritance, profile, evidence policy and limitations
change. Output: json.dumps(indent=2) + newline, the physics-1 convention.
"""
import glob, hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])  # WORKING_ROOT
physics_path = root / "fixtures/results/semantic_contract_v0_3_physics_1.json"
physics_bytes = physics_path.read_bytes()
physics_sha = hashlib.sha256(physics_bytes).hexdigest()
assert physics_sha == "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc", physics_sha
table = json.loads(physics_bytes)
keys = {(r["kind"], r["unit"], r["component"]) for r in table["rows"]}
emitted = set()
for path in sorted(glob.glob(str(root / "fixtures/product_preview/load_reference/*.raw.json"))):
    raw = json.load(open(path))
    for row in raw["results"]:
        emitted.add((row["kind"], row["unit"], (row.get("metadata") or {}).get("component")))
missing = sorted(e for e in emitted if e not in keys)
assert not missing, missing
table["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/load-reference-1"
table["inherited_semantic_contract_sha256"] = physics_sha
table["formulation_profile_id"] = "resolved_straight_load_state_v1"
table["contract_evidence_policy"] = "closed_finite_case_member_material_geometry_region_result_extrema_and_load_reference_state_binding_v1"
table["reserved_inactive_successors"] = table["reserved_inactive_successors"] + [
    "openpipestress.result_semantics/0.3.0/load-reference-source-1"
]
table["supported_profile_limitations"] = [
    "Straight circular members on the exact straight-pressure route with linear restraints or springs only; one resolved case per load case supplies each member's selected E/nu (derived G), temperatures and explicit expansion definition.",
    "Thermal and fit reference strain enter once as an axial eigenstrain; global rigid support translations/rotations are prescribed absolute boundary values with reactions from the unreduced equations.",
    "Ordinary applied loads are exactly the declared source ledger; hydrostatic head, contents weight, per-case mass/support selection, spring base motion and device preload/lock states are unsupported.",
    "Independent equilibrium only; no installation, contact or predecessor history; direct load cases only, combinations and state differences unsupported.",
    "Retained-source recovery is not joined for these inputs; normal-stress maxima are bounded elastic normal stress, not code or equivalent stress.",
    "Source evidence and numerical standing do not authenticate a producer or grant engineering acceptance.",
]
out = root / "fixtures/results/semantic_contract_v0_3_load_reference_1.json"
text = json.dumps(table, indent=2) + "\n"
out.write_text(text)
print(json.dumps({"emitted_signatures": len(emitted), "missing_from_physics_1": missing,
    "physics_1_sha256": physics_sha, "load_reference_1_sha256": hashlib.sha256(text.encode()).hexdigest()}, indent=1))
