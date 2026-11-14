#!/usr/bin/env python3
"""Focused tests for the DEL-17-05 Print > To File CSV parser (invented fixture only)."""

from __future__ import annotations

import json
from pathlib import Path
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import (  # noqa: E402
    PARSER_VERSION,
    PRINT_CSV_COVERAGE,
    parse_caepipe_print_csv,
    read_caepipe_print_csv,
)
from core.handoff.caepipe_external.print_csv import (  # noqa: E402
    PRINTED_DECIMALS,
    RESOLUTION_CAUSES,
    resolve_section,
    token_facts,
)


FIXTURE = ROOT / "fixtures" / "caepipe_external" / "invented" / "print_to_file_results.csv"
PROVENANCE = ROOT / "fixtures" / "caepipe_external" / "invented" / "print_to_file_results.provenance.json"
SUSTAINED = "Sustained (W+P)"
EXPANSION = "Expansion (T1)"


def errors(result):
    return [item for item in result["diagnostics"] if item["severity"] == "error"]


def displacement_block(case: str, unit_row: str = "Node,(mm),(mm),(mm),(deg),(deg),(deg),,") -> list[str]:
    return [
        f"{case}: Displacements,",
        ",,,Displacements (global),,,,",
        ",X,Y,Z,XX,YY,ZZ,",
        unit_row,
        "10,1.5,,,,,,,",
    ]


def text_of(*lines: str) -> str:
    return "\r\n".join(["Invented Prover,INVENTED,", "Version 0.0,Invented,Invented time,", "", *lines, ""])


def test_fixture_parses_into_load_case_tables():
    result = read_caepipe_print_csv(FIXTURE)
    assert result["document_kind"] == "openpipestress.caepipe_print_csv.parsed"
    assert result["parser_version"] == PARSER_VERSION
    assert result["header"] == {
        "product_label": "Invented Prover",
        "version": "0.0",
        "model_name": "INVENTED-PILOT",
        "title": "Invented parser fixture",
        "analyzed": "Invented timestamp",
    }
    assert result["options"] == ["Piping code = Invented", "Reference temperature = 20 (C)", "Vertical direction = Y"]
    assert result["units"] == {"length": "mm", "rotation": "deg", "force": "N", "moment": "N*m"}
    assert errors(result) == []
    assert set(result["load_cases"]) == {SUSTAINED, EXPANSION}
    sustained = result["load_cases"][SUSTAINED]
    assert sustained["anchor_loads_global"]["30"] == {"fx": 0.75, "fy": 2.5, "fz": 0.0, "mx": 0.0, "my": 0.0, "mz": 40.5}
    assert sustained["anchor_loads_local"]["10"]["fy"] == -1.25
    assert sustained["restraint_loads_global"] == {"40": {"fx": 0.5, "fy": 3.75, "fz": 0.0}}
    assert sustained["displacements"]["40"] == {"ux": 0.25, "uy": -2.75, "uz": 0.5, "rx": 0.0, "ry": 0.003, "rz": -0.04}
    expansion = result["load_cases"][EXPANSION]
    assert expansion["anchor_loads_global"]["10"]["fx"] == -100.5
    assert expansion["displacements"]["20"]["ux"] == 1.75
    kinds = sorted((item["load_case"], item["kind"]) for item in result["sections_parsed"])
    assert len(kinds) == 12


def test_blank_numeric_cells_are_zero():
    result = read_caepipe_print_csv(FIXTURE)
    anchor = result["load_cases"][SUSTAINED]["anchor_loads_global"]["10"]
    assert anchor == {"fx": 0.0, "fy": 1.25, "fz": 0.0, "mx": 0.0, "my": 0.0, "mz": -40.5}
    assert result["load_cases"][SUSTAINED]["displacements"]["10"] == {key: 0.0 for key in ("ux", "uy", "uz", "rx", "ry", "rz")}
    assert any("blank numeric field" in record["notes"] for record in PRINT_CSV_COVERAGE)


def test_element_rows_pair_in_layout_order():
    result = read_caepipe_print_csv(FIXTURE)
    assert result["pairing_basis"] == "row_order_from_to_pairs"
    local = result["load_cases"][SUSTAINED]["element_forces_local"]
    assert [(item["element_index"], item["node_from"], item["node_to"]) for item in local] == [
        (0, "10", "20"), (1, "20", "30"), (2, "20", "40")
    ]
    assert local[0]["from"] == {"axial": 0.0, "shear_y": -1.5, "shear_z": 0.0, "torque": 0.0,
                                "moment_in_plane": 12.25, "moment_out_of_plane": 0.0}
    assert local[2]["to"]["moment_in_plane"] == -1.0
    global_forces = result["load_cases"][EXPANSION]["element_forces_global"]
    assert global_forces[2]["from"] == {"fx": 0.0, "fy": -6.25, "fz": 0.0, "mx": 0.0, "my": 0.0, "mz": 2.5}
    assert global_forces[1]["to"]["mz"] == 0.0


def test_odd_element_row_count_is_an_error_and_not_parsed():
    text = text_of(
        "              Element forces in global coordinates: Sustained (W+P),",
        "    Node,   FX (N),   FY (N),   FZ (N),     MX(Nm),     MY(Nm),     MZ(Nm),",
        "10,,1.5,,,,-12.25,,",
        "20,,-1.5,,,,8.75,,",
        "20,,1,,,,-8.75,,",
    )
    result = parse_caepipe_print_csv(text)
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-ELEMENT-PAIRING"]
    assert result["load_cases"] == {}
    assert result["sections_skipped"][-1]["reason"] == "element_pairing_error"


def test_unit_row_mismatch_is_an_error_and_section_not_parsed():
    text = text_of(*displacement_block(SUSTAINED, "Node,(in),(in),(in),(deg),(deg),(deg),,"))
    result = parse_caepipe_print_csv(text)
    found = errors(result)
    assert [item["code"] for item in found] == ["CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH"]
    assert found[0]["section"] == f"{SUSTAINED}: Displacements"
    assert result["load_cases"] == {}
    assert result["sections_skipped"] == [{"title": f"{SUSTAINED}: Displacements", "line": 4, "reason": "header_mismatch"}]


def test_column_header_mismatch_is_an_error():
    text = text_of(
        "Loads on Anchors in Global Coordinates: Sustained (W+P),",
        "Node,Tag,FY (N),FX (N),FZ (N),MX (Nm),MY (Nm),MZ (Nm),,",
        "10,,,1.25,,,,-40.5,,",
    )
    result = parse_caepipe_print_csv(text)
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH"]
    assert result["load_cases"] == {}


def test_unknown_section_is_skipped_with_info():
    result = read_caepipe_print_csv(FIXTURE)
    skipped = {item["title"]: item["reason"] for item in result["sections_skipped"]}
    assert skipped["Sorted stresses"] == "unknown_section"
    assert skipped["Support load summary for anchor at node 10"] == "unknown_section"
    info = [item for item in result["diagnostics"] if item["code"] == "CAEPIPE-PRINT-SECTION-UNKNOWN"]
    assert info and all(item["severity"] == "info" for item in info)


def test_min_max_block_is_skipped():
    result = read_caepipe_print_csv(FIXTURE)
    skipped = [item for item in result["sections_skipped"] if item["reason"] == "min_max_redundant"]
    assert [item["title"] for item in skipped] == [f"Min/Max Displacements: {SUSTAINED}"]
    assert set(result["load_cases"][SUSTAINED]["displacements"]) == {"10", "20", "30", "40"}


def test_empty_weight_and_operating_header_variants_are_accepted():
    blocks = []
    for case, label, unit in (("Empty Weight (W)", "", ""), ("Operating (W+P1+T1)", "Sopr", "(MPa)")):
        blocks += [
            f"{case}: Pipe element forces (local),",
            ",,,,,In plane,Out of plane,In plane,Out of plane,Torsion,",
            f",Axial,y shear,z shear,Torque,Bend.Moment,Bend.Moment,SIF,SIF,SIF,{label},,",
            f"Node,(N),(N),(N),(Nm),(Nm),(Nm),,,,{unit},,",
            "10,,-1.5,,,12.25,,1,1,1,,,",
            "20,,1.5,,,-8.75,,1,1,1,,,",
        ]
    result = parse_caepipe_print_csv(text_of(*blocks))
    assert errors(result) == []
    assert set(result["load_cases"]) == {"Empty Weight (W)", "Operating (W+P1+T1)"}


def test_the_local_axes_layout_is_skipped_never_read_by_guess():
    text = text_of(
        "Sustained (W+P): Pipe element forces (local),",
        ",fx,fy,fz,mx,my,mz,SIF,,,",
        "Node,(N),(N),(N),(Nm),(Nm),(Nm),,,,",
        "10,,-1.5,,,12.25,,1,,,",
        "20,,1.5,,,-8.75,,1,,,",
        "",
        *displacement_block("Sustained (W+P)"),
    )
    result = parse_caepipe_print_csv(text)
    assert errors(result) == []
    assert {"title": "Sustained (W+P): Pipe element forces (local)", "line": 4, "reason": "local_axes_layout"} in \
        result["sections_skipped"]
    tables = result["load_cases"]["Sustained (W+P)"]
    assert not tables["element_forces_local"] and set(tables["displacements"]) == {"10"}
    # With the case's stress column, as the other layout prints it.
    stressed = parse_caepipe_print_csv(text.replace(",fx,fy,fz,mx,my,mz,SIF,,,", ",fx,fy,fz,mx,my,mz,SIF,SL,,")
                                       .replace("Node,(N),(N),(N),(Nm),(Nm),(Nm),,,,", "Node,(N),(N),(N),(Nm),(Nm),(Nm),,(MPa),,"))
    assert errors(stressed) == [] and "local_axes_layout" in {item["reason"] for item in stressed["sections_skipped"]}
    # Any other header in that section is still an error.
    other = parse_caepipe_print_csv(text.replace(",fx,fy,fz,mx,my,mz,SIF", ",fx,fy,fz,mx,mz,my,SIF"))
    assert [item["code"] for item in errors(other)] == ["CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH"]
    # The skip needs valid, consistent unit labels: a length in a force column, or two systems, is refused.
    units = "Node,(N),(N),(N),(Nm),(Nm),(Nm),,,,"
    for label, code in (("Node,(mm),(N),(N),(Nm),(Nm),(Nm),,,,", "CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH"),
                        ("Node,(lb),(N),(N),(Nm),(Nm),(Nm),,,,", "CAEPIPE-PRINT-UNITS-MIXED")):
        refused = parse_caepipe_print_csv(text.replace(units, label))
        assert [item["code"] for item in errors(refused)] == [code]
        assert "local_axes_layout" not in {item["reason"] for item in refused["sections_skipped"]}


def test_unparseable_number_is_an_error_for_that_row():
    lines = displacement_block(SUSTAINED)
    lines.append("20,*****,,,,,,,")
    result = parse_caepipe_print_csv(text_of(*lines))
    found = errors(result)
    assert [item["code"] for item in found] == ["CAEPIPE-PRINT-NUMBER-INVALID"]
    assert found[0]["line"] == 9
    assert set(result["load_cases"][SUSTAINED]["displacements"]) == {"10"}


def test_cp1252_title_is_decoded(tmp_path):
    path = tmp_path / "results.csv"
    text = "\r\n".join(["Invented Prover,INVENTED,", "Version 0.0,Tuyauterie démo – essai,Invented time,",
                        "", *displacement_block(SUSTAINED), ""])
    path.write_bytes(text.encode("cp1252"))
    result = read_caepipe_print_csv(path)
    assert result["header"]["title"] == "Tuyauterie démo – essai"
    assert result["load_cases"][SUSTAINED]["displacements"]["10"]["ux"] == 1.5
    assert errors(result) == []


def test_coverage_records_and_fixture_provenance():
    assert PRINT_CSV_COVERAGE
    for record in PRINT_CSV_COVERAGE:
        assert set(record) == {"section_pattern", "basis", "supported", "unit_row", "notes"}
        assert record["basis"] == "public CAEPIPE-EXPORT-DATA / Print To File page + invented fixture"
    provenance = json.loads(PROVENANCE.read_text(encoding="utf-8"))
    assert provenance["produced_by_caepipe"] is False
    assert provenance["origin"] == "invented_by_project"
    assert provenance["redistribution_status"] == "public_permissive"
    assert FIXTURE.read_bytes().count(b"\r\n") > 10


# English and radian prints (invented text). Hand-derived conversions from the unit catalog's exact definitions:
# 1 inch = 0.0254 m = 25.4 mm; 1 lbf = 4.4482216152605 N; 1 ft-lb = 4.4482216152605 N x 0.3048 m
# = 1.35581794833140040 N*m; 1 rad = 180/pi deg.
def anchor_block(case: str, force: str = "N", moment: str = "Nm", row: str = "10,,100,,-2.5,,10,,") -> list[str]:
    return [
        f"Loads on Anchors in Global Coordinates: {case},",
        f"Node,Tag,FX ({force}),FY ({force}),FZ ({force}),MX ({moment}),MY ({moment}),MZ ({moment}),,",
        row,
    ]


def close(value: float, expected: float) -> bool:
    return abs(value - expected) <= 4e-16 * max(1.0, abs(expected))


def test_english_rows_are_read_and_normalized_by_exact_definitions():
    text = text_of(*anchor_block(SUSTAINED, "lb", "ft-lb"),
                   *displacement_block(SUSTAINED, "Node,(inch),(inch),(inch),(deg),(deg),(deg),,")[:4],
                   "20,1.000,-0.250,,0.5000,,,,")
    result = parse_caepipe_print_csv(text)
    assert errors(result) == []
    anchor = result["load_cases"][SUSTAINED]["anchor_loads_global"]["10"]
    assert close(anchor["fx"], 444.82216152605)           # 100 lbf x 4.4482216152605 N/lbf
    assert close(anchor["fz"], -11.12055403815125)        # -2.5 lbf x 4.4482216152605 N/lbf
    assert close(anchor["my"], 13.558179483314004)        # 10 ft-lb x 1.3558179483314004 N*m/ft-lb
    node = result["load_cases"][SUSTAINED]["displacements"]["20"]
    assert node["ux"] == 25.4 and node["uy"] == -6.35      # 1 in and -0.25 in, x 25.4 mm/in
    assert node["rx"] == 0.5                                # degrees stay degrees
    assert result["units"] == {"length": "mm", "rotation": "deg", "force": "N", "moment": "N*m"}
    assert result["source_units"] == {"system": "English", "rotation": "deg"}
    units = {item["kind"]: item["source_units"] for item in result["sections_parsed"]}
    assert units["anchor_loads_global"] == {"fx": "lb", "fy": "lb", "fz": "lb", "mx": "ft-lb", "my": "ft-lb",
                                            "mz": "ft-lb"}
    assert units["displacements"] == {"ux": "inch", "uy": "inch", "uz": "inch", "rx": "deg", "ry": "deg", "rz": "deg"}


def test_radian_rows_are_read_in_degrees():
    text = text_of(*displacement_block(SUSTAINED, "Node,(mm),(mm),(mm),(rad),(rad),(rad),,")[:4],
                   "20,1.500,,,1.0000,-0.5000,,,")
    result = parse_caepipe_print_csv(text)
    assert errors(result) == []
    node = result["load_cases"][SUSTAINED]["displacements"]["20"]
    assert node["ux"] == 1.5                                    # millimetres unchanged
    assert close(node["rx"], 57.29577951308232)                 # 1 rad x 180/pi
    assert close(node["ry"], -28.64788975654116)                # -0.5 rad x 180/pi
    assert result["source_units"] == {"system": "SI", "rotation": "rad"}


def test_mixed_units_are_refused():
    # Within one section: inches beside millimetres.
    result = parse_caepipe_print_csv(text_of(*displacement_block(SUSTAINED, "Node,(inch),(mm),(mm),(deg),(deg),(deg),,")))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-UNITS-MIXED"]
    assert result["load_cases"] == {} and result["sections_skipped"][-1]["reason"] == "units_mixed"
    # Across sections: an English section after an SI one, and radians after degrees.
    for second in (anchor_block(EXPANSION, "lb", "ft-lb"),
                   displacement_block(EXPANSION, "Node,(mm),(mm),(mm),(rad),(rad),(rad),,")):
        result = parse_caepipe_print_csv(text_of(*displacement_block(SUSTAINED), *second))
        assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-UNITS-MIXED"]
        assert list(result["load_cases"]) == [SUSTAINED]          # the first section stands, the second is refused
    # Force and moment of different systems in one row of headers.
    result = parse_caepipe_print_csv(text_of(*anchor_block(SUSTAINED, "lb", "Nm")))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-UNITS-MIXED"]


def test_unknown_unit_labels_are_refused_by_name():
    for block, label in ((anchor_block(SUSTAINED, "kip", "ft-lb"), "kip"),
                         (anchor_block(SUSTAINED, "N", "kNm"), "kNm"),
                         (displacement_block(SUSTAINED, "Node,(mm),(mm),(mm),(grad),(grad),(grad),,"), "grad")):
        result = parse_caepipe_print_csv(text_of(*block))
        found = errors(result)
        assert [item["code"] for item in found] == ["CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH"]
        assert f"unit label {label!r}" in found[0]["message"] and result["load_cases"] == {}


def test_a_value_that_overflows_on_conversion_is_refused():
    # 1e307 in x 25.4 mm/in and 4e306 rad x 180/pi deg/rad both exceed the largest binary64 (about 1.8e308).
    for block, row in ((displacement_block(SUSTAINED, "Node,(inch),(inch),(inch),(deg),(deg),(deg),,")[:4],
                        "20,1e307,,,,,,,"),
                       (displacement_block(SUSTAINED, "Node,(mm),(mm),(mm),(rad),(rad),(rad),,")[:4],
                        "20,,,,4e306,,,,")):
        result = parse_caepipe_print_csv(text_of(*block, row))
        assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-VALUE-UNREPRESENTABLE"]
        assert result["load_cases"][SUSTAINED]["displacements"] == {}  # the row is not read
        json.dumps(result, allow_nan=False)                              # strictly serializable


# -- printed resolution: the section rule (invented text) -------------------------------------------------------------
CAUSES = ("no_witness", "shorter_than_format", "unsupported_by_this_screen", "unsupported_units_or_angle",
          "malformed_output", "resolution_absent")
MM = ("ux", "uy", "uz")
LONG = "1" * 5000  # 5,000 exponent digits: beyond int()'s default digit limit


def test_candidate_printed_resolutions_are_listed_for_the_si_labels_only():
    """Policy boundary: candidates (observed format behaviour, not proof) exist for the SI labels only."""
    assert PRINTED_DECIMALS == {("SI", "mm"): 3, ("SI", "deg"): 4, ("SI", "N"): 0, ("SI", "Nm"): 0}
    assert RESOLUTION_CAUSES == CAUSES


def displacement_section(*rows: str, case: str = SUSTAINED) -> list[str]:
    """One SI displacement section of `rows`."""
    return [*displacement_block(case)[:4], *rows]


def mm_entries(result: dict, case: str = SUSTAINED) -> dict:
    """{(node, key): printed entry} for every mm value of the case's displacement section."""
    table = result["printed_resolution"][case]["displacements"]
    return {(node, key): row[key] for node, row in table.items() for key in MM}


def assert_all(entries: dict, nodes: set, cause: str | None, resolution: float | None = None) -> None:
    assert {node for node, _ in entries} == nodes
    for where, entry in entries.items():
        if cause is None:
            assert entry["resolution"] == resolution, where
        else:
            assert entry["resolution"] is None and entry["cause"] == cause and entry["reason"], where


def test_each_value_carries_its_token_facts_and_the_sections_outcome():
    """Policy boundary: the parse records each value's token facts and the section rule's outcome over the kept rows.
    In the invented fixture the displacement mm columns are witnessed; its degrees show at most 3 places of 4; its
    forces print decimals the candidate does not, so that unit is unsupported by this screen."""
    result = read_caepipe_print_csv(FIXTURE)
    printed = result["printed_resolution"][SUSTAINED]
    entry = printed["displacements"]["20"]["ux"]
    assert entry["label"] == "mm" and entry["system"] == "SI" and entry["resolution"] == 0.001
    assert entry["tokens"] == [{"form": "fixed", "places": 3, "nonzero": True, "integer_digits": 0}]  # "0.125"
    assert printed["displacements"]["10"]["uz"]["tokens"] == [{"form": "blank"}]
    assert printed["displacements"]["20"]["rx"]["cause"] == "shorter_than_format"
    assert printed["anchor_loads_global"]["10"]["fx"]["cause"] == "unsupported_by_this_screen"  # "1.25" N beside it
    elements = printed["element_forces_global"]
    assert len(elements) == len(result["load_cases"][SUSTAINED]["element_forces_global"]) == 3
    assert set(elements[0]) == {"from", "to"} and set(printed) == set(result["load_cases"][SUSTAINED])


def test_a_section_of_zeros_and_blanks_only_establishes_nothing():  # (a)
    """Policy boundary (no witness)."""
    result = parse_caepipe_print_csv(text_of(f"{SUSTAINED}: Loads on restraints,", "Node,Tag,FX (N),FY (N),FZ (N),,",
                                             "40,,0,,-0,,", "50,,,0,,,"))
    table = result["printed_resolution"][SUSTAINED]["restraint_loads_global"]
    assert_all({(node, key): row[key] for node, row in table.items() for key in ("fx", "fy", "fz")}, {"40", "50"},
               "no_witness")


def test_a_witness_at_the_candidate_places_carries_zero_blank_and_shorter_tokens():  # (b)
    """Policy boundary (established)."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,1.250,0,,,,,,", "20,0.05,,-0,,,,,",
                                                                   "30,,4.8,0.001,,,,,")))
    assert_all(mm_entries(result), {"10", "20", "30"}, None, 0.001)


def test_a_section_whose_nonzero_tokens_are_all_shorter_establishes_nothing():  # (c)
    """Policy boundary (all shorter): the reason states the disagreement."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,0.05,1.2,,,,,,", "20,4.8,0,-0.5,,,,,")))
    entries = mm_entries(result)
    assert_all(entries, {"10", "20"}, "shorter_than_format")
    assert entries[("10", "ux")]["reason"] == "print shows at most 2 places, format expects 3"


def test_a_token_with_more_places_than_the_candidate_unmatches_the_section():  # (d)
    """Policy boundary (more places), beside a witness (1.250)."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,1.250,0,,,,,,", "20,0.1250,,0.5,,,,,")))
    entries = mm_entries(result)
    assert_all(entries, {"10", "20"}, "unsupported_by_this_screen")
    assert entries[("10", "ux")]["reason"] == "print shows 4 places, format expects 3"


def test_an_exponent_form_token_unmatches_the_section():  # (e)
    """Policy boundary (exponent form), beside a witness (1.250); finite values are kept."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,1.250,0,,,,,,", "20,1.25E+00,,0.5,,,,,")))
    entries = mm_entries(result)
    assert_all(entries, {"10", "20"}, "unsupported_by_this_screen")
    assert entries[("20", "ux")]["reason"] == "exponent form"
    assert result["load_cases"][SUSTAINED]["displacements"]["20"]["ux"] == 1.25


def test_a_long_negative_exponent_unmatches_the_section_and_keeps_its_finite_value():  # (f)
    """Defect: a long exponent raised an uncaught ValueError. It is exponent form: unsupported by this screen, the
    exponent never converted; its value underflows to a finite number, which is kept."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,1.250,0,,,,,,", f"20,-1.5e-{LONG},,0.5,,,,,")))
    assert errors(result) == []
    assert_all(mm_entries(result), {"10", "20"}, "unsupported_by_this_screen")
    assert result["load_cases"][SUSTAINED]["displacements"]["20"]["ux"] == 0.0


@pytest.mark.parametrize("token", [f"-1.5e{LONG}", f"+1.5e+{LONG}"])
def test_a_long_positive_exponent_is_refused_as_a_number_never_an_exception(token):
    """Defect: a long exponent raised an uncaught ValueError. The token is exponent form; its value overflows, so the
    existing finite-number check refuses the row and parsing goes on."""
    assert token_facts(token) == {"form": "exponent"}
    result = parse_caepipe_print_csv(text_of(*displacement_block(SUSTAINED), f"20,{token},,,,,,,"))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-NUMBER-INVALID"]
    assert set(result["load_cases"][SUSTAINED]["displacements"]) == {"10"}


def test_a_refused_row_never_witnesses_the_section():  # (g)
    """Policy boundary (evidence stays local): row 20 would witness (0.125) but is refused for its invalid field."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,0,,0.5,,,,,", "20,0.125,*****,,,,,,")))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-NUMBER-INVALID"]
    assert_all(mm_entries(result), {"10"}, "shorter_than_format")


def test_a_rejected_duplicate_row_or_section_never_witnesses():
    """Policy boundary (evidence stays local): a later row for the same node, and a second section for the same case,
    are not read, and their witnesses establish nothing."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,0,,0.5,,,,,", "10,1.250,,,,,,,")))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-DUPLICATE-NODE"]
    assert_all(mm_entries(result), {"10"}, "shorter_than_format")
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,0,,,,,,,"), "",
                                             *displacement_section("10,1.250,,,,,,,")))
    assert [item["code"] for item in errors(result)] == ["CAEPIPE-PRINT-SECTION-DUPLICATE"]
    assert_all(mm_entries(result), {"10"}, "no_witness")


def test_no_resolution_is_borrowed_across_case_or_quantity():
    """Policy boundary (evidence stays local): a sustained witness does not reach the expansion case, and an mm
    witness does not reach the degrees of the same section."""
    result = parse_caepipe_print_csv(text_of(*displacement_section("10,1.250,,,,,,,"), "",
                                             *displacement_section("10,0,,,,,,,", case=EXPANSION)))
    assert_all(mm_entries(result), {"10"}, None, 0.001)
    assert_all(mm_entries(result, EXPANSION), {"10"}, "no_witness")
    assert result["printed_resolution"][SUSTAINED]["displacements"]["10"]["rx"]["cause"] == "no_witness"


def test_summed_restraint_rows_keep_every_rows_token():
    """Policy boundary: a node's summed restraint value keeps the tokens of each row it sums, and each is evidence."""
    def text(second: str) -> str:
        return text_of(f"{SUSTAINED}: Loads on restraints,", "Node,Tag,FX (N),FY (N),FZ (N),,", "40,,1,2,,,", second)
    result = parse_caepipe_print_csv(text("40,,3,4,,,"))
    assert result["load_cases"][SUSTAINED]["restraint_loads_global"]["40"] == {"fx": 4.0, "fy": 6.0, "fz": 0.0}
    entry = result["printed_resolution"][SUSTAINED]["restraint_loads_global"]["40"]["fx"]
    assert entry["resolution"] == 1.0 and len(entry["tokens"]) == 2
    lost = parse_caepipe_print_csv(text("40,,3.5,4,,,"))["printed_resolution"][SUSTAINED]["restraint_loads_global"]
    assert all(lost["40"][key]["cause"] == "unsupported_by_this_screen" for key in ("fx", "fy", "fz"))


def test_english_and_radian_values_have_no_candidate():
    """Policy boundary (unsupported units or angle): English labels, degrees in an English print, and rad. The SI
    lengths of a print with radian rotations keep their candidate."""
    text = text_of(*anchor_block(SUSTAINED, "lb", "ft-lb"),
                   *displacement_block(SUSTAINED, "Node,(inch),(inch),(inch),(deg),(deg),(deg),,")[:4],
                   "20,1.000,-0.250,,0.5000,,,,")
    printed = parse_caepipe_print_csv(text)["printed_resolution"][SUSTAINED]
    for entry, label in ((printed["anchor_loads_global"]["10"]["fx"], "'lb'"),
                         (printed["anchor_loads_global"]["10"]["my"], "'ft-lb'"),
                         (printed["displacements"]["20"]["ux"], "'inch'"),
                         (printed["displacements"]["20"]["rx"], "'deg'")):
        assert entry["cause"] == "unsupported_units_or_angle" and label in entry["reason"]
        assert "English" in entry["reason"]
    radians = parse_caepipe_print_csv(text_of(*displacement_block(SUSTAINED, "Node,(mm),(mm),(mm),(rad),(rad),(rad),,")[:4],
                                              "20,1.500,,,1.0000,-0.5000,,,"))
    node = radians["printed_resolution"][SUSTAINED]["displacements"]["20"]
    assert node["ux"]["resolution"] == 0.001
    assert node["rx"]["cause"] == "unsupported_units_or_angle" and "'rad'" in node["rx"]["reason"]


def test_token_facts_read_the_printed_form_without_converting_it():
    """Policy boundary: token forms. Zero and blank are not witnesses; a number in none of the decimal forms is
    malformed, apart from exponent form."""
    assert token_facts("") == {"form": "blank"}
    assert token_facts("-0.000") == {"form": "fixed", "places": 3, "nonzero": False, "integer_digits": 0}
    assert token_facts("12.50") == {"form": "fixed", "places": 2, "nonzero": True, "integer_digits": 2}
    assert token_facts("-.5") == {"form": "fixed", "places": 1, "nonzero": True, "integer_digits": 0}
    assert token_facts("12.") == {"form": "fixed", "places": 0, "nonzero": True, "integer_digits": 2}
    for text in ("1E+00", "1.5e-3", "-.5E2", f"1e{LONG}"):
        assert token_facts(text) == {"form": "exponent"}, text
    for text in ("1_0", "١", "1.2.3"):
        assert token_facts(text) == {"form": "malformed"}, text


def test_the_integer_digit_count_witness_compares_integer_digits_only():
    """Policy boundary: the integer digit count witness counts, per shorter nonzero token of an established unit,
    whether a full-places token has at least as many integer digits; it changes no outcome."""
    def entry(*tokens: str) -> dict:
        return {"label": "mm", "system": "SI", "tokens": [token_facts(token) for token in tokens]}
    outcomes, counts = resolve_section([entry("0.050"), entry("4.8"), entry("12.5"), entry("0")])
    assert all(outcome == {"resolution": 0.001} for outcome in outcomes)
    assert counts == {"witnessed_by_integer_digit_count": 0, "not_witnessed_by_integer_digit_count": 2}
    outcomes, counts = resolve_section([entry("10.050"), entry("4.8"), entry("12.5"), entry("123.4")])
    assert all(outcome == {"resolution": 0.001} for outcome in outcomes)
    assert counts == {"witnessed_by_integer_digit_count": 2, "not_witnessed_by_integer_digit_count": 1}
    _, counts = resolve_section([entry("4.8"), entry("0")])  # not established: nothing is counted
    assert counts == {"witnessed_by_integer_digit_count": 0, "not_witnessed_by_integer_digit_count": 0}
