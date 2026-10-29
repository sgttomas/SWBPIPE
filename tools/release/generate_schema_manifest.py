#!/usr/bin/env python3
"""Generate the schema manifest the DEC-058 release-candidate scan trusts.

The scan treats a JSON member as a schema (and skips the `properties` maps
that declare provenance fields) only when its path is listed in
`tools/release/schema_manifest.json` and its bytes hash to the listed
SHA-256. This generator derives that list from the project's real schema
set, so it is never hand-typed: every `schemas/*.schema.json` and
`validation/witness/schemas/*.schema.json` file, each checked against the
JSON Schema 2020-12 metaschema with the pinned validator (a file's own
`$schema` never chooses it). Changing a schema therefore means regenerating
the manifest; `tests/test_release_candidate_scan.py` runs `--check`.

Usage:
  python3 tools/release/generate_schema_manifest.py           # write
  python3 tools/release/generate_schema_manifest.py --check   # compare
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = "tools/release/schema_manifest.json"
SCHEMA_SOURCES = ("schemas/*.schema.json", "validation/witness/schemas/*.schema.json")
METASCHEMA = "https://json-schema.org/draft/2020-12/schema"


def schema_paths(root: Path) -> list[str]:
    return sorted(
        path.relative_to(root).as_posix()
        for pattern in SCHEMA_SOURCES
        for path in root.glob(pattern)
        if path.is_file() and not path.is_symlink()
    )


def check_against_metaschema(document) -> None:
    """Raise when `document` is not a valid JSON Schema 2020-12 schema."""
    from jsonschema import Draft202012Validator  # requirements-dev.txt

    Draft202012Validator.check_schema(document)


def build_manifest(root: Path) -> dict:
    entries = []
    for relative in schema_paths(root):
        data = (root / relative).read_bytes()
        check_against_metaschema(json.loads(data.decode("utf-8")))
        entries.append({"path": relative, "sha256": hashlib.sha256(data).hexdigest()})
    return {
        "generated_by": "tools/release/generate_schema_manifest.py",
        "metaschema": METASCHEMA,
        "schemas": entries,
    }


def render(manifest: dict) -> str:
    return json.dumps(manifest, indent=2) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="Compare with the committed manifest.")
    parser.add_argument("--root", type=Path, default=ROOT, help="Project root.")
    args = parser.parse_args(argv)
    root = args.root.resolve()
    expected = render(build_manifest(root))
    target = root / MANIFEST_PATH
    if args.check:
        actual = target.read_text(encoding="utf-8") if target.is_file() else ""
        if actual != expected:
            print(f"{MANIFEST_PATH} is out of date; run generate_schema_manifest.py", file=sys.stderr)
            return 1
        print(f"{MANIFEST_PATH} matches {len(json.loads(expected)['schemas'])} schemas.")
        return 0
    target.write_text(expected, encoding="utf-8")
    print(f"wrote {MANIFEST_PATH}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
