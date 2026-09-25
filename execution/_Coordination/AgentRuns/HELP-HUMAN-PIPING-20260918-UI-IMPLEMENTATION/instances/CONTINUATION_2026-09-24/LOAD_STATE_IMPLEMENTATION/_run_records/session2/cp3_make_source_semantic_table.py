"""Derive the load-reference-source-1 table from the pinned physics-source-1
table (ROOT-reserved identity; active only once its join candidate passes
independent review).

Rows, vocabulary, hash vectors and signature policies are carried unchanged:
every signature the joined route emits is a physics-source-1 signature,
including its declared source basis (check below against the producer-generated
joined raw fixtures). Identity, inheritance, profile, evidence policy, receipt
policy and limitations change. Output: json.dumps(indent=2) + newline.
"""
import glob, hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])  # WORKING_ROOT
def pinned(name, sha):
    data = (root / f"fixtures/results/{name}").read_bytes()
    assert hashlib.sha256(data).hexdigest() == sha, name
    return data
source_bytes = pinned("semantic_contract_v0_3_physics_source_1.json", "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1")
pinned("semantic_contract_v0_3_load_reference_1.json", "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d")
table = json.loads(source_bytes)
declared = {}
for r in table["rows"]:
    # A signature may be declared more than once with distinct source bases.
    declared.setdefault((r["kind"], r["unit"], r["component"]), []).append(r.get("source_basis"))
emitted, missing = set(), []
for path in sorted(glob.glob(str(root / "fixtures/product_preview/load_reference_source/*.raw.json"))):
    raw = json.load(open(path))
    assert raw["producer"]["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/load-reference-source-1", path
    for row in raw["results"]:
        meta = row.get("metadata") or {}
        key = (row["kind"], row["unit"], meta.get("component"))
        emitted.add(key)
        if key not in declared:
            missing.append(key)
        elif None not in declared[key]:
            allowed = [b for basis in declared[key] for b in (basis if isinstance(basis, list) else [basis])]
            if meta.get("basis") not in allowed:
                missing.append(key + (meta.get("basis"),))
assert not missing, sorted(set(missing))
table["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
table["inherited_semantic_contracts"] = [
    {"id": "openpipestress.result_semantics/0.3.0/physics-source-1", "sha256": hashlib.sha256(source_bytes).hexdigest()},
    {"id": "openpipestress.result_semantics/0.3.0/load-reference-1", "sha256": "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d"},
]
table["formulation_profile_id"] = "resolved_straight_load_state_source_v1"
table["contract_evidence_policy"] = "closed_physical_case_and_load_reference_state_evidence_cross_bound_to_actual_invocation_resolved_case_and_source_receipt_v1"
table["source_block_policy"] = "LOAD-REFERENCE-SOURCE-1"
table["supported_profile_limitations"] = [
    "Straight circular members on the exact straight-pressure route with linear restraints or springs only; one resolved case per load case supplies each member's selected E/nu (derived G), temperatures and explicit expansion definition.",
    "A case whose ordinary attempt is sensitive or rejected may publish its retained-source response only when the source adapter closes on that case's resolved state: member pairs reach formation, each eigen axial load E_member*A_s*eps* is identified once and removed once, prescribed support motions own their rigid DOFs, and the pressure-region inventory is explicitly empty.",
    "Ordinary and retained-source cases keep separate selected methods and complete case/row/evidence coverage; every ordinary case must pass its checks, and one case cannot promote another.",
    "Ordinary applied loads are exactly the declared source ledger; hydrostatic head, contents weight, per-case mass/support selection, spring base motion and device preload/lock states are unsupported. Independent equilibrium only; combinations unsupported.",
    "Retained endpoint maxima bound source-action projection and ordered binary64 arithmetic using represented section operands; they are bounded elastic normal stress, not code or equivalent stress, and do not certify primitive geometry or material accuracy.",
    "Source receipt, resolved-case replay and physical evidence never replace actual invocation custody, numerical standing or responsible engineering review.",
]
out = root / "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"
text = json.dumps(table, indent=2) + "\n"
out.write_text(text)
print(json.dumps({"emitted_signatures": len(emitted), "physics_source_1_sha256": hashlib.sha256(source_bytes).hexdigest(),
    "load_reference_source_1_sha256": hashlib.sha256(text.encode()).hexdigest()}, indent=1))
