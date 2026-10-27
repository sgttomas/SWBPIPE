#!/usr/bin/env python3
"""Focused tests for the DEL-17-05 Print > To File CSV parser (invented fixture only)."""

from __future__ import annotations

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import (  # noqa: E402
    PARSER_VERSION,
    PRINT_CSV_COVERAGE,
    parse_caepipe_print_csv,
    read_caepipe_print_csv,
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
