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


def _series(record):
    """The series a record belongs to: its family, and for a variable record its travel range."""
    hanger = record["hanger"]
    assert hanger["travel_range"]["unit"] == "mm"
    return hanger["hanger_type"], hanger["travel_range"]["magnitude"]


def test_each_series_covers_one_to_forty_five_kilonewtons_without_a_gap():
    # Within each series (the two variable travel series and the constant records), a load anywhere from 1 kN to
    # 45 kN lies in some record's working load range. That is coverage of the load test only: whether a record
    # fits also depends on travel, the allowance, the DOF and the person's variation limit (REQ-004).
    series = {}
    for record in _catalogue()["hanger_records"]:
        hanger = record["hanger"]
        series.setdefault(_series(record), []).append(
            (hanger["working_load_range"]["minimum"]["magnitude"], hanger["working_load_range"]["maximum"]["magnitude"]))
    assert sorted(series) == [("constant_effort_support", 150), ("variable_spring_hanger", 50),
                              ("variable_spring_hanger", 100)]
    for name, ranges in series.items():
        ranges.sort()
        assert len(ranges) == 9, name
        assert ranges[0][0] == 1000 and max(high for _, high in ranges) == 45000, name
        reach = ranges[0][1]
        for low, high in ranges[1:]:
            # Working load ranges are closed, so a range that starts where the reach ends leaves no gap.
            assert low <= reach, (name, low, reach)
            reach = max(reach, high)


def test_a_variable_record_spans_its_working_load_range_over_its_travel_range():
    # The catalogue's own construction: rate = (maximum - minimum) / travel, so a spring set at one end of its
    # working load range reaches the other end over its travel range.
    for record in _catalogue()["hanger_records"]:
        hanger = record["hanger"]
        if hanger["hanger_type"] != "variable_spring_hanger":
            continue
        assert hanger["stiffness"]["value"]["unit"] == "N/m"
        low, high = hanger["working_load_range"]["minimum"]["magnitude"], hanger["working_load_range"]["maximum"]["magnitude"]
        travel_mm = hanger["travel_range"]["magnitude"]
        assert hanger["stiffness"]["value"]["magnitude"] * travel_mm == (high - low) * 1000, record["hanger_id"]
