#!/usr/bin/env python3
"""The analysis-run schema dispatcher admits one current version, 0.3.0."""
from pathlib import Path
import json

import pytest

from schema_validation import validate_instance, validate_schema_document

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"


def test_analysis_schema_dispatch_is_current_only_and_local():
    dispatcher = json.loads((SCHEMAS / "analysis_run.schema.json").read_text())
    validate_schema_document(dispatcher)
    assert "oneOf" not in dispatcher
    assert dispatcher["allOf"][0]["properties"]["schema_version"]["const"] == "0.3.0"
    assert dispatcher["allOf"][1] == {"$ref": "analysis_run.v0.3.schema.json"}
    current = json.loads((SCHEMAS / "analysis_run.v0.3.schema.json").read_text())
    validate_schema_document(current, schema_label="analysis_run.v0.3.schema.json")
    assert current["properties"]["schema_version"]["const"] == "0.3.0"
    # The retired versions have no schema: no reader or writer remains for them.
    assert sorted(path.name for path in SCHEMAS.glob("analysis_run*.schema.json")) == [
        "analysis_run.schema.json", "analysis_run.v0.3.schema.json"]


@pytest.mark.parametrize("version", ["0.1.0", "0.2.0"])
def test_retired_record_versions_fail_the_dispatcher(version):
    dispatcher = json.loads((SCHEMAS / "analysis_run.schema.json").read_text())
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(dispatcher, {"schema_version": version, "analysis_run": {}}, instance_label=f"analysis-run {version}")


def main():
    test_analysis_schema_dispatch_is_current_only_and_local()


if __name__ == "__main__":
    main()
