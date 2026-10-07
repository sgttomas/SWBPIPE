"""Run HEAD's new contract-test inputs through a given reader tree (BASE), outside pytest."""
import sys, importlib.util
from pathlib import Path
reader_root, test_root = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(reader_root))
from core.analysis_runs import retained_precision as rp
spec = importlib.util.spec_from_file_location("ct", test_root / "tests/test_retained_precision_contract.py")
ct = importlib.util.module_from_spec(spec); ct.rp = rp; spec.loader.exec_module(ct); ct.rp = rp
print("rp from", Path(rp.__file__).parent.parent.parent.name)
print("(4b) beside a selected case:", ct._b1_verdict(ct.F_BASE, ct._d38_edits()))
for tag in ("evaluation", "formation"):
    print("W2-published not_required", tag, ct._b1_verdict(ct.P_BASE, ct._w2_published(1, tag), must=ct.NOT_REQUIRED))
drop = lambda i: {"path": ["numerical_quality", "cases", i, "solve_quality"], "op": "remove"}
print("N2 selected report:", ct._b1_verdict(ct.O_BASE, [drop(0)]))
