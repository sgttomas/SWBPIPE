"""The shared retained-precision reader corpus: one logical document stored as ordered snapshot files.

Each file stays under 50 MB (`test_fixture_file_size.py`): a new snapshot is a new file, appended to
`CORPUS_FILES` here, in RS's `tests/retained_precision_contract.rs` and in TS's
`src/test-support/retainedPrecisionCorpus.ts`, never grown into an existing file. `cases`, `mutations`
and `must_pass` concatenate in file order; any other member appears once or identically in each file.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS_FILES = (
    "fixtures/results/retained_precision_cases.json",  # 07m + 07n
    "fixtures/results/retained_precision_cases_07o.json",  # 07o (B2), compact JSON
)
LIST_MEMBERS = ("cases", "mutations", "must_pass")


def load_corpus(files=CORPUS_FILES):
    merged = {}
    for name in files:
        for key, value in json.loads((ROOT / name).read_text()).items():
            if key in LIST_MEMBERS:
                assert isinstance(value, list), f"{name}: {key} is not an array"
                merged.setdefault(key, []).extend(value)
            elif key in merged:
                assert merged[key] == value, f"{name}: corpus member {key} differs from an earlier file"
            else:
                merged[key] = value
    return merged
