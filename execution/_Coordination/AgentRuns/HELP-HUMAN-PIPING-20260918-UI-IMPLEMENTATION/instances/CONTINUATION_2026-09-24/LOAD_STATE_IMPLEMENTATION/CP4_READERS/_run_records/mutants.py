"""CP4_READERS mutants: each reverts one new check (or, for RS-N2-PIN, removes
Rust's existing any-magnitude acceptance) in a scratch copy of WORKING_ROOT.
Usage: python mutants.py <scratch WORKING_ROOT> <mutant id> apply|list"""
import sys
from pathlib import Path

PY = "core/analysis_runs/load_reference_evidence.py"
RS = "core/reporting/result_export/src/load_reference.rs"
MUTANTS = {
    "PY-N2-U64": (PY, "    if type(value) is int:\n        try:", "    if type(value) is int:\n        if not -U64_MAX <= value <= U64_MAX:\n            return None\n        try:"),
    "PY-N2-OVERFLOW": (PY, "        try:\n            return float(value)\n        except OverflowError:\n            return None", "        return float(value)"),
    "PY-N2-NOCONVERT": (PY, "    return number\n", "    return value if type(value) is int else number\n"),
    "PY-N3-ADJ": (PY, "upper == lower + 1 and (start", "lower <= upper and (start"),
    "PY-N3-STRICT": (PY, "else start < end)", "else start <= end)"),
    "PY-N3-DUP": (PY, '            _require(entry not in entries, "LAW_SEGMENT_DUPLICATE")\n', ""),
    "RS-N3-ADJ": (RS, "lower.checked_add(1) == Some(upper)", "lower <= upper"),
    "RS-N3-STRICT": (RS, "} else { start < end }", "} else { start <= end }"),
    "RS-N3-DUP": (RS, '            require(!entries.contains(&entry), "LAW_SEGMENT_DUPLICATE")?;\n', ""),
    "RS-N2-PIN": (RS, "        .filter(|n| n.is_finite())\n        .ok_or_else(|| code(\"NUMBER_INVALID\"))", "        .filter(|n| n.is_finite() && n.abs() < 18446744073709551616.0)\n        .ok_or_else(|| code(\"NUMBER_INVALID\"))"),
}
root, mutant, action = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
if action == "list":
    print(" ".join(MUTANTS)); sys.exit()
path, old, new = MUTANTS[mutant]
target = root / path
text = target.read_text()
assert text.count(old) == 1, (mutant, text.count(old))
target.write_text(text.replace(old, new))
print(f"{mutant}: applied to {path}")
