#!/usr/bin/env python3
"""Sorted-compact JSON vectors for the W-12 model-state record hash.

DEL-14-01 Design `desktop-states-runs-comparison.md` section 2 (serializer pin).
Every expected text and digest is computed by
`core/project_persistence/service.py` `canonical_json`
(`json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=True)`). The
desktop store's Rust serializer (`apps/desktop/src-tauri/src/model_states.rs`)
must produce the same text for `vectors`, and must refuse every value in
`refused_numbers` (`MODEL-STATE-RECORD-NUMBER-UNSUPPORTED`): no field of a
`ModelStateRecord` has a number type, so floats and integers outside i64/u64
are refused rather than approximated. Each refused entry keeps Python's own
text, which an approximation would have had to match (Codex r1 found
`999999999999975.2` and `279836846559720.12` where Rust's shortest-digit
tie-breaking differs). `tests/test_model_state_record_parity.py` checks the
fixture against this writer.

Usage:
    python3 tools/fixtures/sorted_compact_json_vectors.py            # write the fixture
    python3 tools/fixtures/sorted_compact_json_vectors.py --check    # compare, write nothing
"""
import hashlib
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True
PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))
from core.project_persistence import service  # noqa: E402

FIXTURE = PROJECT / "fixtures/persistence/sorted_compact_json_vectors.json"

CASES = [
    ("non_ascii_text_and_keys", {"z": "Zürich Ω 配管 😀", "é": ["naïve", "ß"], "Z": "upper", "a": {"😀": 1, "é": 2, "e": 3}}),
    ("escapes_and_controls", {"quote": "say \"hi\"", "backslash": "a\\b", "slash": "a/b", "controls": "\u0001\u001f\u007f\b\f\n\r\t", "nbsp": " ", "line_sep": " "}),
    ("integers", {"ints": [0, -1, 42, 9007199254740993, -9223372036854775808, 9223372036854775807, 18446744073709551615]}),
    ("literals_and_empty", {"t": True, "f": False, "n": None, "empty_list": [], "empty_object": {}, "nested": [[], [{}], [None]]}),
    ("model_state_like", {"state_name": "Basis – rev B · «tie-in»", "tags": [{"tag": "révision", "tag_kind": "user_label"}], "notes": [{"statement": "Δp ≤ 5 kPa; see “note”"}]}),
]

REFUSED_NUMBERS = [
    0.1, 1.5, -0.0, 0.0, 1e-05, 1e16, 123.0,
    999999999999975.2, 279836846559720.12,
    1.7976931348623157e308, 5e-324,
    18446744073709551616, -9223372036854775809,
]


def document() -> dict:
    vectors = []
    for name, value in CASES:
        text = service.canonical_json(value)
        vectors.append({"name": name, "value": value, "sorted_compact_json": text,
                        "sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()})
    return {
        "description": (
            "Sorted-compact JSON vectors for the W-12 model-state record hash, written by "
            "tools/fixtures/sorted_compact_json_vectors.py from core/project_persistence/service.py "
            "canonical_json. The Rust serializer in apps/desktop/src-tauri/src/model_states.rs must "
            "produce each vector's text and refuse every refused_numbers value "
            "(MODEL-STATE-RECORD-NUMBER-UNSUPPORTED); each refused entry keeps Python's own text."
        ),
        "vectors": vectors,
        "refused_numbers": [{"value": value, "python_sorted_compact_json": service.canonical_json(value)}
                            for value in REFUSED_NUMBERS],
    }


def render() -> str:
    return json.dumps(document(), indent=2, ensure_ascii=True) + "\n"


def main(argv: list[str]) -> int:
    text = render()
    if "--check" in argv:
        if FIXTURE.read_text(encoding="utf-8") != text:
            print(f"{FIXTURE.relative_to(PROJECT)} is stale; run {Path(__file__).relative_to(PROJECT)}", file=sys.stderr)
            return 1
        return 0
    FIXTURE.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
