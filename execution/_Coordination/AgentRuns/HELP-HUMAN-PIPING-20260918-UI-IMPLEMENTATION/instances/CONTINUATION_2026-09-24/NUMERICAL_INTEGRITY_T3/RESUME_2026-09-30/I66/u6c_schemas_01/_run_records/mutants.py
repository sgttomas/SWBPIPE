#!/usr/bin/env python3
"""I66 U6c schema mutants (scratch lane only). Each mutant changes one constraint
of the candidate schemas in WT/scratch/i66_u6c_schemas/mut, runs the U6c schema
tests, and restores the file. A mutant is killed only by a failing test."""
import json, re, shutil, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u6c_schemas"
CAND = f"{WT}/f2a-carriers/projects/chirality-piping"
MUT = f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-1"
PSRC = "openpipestress.result_semantics/0.3.0/physics-source-1"
PREC = "openpipestress.result_semantics/0.3.0/precision-1"
PREVIEW_SHA = "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a"
RES, AR, SN = "results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json", "stress_neutral_export.v0.3.schema.json"


def res_branch(d, sid):
    return next(b for b in d["$defs"]["ResultEnvelope"]["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == sid)


def ar_branch(d, sid):
    return next(b for b in d["$defs"]["AnalysisRun"]["oneOf"] if b["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["id"]["const"] == sid)


def sn_branch(d, sid):
    return next(b for b in d["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == sid)


def drop(lst, item):
    lst.remove(item)


M = [
    ("R01_codes_not_in_enum", RES, lambda d: [d["$defs"]["RowDisclosure"]["properties"]["reason_code"]["enum"].remove(c) for c in ("retained_precision_absolute_verified", "retained_precision_not_covered")]),
    ("R02_preview_admits_codes", RES, lambda d: res_branch(d, PREVIEW)["allOf"].pop()),
    ("R03_precision_admits_codes", RES, lambda d: res_branch(d, PREC)["allOf"].pop()),
    ("R04_forbid_only_absolute", RES, lambda d: res_branch(d, PREVIEW)["allOf"][1]["properties"]["row_disclosures"]["items"]["properties"]["reason_code"]["not"]["enum"].pop()),
    ("A01_no_receipt_property", AR, lambda d: d["$defs"]["AnalysisRun"]["properties"].pop("retained_precision")),
    ("A02_receipt_not_required", AR, lambda d: ar_branch(d, SUCC).pop("required")),
    ("A03_successor_sha_preview", AR, lambda d: ar_branch(d, SUCC)["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["sha256"].__setitem__("const", PREVIEW_SHA)),
    ("A04_contract_oneof_missing", AR, lambda d: d["$defs"]["SemanticContract"]["oneOf"].pop()),
    ("A05_contract_id_enum_missing", AR, lambda d: d["$defs"]["SemanticContract"]["properties"]["id"]["enum"].remove(SUCC)),
    ("A06_preview_admits_receipt", AR, lambda d: drop(ar_branch(d, PREVIEW)["not"]["anyOf"], {"required": ["retained_precision"]})),
    ("A07_physics_source_admits_receipt", AR, lambda d: ar_branch(d, PSRC).pop("not")),
    ("A08_successor_admits_sbr", AR, lambda d: drop(ar_branch(d, SUCC)["not"]["anyOf"], {"required": ["source_block_recovery"]})),
    ("A09_successor_admits_evidence", AR, lambda d: drop(ar_branch(d, SUCC)["not"]["anyOf"], {"required": ["contract_evidence"]})),
    ("A10_receipt_ref_open", AR, lambda d: d["$defs"]["AnalysisRun"]["properties"].__setitem__("retained_precision", {"type": "object"})),
    ("A11_row_identity_open", AR, lambda d: ar_branch(d, SUCC)["properties"].pop("result_refs")),
    ("S01_producer_enum_missing", SN, lambda d: d["properties"]["producer"]["properties"]["semantic_contract_id"]["enum"].remove(SUCC)),
    ("S02_profile_enum_missing", SN, lambda d: d["properties"]["formulation_basis"]["properties"]["profile_id"]["enum"].remove("product_preview_retained_w1a_v2")),
    ("S03_receipt_not_required", SN, lambda d: sn_branch(d, SUCC)["required"].remove("retained_precision")),
    ("S04_successor_profile_preview", SN, lambda d: sn_branch(d, SUCC)["properties"]["formulation_basis"]["properties"]["profile_id"].__setitem__("const", "product_preview_mechanics_v1")),
    ("S05_preview_admits_receipt", SN, lambda d: drop(sn_branch(d, PREVIEW)["not"]["anyOf"], {"required": ["retained_precision"]})),
    ("S06_successor_admits_sbr", SN, lambda d: sn_branch(d, SUCC).pop("not")),
    ("S07_receipt_ref_open", SN, lambda d: d["properties"].__setitem__("retained_precision", {"type": "object"})),
    ("S08_successor_sha_preview", SN, lambda d: sn_branch(d, SUCC)["properties"]["semantic_contract"]["properties"]["sha256"].__setitem__("const", PREVIEW_SHA)),
    ("S09_evidence_not_required", SN, lambda d: sn_branch(d, SUCC)["required"].remove("contract_evidence")),
    ("S10_ref_enum_missing", SN, lambda d: d["properties"]["semantic_contract_ref"]["properties"]["ref_id"]["enum"].remove(SUCC)),
    ("S11_physics_source_admits_receipt", SN, lambda d: sn_branch(d, PSRC).pop("not", None) if "not" in sn_branch(d, PSRC) else None),
]


def main(only):
    results = []
    for mid, name, mutate in M:
        if only and mid not in only:
            continue
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            print("MEMGUARD NOT RUNNING"); sys.exit(9)
        raw = open(f"{CAND}/schemas/{name}", "rb").read()
        data = json.loads(raw)
        before = json.dumps(data, sort_keys=True)
        mutate(data)
        changed = json.dumps(data, sort_keys=True) != before
        open(f"{MUT}/schemas/{name}", "w").write(json.dumps(data, indent=2) + "\n")
        t = time.time()
        r = subprocess.run(f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                           f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1200 "
                           f"{VENV}/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_schema.py", shell=True, capture_output=True, text=True)
        failed = sorted(set(re.findall(r"^FAILED (\S+)", r.stdout, re.M)))
        status = "NO_CHANGE" if not changed else ("KILLED" if r.returncode != 0 and failed else ("SURVIVED" if r.returncode == 0 else "ERROR"))
        results.append({"id": mid, "file": name, "status": status, "failed": [f.split("::")[-1] for f in failed][:5], "seconds": round(time.time() - t)})
        print(mid, status, [f.split("::")[-1] for f in failed][:3], flush=True)
        open(f"{MUT}/schemas/{name}", "wb").write(raw)
    json.dump(results, open(f"{S}/logs/mutants_final.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1:])
