"""RV108 probe batch 2: the blocked envelope (scope sentence), two-defect orders, and edge keys."""
import json, sys
from copy import deepcopy
from pathlib import Path
PROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2]); sys.path.insert(0, str(PROOT)); sys.path.insert(0, sys.argv[3])
import gen_probes_lib as L
corpus = json.loads((PROOT / "fixtures/results/retained_precision_cases.json").read_text()); cases = {c["id"]: c for c in corpus["cases"]}
bases = {"ordinary_prepared_synthetic": (cases["ordinary_prepared_synthetic"]["source"], cases["ordinary_prepared_synthetic"]["invocation"])}
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((PROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text()); bases[f"milestone_{mode}"] = (doc["source"], doc["invocation"])
probes = []
def add(pid, fam, base, fn):
    s, inv = deepcopy(bases[base][0]), deepcopy(bases[base][1]); fn(s); L.rehash(s)
    probes.append({"id": f"{base}|{pid}", "family": fam, "base": base, "source": s, "invocation": inv})
for base in bases:
    for st in ("MODEL_INCOMPLETE", "MECHANICS_FAILED", "NOT_RUN"):
        add(f"mechanics={st}", "C_blocked", base, lambda s, st=st: L.setp(s, ["status", "mechanics"], st))
    add("two:contract_evidence_null+source_block_recovery", "C_two_defect", base, lambda s: (L.setp(s, ["contract_evidence"], None), L.setp(s, ["source_block_recovery"], None)))
    add("two:case_enum+formulation_empty", "C_two_defect", base, lambda s: (L.setp(s, ["numerical_quality", "cases", 0, "structural_status"], "x"), L.setp(s, ["formulation_basis", "limitations"], [])))
    add("two:carrier_evidence+case_enum", "C_two_defect", base, lambda s: (L.setp(s, ["carrier_evidence"], {}), L.setp(s, ["numerical_quality", "cases", 0, "structural_status"], "x")))
    add("case0.__proto__key", "C_edge", base, lambda s: s["numerical_quality"]["cases"][0].__setitem__("__proto__", 1))
    add("case0.evidence_refs=[unknown_id]", "C_edge", base, lambda s: L.setp(s, ["numerical_quality", "cases", 0, "evidence_refs"], ["rv108:no-such-row"]))
    add("case0.accuracy_evidence=unresolved(valid)", "C_edge", base, lambda s: L.setp(s, ["numerical_quality", "cases", 0, "accuracy_evidence"], "unresolved"))
    add("case0.structural_status=trailing_space", "C_edge", base, lambda s: L.setp(s, ["numerical_quality", "cases", 0, "structural_status"], "passive_model_basis "))
OUT.write_text(json.dumps(probes)); print(len(probes))
