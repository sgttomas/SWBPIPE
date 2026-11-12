"""The reference system's invented hanger catalogue (DEL-04-11 REQ-006; release target exit criterion 1).

The catalogue stands in for a user's private hanger library. Every value in it is invented; the checks
below hold it to the library import boundary and to what sizing reads from a record."""
import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from core.library_import.provenance_checker import validate_library_import  # noqa: E402

CATALOGUE = ROOT / "fixtures/reference_system/invented_hanger_catalogue.json"
SCHEMA = json.loads((ROOT / "schemas/hanger.schema.yaml").read_text())


def _catalogue():
    return json.loads(CATALOGUE.read_text())


def test_catalogue_passes_the_schema_and_the_private_import_boundary():
    document = _catalogue()
    assert document["schema_version"] == "1.1.0"
    assert not list(Draft202012Validator(SCHEMA).iter_errors(document))
    result = validate_library_import(document, library_kind="hanger", intended_visibility="private")
    assert result.accepted and result.outcome == "PRIVATE_LOCAL_ONLY" and not result.findings


def test_every_record_is_sizable_and_says_it_is_invented():
    document = _catalogue()
    records = document["hanger_records"]
    ids = [record["hanger_id"] for record in records]
    assert len(ids) == len(set(ids))
    families = {record["hanger"]["hanger_type"] for record in records}
    assert families == {"variable_spring_hanger", "constant_effort_support"}
    for record in records:
        hanger = record["hanger"]
        low, high = hanger["working_load_range"]["minimum"], hanger["working_load_range"]["maximum"]
        assert low["unit"] == high["unit"] == "N" and 0 < low["magnitude"] < high["magnitude"]
        assert hanger["travel_range"]["magnitude"] > 0
        if hanger["hanger_type"] == "variable_spring_hanger":
            assert hanger["stiffness"]["dof"] == "UZ" and hanger["stiffness"]["value"]["magnitude"] > 0
        else:
            # Sizing reads neither member of a constant record (REQ-006); the catalogue carries none.
            assert "constant_load" not in hanger and "stiffness" not in hanger
        assert "movement_limit" not in hanger
        assert record["provenance"]["source_name"].startswith("Invented")
        assert hanger["source_reference"].startswith("invented-local:")


def test_working_load_ranges_leave_no_gap_within_a_family():
    # A hot-load basis anywhere between the smallest minimum and the largest maximum lies in some record's
    # range, so the reference system's sizing request does not fail for want of a size.
    for family in ("variable_spring_hanger", "constant_effort_support"):
        ranges = sorted(
            (record["hanger"]["working_load_range"]["minimum"]["magnitude"],
             record["hanger"]["working_load_range"]["maximum"]["magnitude"])
            for record in _catalogue()["hanger_records"] if record["hanger"]["hanger_type"] == family)
        reach = ranges[0][1]
        for low, high in ranges[1:]:
            assert low < reach, (family, low, reach)
            reach = max(reach, high)
