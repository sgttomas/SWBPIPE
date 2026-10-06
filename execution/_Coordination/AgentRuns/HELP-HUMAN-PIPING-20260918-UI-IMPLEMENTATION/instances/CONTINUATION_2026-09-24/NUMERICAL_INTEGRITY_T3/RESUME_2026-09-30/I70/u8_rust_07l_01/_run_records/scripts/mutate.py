"""I70 records-only mutants, applied to the scratch probe copy only (never the candidate).

usage: mutate.py <probe P root> <name>   name in: none RM1 RM2 RM3 RM4 TMA
Each replacement must match exactly once; the caller restores the pristine files afterwards.
"""
import json
import sys
from pathlib import Path

P = Path(sys.argv[1])
name = sys.argv[2]
SRC = P / "core/reporting/result_export/src/retained_precision.rs"
CORPUS = P / "fixtures/results/retained_precision_cases.json"

RUST = {
    # RM1: the A-exclusion removed (a permitted A may be nonzero where no non-input row exists).
    "RM1": ("            if (0..4).any(|k| a[k] && !non_input[k]) {\n                return false;\n            }\n", ""),
    # RM2: feasibility coupled at L = 0 (translation implies rotation, force implies moment).
    "RM2": ("            let mut positive = if l == 0.0 {\n                a\n", "            let mut positive = if false {\n                a\n"),
    # RM3: the estimate hats coupled at L = 0.
    "RM3": ("        let hats = if l == 0.0 { e } else { [e[0] || e[1]; 2] };", "        let hats = if false { e } else { [e[0] || e[1]; 2] };"),
    # RM4: the no-free-DOF rule removed.
    "RM4": ("            fail(!has_data[bi])?;\n", "            let _ = &has_data;\n"),
}

if name == "none":
    pass
elif name in RUST:
    old, new = RUST[name]
    text = SRC.read_text()
    assert text.count(old) == 1, (name, text.count(old))
    SRC.write_text(text.replace(old, new))
elif name == "TMA":
    # TM-A: one new mutation is turned into a G1 refusal that its own corpus expectation
    # agrees with, so only a check of the slice's literal first gate and code can see it.
    raw = CORPUS.read_text()
    corpus = json.loads(raw)
    assert raw == json.dumps(corpus, indent=2) + "\n"
    m = corpus["mutations"][279]
    assert m["id"] == "isolated_has_data_sparse_interactive", m["id"]
    m["after_rehash"] = [{"path": ["retained_precision", "receipt_sha256"], "op": "set", "value": "0" * 64}]
    m["expected"] = {"gate": "G1", "code": "RETAINED_PRECISION_RECEIPT_MISMATCH"}
    CORPUS.write_text(json.dumps(corpus, indent=2) + "\n")
else:
    raise SystemExit(f"unknown mutant {name}")
print(f"applied {name}")
