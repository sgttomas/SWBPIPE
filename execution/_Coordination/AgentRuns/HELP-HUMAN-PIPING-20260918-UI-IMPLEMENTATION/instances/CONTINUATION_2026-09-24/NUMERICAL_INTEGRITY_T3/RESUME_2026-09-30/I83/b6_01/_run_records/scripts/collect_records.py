"""Copy I83's run records into the records folder with placeholder paths only. Usage: collect_records.py <dest>
(In the recorded copy of this script, SUBS's host paths appear already replaced by their placeholders.)"""
import os, re, shutil, sys
from pathlib import Path
S = Path(os.environ["S"]); DEST = Path(sys.argv[1]); RR = DEST / "_run_records"
SUBS = [
    ("VENV", "VENV"),
    ("NMS", "NMS"),
    ("R", "R"),
    ("S", "S"),
    ("WT", "WT"),
    ("PYTHON_HOME", "PYTHON_HOME"),
]
FILES = {
    "scripts": ["scripts/env.sh", "scripts/suites.sh", "scripts/build_07m.py", "scripts/build_case_file.py", "scripts/stage_patch.py",
                "scripts/g7_probe.py", "scripts/transport_py.py", "scripts/zzI83Probe.test.ts", "scripts/zzI83Transport.test.ts",
                "dev/projects/chirality-piping/core/reporting/result_export/tests/zz_i83_probe.rs", "scripts/rv92_probes.py",
                "scripts/mutants.py", "logs/rv92_probes.patch.txt", "scripts/item4_base.sh", "scripts/compare_suites.py", "scripts/collect_records.py"],
    "probes": ["g7_header_probe.json", "transport_parity_base_head.json"],
    "mutants": ["logs/mutants.jsonl", "logs/mutants/base_harness.txt", "logs/mutants/base_harness_C1.log", "logs/mutants/base_harness_C2.log"],
    "diff": ["logs/b6.diff", "logs/changed_files.tsv", "logs/commits.txt"],
    "suites": ["logs/suites_compare.json", "logs/suites_summary.txt", "logs/base_window.txt", "logs/head_window.txt", "logs/stageA_summary.txt",
               "logs/wasm_assets_worktree.sha256", "logs/base_tsc.log", "logs/head_tsc.log"],
}
for sub, files in FILES.items():
    (RR / sub).mkdir(parents=True, exist_ok=True)
    for rel in files:
        src = S / rel
        text = src.read_text(encoding="utf-8")
        for a, b in SUBS:
            text = text.replace(a, b)
        assert "/Us" "ers/" not in text and "/pri" "vate/" not in text, rel
        (RR / sub / src.name).write_text(text, encoding="utf-8")
print("ok")
