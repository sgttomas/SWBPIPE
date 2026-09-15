#!/usr/bin/env python3
"""Exact-version dispatch checks for analysis-run schemas."""
from pathlib import Path
import json
from schema_validation import validate_schema_document

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"


def test_analysis_schema_dispatch_is_exact_and_local():
    dispatcher = json.loads((SCHEMAS / "analysis_run.schema.json").read_text())
    validate_schema_document(dispatcher)
    assert dispatcher["oneOf"][0]["allOf"][0]["properties"]["schema_version"]["const"] == "0.1.0"
    assert dispatcher["oneOf"][1]["allOf"][0]["properties"]["schema_version"]["const"] == "0.2.0"
    for name, version in [
        ("analysis_run.v0.1.schema.json", "0.1.0"),
        ("analysis_run.legacy-desktop.v0.1.schema.json", "0.1.0"),
        ("analysis_run.v0.2.schema.json", "0.2.0"),
    ]:
        schema = json.loads((SCHEMAS / name).read_text())
        validate_schema_document(schema, schema_label=name)
        constraint = schema["properties"]["schema_version"]
        if name == "analysis_run.v0.1.schema.json":
            assert constraint["pattern"] == "^[0-9]+\\.[0-9]+\\.[0-9]+$"
        else:
            assert constraint["const"] == version


def main():
    test_analysis_schema_dispatch_is_exact_and_local()


if __name__ == "__main__":
    main()
