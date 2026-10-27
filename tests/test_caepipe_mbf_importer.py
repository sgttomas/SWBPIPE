#!/usr/bin/env python3
"""Grammar, refusal, loss-entry and exact-conversion tests of the DEL-17-04 MBF importer (Design
`caepipe-mbf-import.md` §4 to §16, §18.1 items 1 and 2).

Every MBF here is invented by this test file for the importer's checks; none is CAEPIPE output, a vendor example or
engineering data. Each named refusal asserts its code, its line and that no model is produced; each loss entry
asserts that the model is produced and the entry is listed.
"""

from __future__ import annotations

from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_mbf import importer as importer_module  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from core.handoff.caepipe_mbf.importer import (  # noqa: E402
    IMPORT_PROFILE_ID,
    REPORT_KIND,
    half_unit,
    import_caepipe_mbf,
    main,
    parse_english_length,
    split_fields,
)


INVENTED = {"*": {"origin": "project_invented", "source_name": "invented importer test material"}}
FIXTURE = ROOT / "fixtures" / "caepipe_mbf" / "invented" / "import" / "units_si.mbf"


def mbf(*layout: str, options: str = "HGRA,SI", optemp: str | None = "20", material=None, pipe=None,
        loads=("L1,20,0,0",), extra: tuple[str, ...] = (), version: str | None = "15.10",
        heading: tuple[str, ...] = ("INVENTED IMPORTER TEST",), end: str = "\r\n") -> bytes:
    """An invented SI MBF: one steel-like material, one section, one load and the given layout rows."""
    lines = list(heading)
    if version is not None:
        lines += ["VERSION", version]
    lines += ["OPTIONS", options]
    if optemp is not None:
        lines += ["OPTEMP", optemp]
    lines += ["MATERIAL", *(material or ("M1,0,0.3,1,1,CS,invented steel", "M1,20,200000,1.2e-05"))]
    lines += ["PIPE", *(pipe or ("P1,168.3,7.11,0,0,0,0,0,0",))]
    if loads is not None:
        lines += ["LOADS", *loads]
    lines += list(extra)
    lines += ["LAYOUT", *(layout or ("F10,KA,MM1,PP1,CL=L1", "T20,X3000"))]
    return "".join(line + end for line in lines).encode("utf-8")


def run(data: bytes, **options):
    options.setdefault("material_provenance", INVENTED)
    return import_caepipe_mbf(data, source_label="invented-test", **options)


def refused(result, code: str, line: int | None = None) -> dict:
    assert result.status == "refused", result.diagnostics
    assert result.model is None and result.id_map == {}
    matching = [item for item in result.diagnostics if item["code"] == code and item["severity"] == "error"]
    assert matching, result.diagnostics
    if line is not None:
        assert any(item["line"] == line for item in matching), matching
    assert any(item["code"] == code and item["category"] == "unsupported" for item in result.loss_report)
    return matching[0]


def listed(result, code: str, category: str) -> dict:
    assert result.status == "imported", result.diagnostics
    assert result.model is not None
    entries = [item for item in result.loss_report if item["code"] == code]
    assert entries and all(item["category"] == category for item in entries), result.loss_report
    return entries[0]


def layout_line(data: bytes, prefix: str) -> int:
    """The line number of the first line equal to `prefix`, else of the first line starting with it."""
    lines = data.decode("utf-8").replace("\r\n", "\n").split("\n")
    exact = [index for index, text in enumerate(lines, start=1) if text == prefix]
    return exact[0] if exact else next(index for index, text in enumerate(lines, start=1) if text.startswith(prefix))


def loads_of(result) -> list[tuple]:
    return [(case["id"], load["category"], load["target"].get("node") or load["target"].get("pipe")
             or load["target"].get("support"), load["direction"], load["magnitude"]["value"], load["magnitude"]["unit"])
            for case in result.model["load_cases"] for load in case["primitive_loads"]]


# --- §5.4 English lengths ---------------------------------------------------------------------------------
@pytest.mark.parametrize("text, inches", [
    ("7", Fraction(84)), ("-2", Fraction(-24)), ("4.25", Fraction(51)), (".5", Fraction(6)),
    ("7'3", Fraction(87)), ("7-3", Fraction(87)), ("7'3\"", Fraction(87)), ("7\u20193\u201d", Fraction(87)),
    ("7'3-1/4\"", Fraction(349, 4)), ("7-3-1/4", Fraction(349, 4)), ("0'0-5/16\"", Fraction(5, 16)),
    ("-2'0", Fraction(-24)), ("-1'6", Fraction(-18)), ("1'6-3/8", Fraction(147, 8)), ("10'11", Fraction(131)),
])
def test_every_documented_english_length_form(text, inches):
    assert parse_english_length(text) == inches


@pytest.mark.parametrize("text", ["7'12", "7'3.5", "7.5'3", "7'-1/4", "--2", "7'", "7-", "7'3-3/2", "7'3-1/0",
                                  "7\"", "1e2", "+-7", "7'3-1/4'", "7 3", "7'3/4", ""])
def test_undocumented_english_length_forms_are_refused(text):
    assert parse_english_length(text) is None


def test_an_undocumented_length_in_an_english_file_is_refused_by_name():
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X7'12", options="HGRA")
    refused(run(data), "MBF-IMPORT-LENGTH-NOTATION-INVALID", layout_line(data, "T20"))
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X7'3", options="HGRA,SI")  # feet-inch is not an SI length
    refused(run(data), "MBF-IMPORT-NUMBER-INVALID", layout_line(data, "T20"))
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B1'0,X10", "T30,Z10", options="HGRA")
    refused(run(data), "MBF-IMPORT-LENGTH-NOTATION-INVALID", layout_line(data, "T20"))


def test_rational_offset_sums_round_once_per_node():
    # Three offsets of 0.1 ft: 3.6 in exactly; a running binary64 sum gives 3.5999999999999996.
    assert 1.2 + 1.2 + 1.2 != 3.6
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X0.1", "T30,X0.1", "T40,X0.1", options="HGRA")
    result = run(data)
    assert [node["position"]["x"] for node in result.model["nodes"]] == [0.0, 1.2, 2.4, 3.6]
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X0.1", "T30,X0.1", "T40,X0.1")  # SI mm: 0.3, not 0.30000000000000004
    assert run(data).model["nodes"][-1]["position"]["x"] == 0.3


def test_half_unit_of_the_last_printed_digit():
    assert half_unit("3000") == Fraction(1, 2)
    assert half_unit("1.25") == Fraction(1, 200)
    assert half_unit("0") == 0
    assert half_unit("1.5e3") == 50
    assert half_unit("2.5E-3") == Fraction(1, 20000)


# --- §4 fields, numbers, comments and continuation -------------------------------------------------------------
def test_fields_keep_commas_inside_parentheses():
    assert split_fields("T20,X5,CFIXD(1,0,1),FOR( 1, 2 ,3)") == ["T20", "X5", "CFIXD(1,0,1)", "FOR( 1, 2 ,3)"]
    assert split_fields("A , B") == ["A", "B"]
    assert split_fields("FIXD(1,0") is None and split_fields("A)") is None


@pytest.mark.parametrize("value", ["1.2.3", "abc", "1e", "0x10", "1 5"])
def test_invalid_numbers_are_refused_with_line_and_field(value):
    data = mbf(pipe=(f"P1,{value},7.11,0,0,0,0,0,0",))
    diagnostic = refused(run(data), "MBF-IMPORT-NUMBER-INVALID", layout_line(data, "P1,"))
    assert "PIPE P1" in diagnostic["location"]


def test_an_empty_required_field_is_refused():
    data = mbf(pipe=("P1,,7.11",))
    refused(run(data), "MBF-IMPORT-FIELD-MISSING", layout_line(data, "P1,"))


def test_the_first_c_field_starts_the_comments_and_loses_one_c():
    # CWGT=5 is the comment WGT=5 (a joint weight), not CWGT; CCWGT=5 is the concentrated weight CWGT.
    diagnostic = refused(run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CWGT=5")), "MBF-IMPORT-ROW-KEY-UNSUPPORTED")
    assert "WGT" in diagnostic["message"] and "CWGT" not in diagnostic["message"]
    diagnostic = refused(run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CCWGT=5")), "MBF-IMPORT-LOAD-UNSUPPORTED")
    assert "CWGT" in diagnostic["message"]


def test_comment_names_match_exactly_so_the_longest_name_wins():
    for comment in ("FIXDX", "FX", "FIXDX=1", "FIXD(1,0,0)", "F(1,0,0)"):
        result = run(mbf("F10,KA,MM1,PP1,CL=L1", f"T20,X3000,C{comment}"))
        assert result.status == "imported", (comment, result.diagnostics)
        assert [(item["id"], item["restraints"]) for item in result.model["supports"]] == [
            ("support:10", ["UX", "UY", "UZ", "RX", "RY", "RZ"]), ("support:20-restraint", ["UX"])], comment
    refused(run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFIXDQ")), "MBF-IMPORT-COMMENT-UNKNOWN")


def test_a_trailing_comma_continues_the_comments_on_the_next_line():
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(0,-10,0),", "FLC=1,MOM(0,0,5)")
    result = run(data)
    assert ("case:mbf-sustained", "concentrated_moment", "node:20", "RZ", 5.0, "N*m") in loads_of(result)
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,")
    refused(run(data), "MBF-IMPORT-ROW-SYNTAX-INVALID", layout_line(data, "T20"))
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(0,-10,0),")
    refused(run(data), "MBF-IMPORT-ROW-SYNTAX-INVALID", layout_line(data, "T20"))


def test_a_documented_comment_with_an_empty_value_is_counted_and_carries_nothing():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CTAG=,LTAG="))
    assert result.status == "imported" and result.summary["empty_comments"] == 2


# --- §4.1 encoding and line ends --------------------------------------------------------------------------------
def test_a_non_utf8_byte_is_refused_naming_its_offset():
    data = mbf()
    broken = data.replace(b"invented steel", b"invented st\xe9el")
    diagnostic = refused(run(broken), "MBF-IMPORT-ENCODING-UNSUPPORTED")
    offset = broken.index(b"\xe9")
    assert diagnostic["location"] == f"byte {offset}"
    refused(run(b"\xef\xbb\xbf" + data), "MBF-IMPORT-ENCODING-UNSUPPORTED")


def test_a_lone_carriage_return_is_refused_and_lf_reads_like_crlf():
    data = mbf()
    refused(run(data.replace(b"\r\nPIPE", b"\rPIPE")), "MBF-IMPORT-LINE-END-UNSUPPORTED", layout_line(data, "M1,20"))
    crlf, lf = run(data), run(mbf(end="\n"))
    strip = lambda model: json.loads(json.dumps(model).replace(crlf.source["mbf_sha256"], "H").replace(  # noqa: E731
        lf.source["mbf_sha256"], "H").replace(crlf.source["mbf_sha256"][:12], "h").replace(lf.source["mbf_sha256"][:12], "h"))
    assert strip(crlf.model) == strip(lf.model)


def test_non_ascii_is_read_only_in_free_text_and_length_marks():
    accepted = run(mbf("* invented note \u00e9", "F10,KA,MM1,PP1,CL=L1", "T20,X3000",
                       heading=("INVENTED \u00c9", "second \u00e9"),
                       material=("M1,0,0.3,1,1,CS,acier invent\u00e9", "M1,20,200000,1.2e-05")))
    assert accepted.status == "imported", accepted.diagnostics
    assert accepted.model["materials"][0]["label"] == "acier invent\u00e9"
    assert "line 2: second \u00e9" in accepted.model["project"]["notes"]
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CTAG=s\u00e9")
    refused(run(data), "MBF-IMPORT-CHARACTER-UNSUPPORTED", layout_line(data, "T20"))
    data = mbf(pipe=("P1,168.3\u201d,7.11",))
    refused(run(data), "MBF-IMPORT-CHARACTER-UNSUPPORTED", layout_line(data, "P1"))
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(0,1\u2019,0),FLC=1")
    refused(run(data), "MBF-IMPORT-CHARACTER-UNSUPPORTED", layout_line(data, "T20"))


# --- §4.2 keywords, version and sections ---------------------------------------------------------------------------
def test_three_character_keywords_are_read_when_they_name_one_section():
    text = mbf().decode("utf-8")
    for full, short in (("MATERIAL", "MAT"), ("PIPE", "PIP"), ("LOADS", "LOA"), ("LAYOUT", "LAY")):
        text = text.replace(f"\r\n{full}\r\n", f"\r\n{short}\r\n")
    assert run(text.encode()).status == "imported"
    data = mbf().replace(b"OPTEMP", b"OPT")
    refused(run(data), "MBF-IMPORT-SECTION-AMBIGUOUS", layout_line(data, "OPT"))


def test_repeated_and_unknown_sections_are_refused():
    data = mbf(extra=("PIPE", "P2,100,5"))
    refused(run(data), "MBF-IMPORT-SECTION-REPEATED")
    data = mbf(pipe=("P1,168.3,7.11,0,0,0,0,0,0", "NEWSECTION"))
    refused(run(data), "MBF-IMPORT-SECTION-UNKNOWN", layout_line(data, "NEWSECTION"))


def test_version_is_required_and_listed():
    refused(run(mbf(version=None)), "MBF-IMPORT-VERSION-MISSING")
    data = mbf(version="15.00")
    refused(run(data), "MBF-IMPORT-VERSION-UNSUPPORTED", layout_line(data, "15.00"))


@pytest.mark.parametrize("section, code", [
    (("SOILS", "S1,0,150,0,30,0.8"), "MBF-IMPORT-SECTION-UNSUPPORTED"),
    (("PUMPS", "PUMP1,1,350"), "MBF-IMPORT-SECTION-UNSUPPORTED"),
    (("SEISMIC", "0.25,0,0"), "MBF-IMPORT-SECTION-UNSUPPORTED"),
])
def test_sections_outside_tranche_one_are_refused(section, code):
    data = mbf(extra=section)
    refused(run(data), code, layout_line(data, section[0]))


def test_all_zero_load_sections_are_omitted_and_check_sections_delegated():
    result = run(mbf(extra=("SEISMIC", "0,0,0", "FCURVE", "FC1,invented,2", "100,10", "50,1000")))
    seismic = listed(result, "MBF-IMPORT-SECTION-ZERO-OMITTED", "omitted")
    assert seismic["location"] == "SEISMIC"
    listed(result, "MBF-IMPORT-SECTION-DELEGATED", "delegated")
    assert result.summary["delegated_sections"] == {"FCURVE": 3}
    assert "FC1" not in json.dumps(result.to_dict())  # the delegated values are not copied


# --- §5, §6 options ------------------------------------------------------------------------------------------------
def test_options_vertical_axis_z_and_unknown_tokens_are_refused():
    data = mbf(options="HGRA,SI,Z")
    refused(run(data), "MBF-IMPORT-OPTION-UNSUPPORTED", layout_line(data, "HGRA"))
    refused(run(mbf(options="HGRA,SI,QQ")), "MBF-IMPORT-OPTION-UNSUPPORTED")


def test_hanger_and_code_options_are_pass_through_only():
    result = run(mbf(options="HGRB,B313,SI"))
    listed(result, "MBF-IMPORT-TARGET-OPTION-DELEGATED", "delegated")
    assert result.target_options["OPTIONS.piping_code"] == ["B313"]
    assert result.target_options["OPTIONS.hanger_design"] == ["HGRB"]
    assert "rule_pack_refs" not in result.model and "B313" not in json.dumps(result.model)


def test_rad_option_keeps_rotations_in_radians_and_the_default_in_degrees():
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X3000,KA,CROT(0,0,0.01)")
    degrees, radians = run(mbf(*rows)), run(mbf(*rows, options="HGRA,SI,RAD"))
    assert loads_of(degrees)[-1][-2:] == (0.01, "deg") and loads_of(radians)[-1][-2:] == (0.01, "rad")
    assert radians.model["project"]["units"]["angle"] == "rad"


def test_optemp_and_opmisc_records():
    data = mbf(optemp="20,7000,2")
    refused(run(data), "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED", layout_line(data, "20,7000"))
    refused(run(mbf(optemp="20,0,1,0,0,9")), "MBF-IMPORT-FIELD-UNDOCUMENTED")
    data = mbf(optemp=None)
    refused(run(data), "MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING")
    data = mbf(extra=("OPMISC", "33,20,1,1,1,2,2,1,9.80665,1"))
    refused(run(data), "MBF-IMPORT-TBD-17-04-005", layout_line(data, "33,20"))
    refused(run(mbf(extra=("OPMISC", "33,20,1,1,1,2,2,1,9.80665,1,0"))), "MBF-IMPORT-FIELD-UNDOCUMENTED")
    result = run(mbf(extra=("OPMISC", "", "OPPRES", "")))
    assert result.target_options["OPMISC"] == result.target_options["OPPRES"] == "section present with CAEPIPE defaults"
    result = run(mbf(extra=("OPCODE", "1,1,1", "OPMISC", "33,20,1,1,1,2,2,1,9.80665")))
    assert result.target_options["OPCODE"] == {"fields": ["1", "1", "1"]}
    listed(result, "MBF-IMPORT-TARGET-OPTION-DELEGATED", "delegated")


def test_the_gravitational_direction_is_refused_in_a_bound_import_too():
    data = mbf(extra=("OPMISC", "33,20,1,1,1,2,2,1,9.80665,1"))
    sidecar = {"mbf_sha256": hashlib.sha256(data).hexdigest(), "node_map": {}}
    assert run(data, sidecar=sidecar).source["binding"] == "bound"
    refused(run(data, sidecar=sidecar), "MBF-IMPORT-TBD-17-04-005")


# --- §7 materials ---------------------------------------------------------------------------------------------------
def test_material_records():
    data = mbf(material=("M1,0,0.3,1,1,FR,invented frp", "M1,20,1,2,3,4"))
    refused(run(data), "MBF-IMPORT-MATERIAL-FRP-UNSUPPORTED", layout_line(data, "M1,0"))
    data = mbf(material=("M1,0,0.3,1,1,CS,x", "M1,20,200000,1.2e-05", "M1,300,190000,1.3e-05"))
    refused(run(data), "MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT", layout_line(data, "M1,0"))
    data = mbf(material=("M1,0,0.5,1,1,CS,x", "M1,20,200000,1.2e-05"))
    refused(run(data), "MBF-IMPORT-VALUE-OUT-OF-RANGE", layout_line(data, "M1,0"))
    refused(run(mbf(material=("M1,-1,0.3", "M1,20,200000,1.2e-05"))), "MBF-IMPORT-VALUE-OUT-OF-RANGE")
    refused(run(mbf(material=("M1,0,0.3", "M1,20,0,1.2e-05"))), "MBF-IMPORT-VALUE-OUT-OF-RANGE")
    refused(run(mbf(material=("M1,0,0.3",))), "MBF-IMPORT-FIELD-MISSING")
    refused(run(mbf(material=("M1,0,0.3", "M1,20,200000,1.2e-05", "M2,0,0.3", "M2,20,1,1", "M1,0,0.3"))),
            "MBF-IMPORT-RECORD-REPEATED")
    refused(run(mbf(material=("M1,0,0.3,1,1,CS,x,0,0,0,0,0,FC1,9", "M1,20,200000,1.2e-05"))),
            "MBF-IMPORT-FIELD-UNDOCUMENTED")


@pytest.mark.parametrize("material, column", [
    (("M1,0,0.3,1,1,CS,x,30000", "M1,20,200000,1.2e-05"), "yield strength"),
    (("M1,0,0.3,1,1,CS,x,0,0,0,0,0,FC9", "M1,20,200000,1.2e-05"), "fatigue curve name"),
    (("M1,0,0.3,1,1,CS,x", "M1,20,200000,1.2e-05,137.9"), "allowable stress"),
    (("M1,0,0.3,1,1,CS,x", "M1,20,200000,1.2e-05,0,0,111"), "rupture stress"),
])
def test_strength_allowable_and_fatigue_data_refuse_the_whole_material(material, column):
    result = run(mbf(material=material))
    diagnostic = refused(result, "MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED")
    assert column in diagnostic["location"]
    report = json.dumps(result.to_dict())
    assert all(value not in report for value in ("30000", "FC9", "137.9", "111"))  # no value is copied


def test_material_provenance_must_be_affirmative():
    refused(run(mbf(), material_provenance=None), "MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED")
    for declaration in ({"origin": "vendor_library", "source_name": "x"}, {"origin": "unknown", "source_name": "x"},
                        {"origin": "project_invented"}, {"origin": "project_invented", "source_name": "x",
                                                         "redistribution_status": "protected_suspected"}):
        refused(run(mbf(), material_provenance={"M1": declaration}), "MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED")
    user = run(mbf(), material_provenance={"M1": {"origin": "user_supplied", "source_name": "my own test data"}})
    record = user.model["materials"][0]["provenance_record"]
    assert record["redistribution_status"] == "private_only" and record["source_name"] == "my own test data"
    assert record["source_location"] == f"sha256:{user.source['mbf_sha256']}"


def test_a_bound_sidecar_origin_qualifies_and_a_rebound_one_does_not():
    model = {"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
             "project": {"id": "invented", "units": {"length": "mm", "force": "N", "temperature": "degC"}},
             "analysis_status": {"mechanics": "x", "rule_check": "x", "professional_acceptance": "x"},
             "nodes": [{"id": "A", "position": {"x": 0, "y": 0, "z": 0}}, {"id": "B", "position": {"x": 1000, "y": 0, "z": 0}}],
             "materials": [{"id": "steel", "elastic_modulus": {"value": 200000, "unit": "MPa"},
                            "poisson_ratio": {"value": 0.3, "unit": "1"},
                            "provenance_record": {"source_name": "invented",
                                                  "contributor_certification": "invented non-engineering example",
                                                  "redistribution_status": "public_permissive"}}],
             "pipe_segments": [{"id": "S", "from": "A", "to": "B", "material": "steel",
                                "section": {"outside_diameter": {"value": 100, "unit": "mm"},
                                            "wall_thickness": {"value": 5, "unit": "mm"}}}],
             "supports": [{"id": "anchor", "node": "A", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
             "load_cases": [{"id": "c", "primitive_loads": [{"id": "f", "category": "concentrated_force",
                                                             "target": {"type": "node", "node": "B"}, "direction": "global_y",
                                                             "magnitude": {"value": -1, "unit": "N"}, "dimension": "force"}]}]}
    written = render_caepipe_mbf_si(model, title="ORIGIN")
    data = written.text.encode()
    sidecar = {**{key: value for key, value in written.to_dict().items() if key.endswith("_map")},
               "mbf_sha256": hashlib.sha256(data).hexdigest()}
    bound = run(data, sidecar=sidecar, material_provenance=None)
    assert bound.status == "imported", bound.diagnostics
    assert bound.model["materials"][0]["id"] == "steel"
    edited = data.replace(b"ORIGIN", b"EDITED")
    refused(run(edited, sidecar=sidecar, rebind_by_node_number=True, material_provenance=None),
            "MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED")


def test_material_description_is_the_label_and_unused_records_are_listed():
    result = run(mbf(material=("M1,0,0.3,1,1,CS,invented steel", "M1,20,200000,1.2e-05", "M9,0,0.3", "M9,20,1,0"),
                     pipe=("P1,168.3,7.11", "P9,100,5"), loads=("L1,20,0,0", "L9,30,0,0")))
    assert result.model["materials"][0]["label"] == "invented steel"
    unused = {item["location"] for item in result.loss_report if item["code"] == "MBF-IMPORT-RECORD-UNUSED"}
    assert unused == {"MATERIAL M9", "PIPE P9", "LOADS L9"}


# --- §8 sections, §9 loads ------------------------------------------------------------------------------------------
@pytest.mark.parametrize("pipe, code", [
    ("P1,168.3,7.11,1.5", "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED"),
    ("P1,168.3,7.11,0,12.5", "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED"),
    ("P1,168.3,7.11,0,0,0,0,0,0,S1", "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED"),
    ("P1,168.3,90", "MBF-IMPORT-VALUE-OUT-OF-RANGE"),
    ("P1,168.3,7.11,0,0,0,0,0,0,,9", "MBF-IMPORT-FIELD-UNDOCUMENTED"),
])
def test_section_records(pipe, code):
    data = mbf(pipe=(pipe,))
    refused(run(data), code, layout_line(data, "P1,"))


def test_zero_section_fields_are_written_as_absent():
    section = run(mbf(pipe=("P1,168.3,7.11,0,0.0,,0,0,0",))).model["pipe_segments"][0]["section"]
    assert section == {"outside_diameter": {"value": 168.3, "unit": "mm"}, "wall_thickness": {"value": 7.11, "unit": "mm"}}


@pytest.mark.parametrize("record, code", [
    ("L1,20,5,0", "MBF-IMPORT-PRESSURE-UNSUPPORTED"),
    ("L1,20,0,1", "MBF-IMPORT-CONTENTS-UNSUPPORTED"),
    ("L1,20,0,0,150,0", "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED"),
    ("L1,20,0,0,,,,,3", "MBF-IMPORT-LOAD-FIELD-UNSUPPORTED"),
    ("L1,20,0,0" + ",0" * 21 + ",1", "MBF-IMPORT-LOAD-FIELD-UNSUPPORTED"),
    ("L1,20,0,0" + ",0" * 32, "MBF-IMPORT-FIELD-UNDOCUMENTED"),
])
def test_load_records(record, code):
    data = mbf(loads=(record,))
    refused(run(data), code, layout_line(data, "L1,"))


def test_a_second_thermal_load_at_the_reference_is_inactive_and_peak_values_delegated():
    result = run(mbf(loads=("L1,100,0,0,20,0,,,0,150,10",)))
    inactive = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-LOAD-SLOT-INACTIVE"]
    assert [(item["severity"], item["location"]) for item in inactive] == [("info", "LOADS L1 T2/P2")]
    assert any(item["code"] == "MBF-IMPORT-TARGET-OPTION-DELEGATED" and "peak" in item["location"]
               for item in result.loss_report)
    assert ("case:mbf-expansion-t1", "thermal", "pipe:10-20", "global_x", 80.0, "degC") in loads_of(result)


# --- §10 layout -----------------------------------------------------------------------------------------------------
@pytest.mark.parametrize("rows, code, at", [
    (("T20,X3000",), "MBF-IMPORT-ROW-SYNTAX-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20A,X3000"), "MBF-IMPORT-NODE-NUMBER-INVALID", "T20A"),
    (("F10,KA,MM1,PP1,CL=L1", "T99999,X3000"), "MBF-IMPORT-NODE-NUMBER-INVALID", "T99999"),
    (("F10,KA,MM1,PP1,CL=L1", "T*20,X3000"), "MBF-IMPORT-TBD-17-04-009", "T*20"),
    (("F10,KA,MM1,PP1,CL=L1", "T2*0,X3000"), "MBF-IMPORT-TBD-17-04-009", "T2*0"),
    (("F10,KA,MM1,PP1,CL=L1", "T20*,X3000,Y0"), "MBF-IMPORT-NODE-POSITION-MISSING", "T20*"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "F30", "T40,X10"), "MBF-IMPORT-NODE-POSITION-MISSING", "F30"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "F20,X3002,Y0,Z0", "T30,Y10"),
     "MBF-IMPORT-FROM-COORDINATES-CONFLICT", "F20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T10,X-3000"), "MBF-IMPORT-TOPOLOGY-LOOP", "T10"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X0"), "MBF-IMPORT-GEOMETRY-ZERO-LENGTH", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "L30,CFOR(1,0,0),FLC=1"), "MBF-IMPORT-NODE-UNKNOWN", "L30"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "L20,X5"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "L20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000,X5"), "MBF-IMPORT-ROW-KEY-REPEATED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,JR,X3000"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KB,X3000"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "H20,X3000"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "H20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,Q5,X3000"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,B100,X3000"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,X3000", "T30,Z100"), "MBF-IMPORT-BEND-RADIUS-MISSING", "T20"),
    (("F10,KA,PP1,CL=L1", "T20,X3000"), "MBF-IMPORT-PROPERTY-MISSING", "T20"),
    (("F10,KA,MM1,PP2,CL=L1", "T20,X3000"), "MBF-IMPORT-REFERENCE-UNRESOLVED", "F10"),
    (("F10,KA,MM1,PP1", "T20,X3000"), "MBF-IMPORT-PROPERTY-MISSING", "T20"),
    (("F10,KA,MM1,PP1,CL=L7", "T20,X3000"), "MBF-IMPORT-REFERENCE-UNRESOLVED", "F10"),
    (("F10,KA,MM1,PP1,CL=L1", "F,X3000"), "MBF-IMPORT-NODE-NUMBER-INVALID", "F,"),
    (("F10*,KA,MM1,PP1,CL=L1", "T20,X3000"), "MBF-IMPORT-ROW-KEY-UNSUPPORTED", "F10*"),
])
def test_layout_refusals(rows, code, at):
    data = mbf(*rows)
    refused(run(data), code, layout_line(data, at))


def test_offsets_absolute_rows_branches_and_the_origin_rule():
    result = run(mbf("* invented comment", "F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30*,X3000,Y0,Z500",
                     "F20,X3000,Y0,Z0", "T40,Y2000", "F50,X0,Y0,Z-1000", "T60,Y-10.5,X1e3"))
    assert result.status == "imported", result.diagnostics
    assert {node["id"]: tuple(node["position"].values()) for node in result.model["nodes"]} == {
        "node:10": (0.0, 0.0, 0.0), "node:20": (3000.0, 0.0, 0.0), "node:30": (3000.0, 0.0, 500.0),
        "node:40": (3000.0, 2000.0, 0.0), "node:50": (0.0, 0.0, -1000.0), "node:60": (1000.0, -10.5, -1000.0)}
    assert [pipe["id"] for pipe in result.model["pipe_segments"]] == ["pipe:10-20", "pipe:20-30", "pipe:20-40", "pipe:50-60"]
    assert "invented comment" in result.model["project"]["notes"]


def test_a_branch_from_row_may_repeat_coordinates_within_half_a_printed_unit():
    assert run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "F20,X3000.4,Y0,Z0", "T30,Y10")).status == "imported"


def test_properties_and_the_load_reference_are_carried_until_changed():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,X1000,PP2,CL=L2", "T40,X1000",
                     pipe=("P1,168.3,7.11", "P2,114.3,6.02"), loads=("L1,20,0,0", "L2,70,0,0")))
    assert [pipe["section"]["outside_diameter"]["value"] for pipe in result.model["pipe_segments"]] == [168.3, 114.3, 114.3]
    assert [load[2] for load in loads_of(result) if load[1] == "thermal"] == ["pipe:20-30", "pipe:30-40"]
    assert any("PD-1" in note and "L=" in note for note in result.assumptions)


def test_a_file_without_loads_imports_with_a_tbd_entry():
    result = run(mbf("F10,KA,MM1,PP1", "T20,X3000", loads=None, optemp=None))
    listed(result, "MBF-IMPORT-LOADS-ABSENT", "tbd")


def test_unbound_imports_follow_the_base_rule_without_pd1(monkeypatch):
    monkeypatch.setattr(importer_module, "PD1_VERSIONS", ())
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000")
    refused(run(data), "MBF-IMPORT-TBD-17-04-007", layout_line(data, "F10"))
    data = mbf("F10,KA,MM1,PP1", "T20,X3000,CFOR(0,-1,0)", loads=None, optemp=None)
    refused(run(data), "MBF-IMPORT-TBD-17-04-007", layout_line(data, "T20"))
    assert run(mbf("F10,KA,MM1,PP1", "T20,X3000,CFOR(0,-1,0),FLC=1", loads=None, optemp=None)).status == "imported"


# --- §12 supports, §13 point loads -------------------------------------------------------------------------------------
def test_supports():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1,LA=0", "T20,X3000,CFIXD(1,0,1),FIXRY,TAG=invented guide-like"))
    assert [(item["id"], item["restraints"], item.get("family"), item["label"]) for item in result.model["supports"]] == [
        ("support:10", ["UX", "UY", "UZ", "RX", "RY", "RZ"], "anchor", "anchor at 10"),
        ("support:20-restraint", ["UX", "UZ", "RY"], None, "invented guide-like")]
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,X10,CFIXD(1,1,1),FIXR(1,1,1)"))
    assert result.model["supports"][-1]["restraints"] == ["UX", "UY", "UZ", "RX", "RY", "RZ"]  # kept as written
    assert "family" not in result.model["supports"][-1]
    listed(run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CTAG=lonely")), "MBF-IMPORT-TAG-WITHOUT-SUPPORT", "omitted")


@pytest.mark.parametrize("comment, code", [
    ("FIXD(2,0,0)", "MBF-IMPORT-COMMENT-INVALID"),
    ("FIXD(1,0)", "MBF-IMPORT-COMMENT-INVALID"),
    ("F", "MBF-IMPORT-COMMENT-INVALID"),
    ("G", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("VS", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("TRAK(1,2,3)", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("NOZZLE=650", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("LTAG=L1", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("LA=1", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
    ("D(0,1,0)", "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED"),
    ("DIS2(0,1,0)", "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED"),
    ("SETT(0,1,0)", "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED"),
    ("LD=1", "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED"),
    ("CWGT=200", "MBF-IMPORT-LOAD-UNSUPPORTED"),
    ("U=5", "MBF-IMPORT-LOAD-UNSUPPORTED"),
    ("T=150", "MBF-IMPORT-LOAD-UNSUPPORTED"),
    ("SIF=2", "MBF-IMPORT-COMMENT-UNSUPPORTED"),
    ("FF=1.5", "MBF-IMPORT-COMMENT-INVALID"),
    ("FOR(0,1,0),FLC=2", "MBF-IMPORT-FORCE-CASE-UNSUPPORTED"),
    ("FLC=1", "MBF-IMPORT-COMMENT-INVALID"),
    ("NOSUCH=1", "MBF-IMPORT-COMMENT-UNKNOWN"),
    ("fx", "MBF-IMPORT-COMMENT-INVALID"),
])
def test_comment_refusals(comment, code):
    data = mbf("F10,KA,MM1,PP1,CL=L1", f"T20,X3000,C{comment}")
    refused(run(data), code, layout_line(data, "T20"))


def test_point_loads_displacements_and_their_cases():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(0,-100,0),FORX=5,FLC=1", "L20,CFOR(0,-50,0),FLC=1",
                     "T30,X1000,KA,CD(0,5,0),DX=0,ROT(0,0,0.5),MOMZ=7"))
    assert loads_of(result) == [
        ("case:mbf-sustained", "concentrated_force", "node:20", "global_x", 5.0, "N"),
        ("case:mbf-sustained", "concentrated_force", "node:20", "global_y", -150.0, "N"),  # summed across rows
        ("case:mbf-sustained", "concentrated_moment", "node:30", "RZ", 7.0, "N*m"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:30", "UY", 5.0, "mm"),  # zero entries give nothing
        ("case:mbf-expansion-t1", "imposed_displacement", "support:30", "RZ", 0.5, "deg"),
    ]
    assert [load["id"] for case in result.model["load_cases"] for load in case["primitive_loads"]][:2] == [
        "load:concentrated_force:node:20:global_x", "load:concentrated_force:node:20:global_y"]
    assert [case["label"] for case in result.model["load_cases"]] == ["Sustained (W+P)", "Expansion (T1)"]
    assert "combinations" not in result.model


# --- §11 bends -------------------------------------------------------------------------------------------------------
L_BEND = ("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,KA,Z3000")


def test_bend_geometry_is_rebuilt_from_the_tangent_intersection():
    # Hand-derived: TI (4000, 0, 0), legs along +X and +Z, theta = 90 deg, t = 250 tan 45 = 250:
    # near (3750, 0, 0), far (4000, 0, 250), y_reference = unit((4000,0,0) - (3875,0,125)) = (1, 0, -1)/sqrt 2.
    # binary64 tan(pi/4) is 1 - 2^-53, so t = 250 - 2.8e-14: within one rounding of the exact value.
    result = run(mbf(*L_BEND))
    nodes = {node["id"]: tuple(node["position"].values()) for node in result.model["nodes"]}
    assert nodes == {"node:10": (0.0, 0.0, 0.0), "node:20A": pytest.approx((3750.0, 0.0, 0.0), abs=1e-12),
                     "node:20B": pytest.approx((4000.0, 0.0, 250.0), abs=1e-12), "node:30": (4000.0, 0.0, 3000.0)}
    pipes = {pipe["id"]: (pipe["from"], pipe["to"]) for pipe in result.model["pipe_segments"]}
    assert pipes == {"pipe:10-20A": ("node:10", "node:20A"), "pipe:20A-20B": ("node:20A", "node:20B"),
                     "pipe:20B-30": ("node:20B", "node:30")}
    bend_pipe = result.model["pipe_segments"][1]
    assert tuple(bend_pipe["y_reference"].values()) == pytest.approx((math.sqrt(0.5), 0.0, -math.sqrt(0.5)), abs=1e-16)
    component = result.model["components"][0]
    assert (component["id"], component["kind"], component["node"]) == ("component:bend-20", "bend", "node:20A")
    assert component["geometry"]["bend_radius"] == {"value": 250.0, "unit": "mm"}
    assert component["modifiers"]["flexibility_factor_user_value"] == {"value": 1.0, "unit": "none"}
    assert component["mechanics_interface"] == {"solver_consumption": "curved_bend_macro_element"}
    assert "bend_angle" not in component["geometry"]


def test_a_bend_without_ff_imports_with_an_open_issue():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000", "T30,KA,Z3000"))
    listed(result, "MBF-IMPORT-BEND-FLEXIBILITY-MISSING", "omitted")
    component = result.model["components"][0]
    assert "flexibility_factor_user_value" not in component["modifiers"]
    assert result.model["unresolved_assumptions"] == [{
        "assumption_id": "assumption:mbf-bend-20-flexibility", "kind": "open_issue",
        "statement": "bend 20 has no flexibility factor in the MBF; enter a user factor", "status": "unresolved",
        "affected_refs": ["component:bend-20"]}]


def test_two_bends_with_a_straight_between_them():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,KI,B250,Z3000,CFF=1", "T40,KA,X-2000"))
    assert result.status == "imported", result.diagnostics
    assert [pipe["id"] for pipe in result.model["pipe_segments"]] == [
        "pipe:10-20A", "pipe:20A-20B", "pipe:20B-30A", "pipe:30A-30B", "pipe:30B-40"]


@pytest.mark.parametrize("rows, code, at", [
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "F30,X0,Y0,Z5000", "T40,X10"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,Z3000", "F20", "T40,Y100"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,FIXD(0,1,0)", "T30,Z3000"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "L20,CFOR(0,1,0),FLC=1", "T30,Z3000"),
     "MBF-IMPORT-BEND-UNSUPPORTED", "L20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,X3000"), "MBF-IMPORT-BEND-GEOMETRY-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X200,CFF=1", "T30,Z3000"), "MBF-IMPORT-BEND-GEOMETRY-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,KI,B250,Z400,CFF=1", "T40,X-2000"),
     "MBF-IMPORT-BEND-GEOMETRY-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,IN1(300,30)", "T30,Z3000"), "MBF-IMPORT-BEND-UNSUPPORTED", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B-1,X4000,CFF=1", "T30,Z3000"), "MBF-IMPORT-VALUE-OUT-OF-RANGE", "T20"),
])
def test_bend_refusals(rows, code, at):
    data = mbf(*rows)
    refused(run(data), code, layout_line(data, at))


# --- exact conversions with hand-derived expected values (§5.3, §7.2, §7.3, §9.2) -------------------------------------------
def test_si_weight_density_becomes_weight_per_length_by_standard_gravity():
    result = run(mbf(material=("M1,7850,0.3", "M1,20,200000,1.2e-05")))
    weight = next(load for load in loads_of(result) if load[1] == "weight")
    area = math.pi / 4 * (0.1683**2 - 0.15408**2)  # m^2, OD 168.3 mm, ID 154.08 mm
    assert weight[3:] == ("global_y", pytest.approx(-7850 * 9.80665 * area, rel=1e-15), "N/m")
    # By hand: 0.1683^2 - 0.15408^2 = 0.0045842436 m^2, times pi/4 = 3.60046e-3 m^2; 7850 * 9.80665 * that = 277.171.
    assert weight[4] == pytest.approx(-277.1711, rel=1e-6)


def test_english_weight_density_becomes_weight_per_length_without_conversion():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X10", options="HGRA",
                     material=("M1,0.283,0.3", "M1,70,29000000,6.5e-06"), pipe=("P1,6.625,0.28",),
                     loads=("L1,70,0,0",), optemp="70"))
    weight = next(load for load in loads_of(result) if load[1] == "weight")
    assert weight[3:] == ("global_y", pytest.approx(-0.283 * math.pi / 4 * (6.625**2 - 6.065**2), rel=1e-15), "lbf/in")
    assert weight[4] == pytest.approx(-1.5795, rel=1e-4)  # by hand: 0.283 lbf/in3 * 5.5814 in2
    material = result.model["materials"][0]
    assert material["elastic_modulus"] == {"value": 29000000.0, "unit": "psi"}
    assert material["thermal_expansion_coefficient"] == {"value": 6.5e-06, "unit": "1/degF"}
    # G in Pa from psi by the catalog factor 4.4482216152605 N / 0.00064516 m^2, then E/(2(1+nu)).
    e_pa = 29000000.0 * (4.4482216152605 / 0.00064516)
    assert material["shear_modulus"] == {"value": float(Fraction(e_pa) / Fraction(13, 5)), "unit": "Pa"}
    assert material["shear_modulus"]["value"] == pytest.approx(7.6904e10, rel=1e-4)
    assert result.model["project"]["units"] == {"length": "in", "force": "lbf", "moment": "lbf*in", "pressure": "psi",
                                                "stress": "psi", "temperature": "degF", "angle": "deg"}


def test_g_is_derived_in_pa_and_the_basis_is_recorded():
    material = run(mbf()).model["materials"][0]
    assert material["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1"
    assert material["elastic_modulus"] == {"value": 200000.0, "unit": "MPa"}
    assert material["poisson_ratio"] == {"value": 0.3, "unit": "1"}
    assert material["shear_modulus"] == {"value": 76923076923.07692, "unit": "Pa"}  # 2e11 / 2.6, correctly rounded
    assert "derived from E and nu (isotropic)" in material["provenance_record"]["verification_note"]


def test_temperature_change_is_the_exact_difference_rounded_once():
    result = run(mbf(loads=("L1,20.12345679,0,0",)))
    delta = next(load for load in loads_of(result) if load[1] == "thermal")[4]
    assert delta == float(Fraction("20.12345679") - 20) == 0.12345679
    assert 20.12345679 - 20 != 0.12345679  # the binary64 difference would not be correctly rounded
    assert not [load for load in loads_of(run(mbf(loads=("L1,20,0,0",)))) if load[1] == "thermal"]


def test_kgf_is_exactly_standard_gravity_newtons():
    assert importer_module.STANDARD_GRAVITY == 9.80665


# --- sidecar binding and ids (§15) ------------------------------------------------------------------------------------
def test_unbound_ids_are_formed_from_caepipe_names():
    result = run(mbf(*L_BEND))
    assert result.id_map["node_map"] == {"10": "node:10", "20A": "node:20A", "20B": "node:20B", "30": "node:30"}
    assert result.id_map["bend_map"] == {"20": "component:bend-20"}
    assert result.id_map["support_map"] == {"10": ["support:10"], "30": ["support:30"]}
    assert result.id_map["material_map"] == {"M1": "material:M1"}
    assert result.id_map["binding"] == "unbound" and result.id_map["rebound_unconfirmed"] == []


def test_a_sidecar_with_another_hash_is_not_applied_unless_rebinding_is_requested():
    data = mbf(*L_BEND)
    sidecar = {"mbf_sha256": "0" * 64, "node_map": {"A": "10", "T1": "20A", "T2": "20B", "B": "30"},
               "support_map": {"SA": {"node": "10", "restrained": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "family": "anchor"},
                               "GONE": {"node": "99", "restrained": ["UY"], "family": None}}}
    plain = run(data, sidecar=sidecar)
    assert plain.source["binding"] == "unbound" and plain.model["nodes"][0]["id"] == "node:10"
    assert any(item["code"] == "MBF-IMPORT-SIDECAR-NOT-APPLIED" for item in plain.diagnostics)
    rebound = run(data, sidecar=sidecar, rebind_by_node_number=True)
    assert rebound.source["binding"] == "rebound_unconfirmed"
    assert [node["id"] for node in rebound.model["nodes"]] == ["A", "T1", "T2", "B"]
    assert {"A", "T1", "T2", "B", "SA"} <= set(rebound.id_map["rebound_unconfirmed"])
    assert any(item["code"] == "MBF-IMPORT-SIDECAR-ENTRY-ABSENT" and "GONE" in item["location"]
               for item in rebound.loss_report)
    conflict = {**sidecar, "node_map": {"A": "10", "B": "10"}}
    refused(run(data, sidecar=conflict, rebind_by_node_number=True), "MBF-IMPORT-SIDECAR-CONFLICT")
    refused(run(data, rebind_by_node_number=True), "MBF-IMPORT-SIDECAR-INVALID")


# --- outputs, determinism and the report (§3, §16) -----------------------------------------------------------------
def test_identical_inputs_give_identical_bytes_and_the_report_is_typed():
    first, second = run(mbf(*L_BEND)), run(mbf(*L_BEND))
    assert (first.model_bytes(), first.report_bytes(), first.id_map_bytes()) == (
        second.model_bytes(), second.report_bytes(), second.id_map_bytes())
    report = first.to_dict()
    assert report["document_kind"] == REPORT_KIND and report["profile_id"] == IMPORT_PROFILE_ID
    assert report["source"]["mbf_sha256"] == hashlib.sha256(mbf(*L_BEND)).hexdigest()
    assert report["source"]["unit_mode"] == "SI" and report["source"]["mbf_version"] == "15.10"
    assert first.model["project"]["id"] == f"project:mbf-{report['source']['mbf_sha256'][:12]}"
    assert all(entity["provenance"].startswith(f"imported from MBF sha256:{report['source']['mbf_sha256']}, line ")
               for key in ("nodes", "pipe_segments", "supports", "components", "materials")
               for entity in first.model[key])
    assert first.model["diagnostics"] == [] and first.model["data_boundary"]["private_data_policy"].endswith(
        "local user document not committed to repository")
    other = run(mbf(*L_BEND), material_provenance={"*": {"origin": "project_invented", "source_name": "another"}})
    assert other.model_bytes() != first.model_bytes()  # the material-provenance declaration is an input


def test_a_refused_import_reports_no_values_from_refused_records():
    result = run(mbf(loads=("L1,20,12.345,0",)))
    refused(result, "MBF-IMPORT-PRESSURE-UNSUPPORTED")
    assert "12.345" not in result.report_bytes().decode()


@pytest.mark.parametrize("end, expected", [
    ((3000.0, 0.0, 0.0), (0.0, 1.0, 0.0)),  # horizontal: global Y
    ((0.0, 0.0, -3000.0), (0.0, 1.0, 0.0)),
    ((1000.0, 1000.0, 0.0), (-math.sqrt(0.5), math.sqrt(0.5), 0.0)),  # skew: Y projected off the axis
    ((0.0, 3000.0, 0.0), (-1.0, 0.0, 0.0)),  # vertical: global -X
    ((0.0, -3000.0, 0.0), (-1.0, 0.0, 0.0)),
])
def test_documented_local_y_of_a_straight(end, expected):
    # CAEPIPE Technical Reference, Local Coordinate System (CAEPIPE-LOCAL-AXES), global Y vertical.
    assert importer_module.local_y_reference((0.0, 0.0, 0.0), end) == pytest.approx(expected, abs=1e-15)


def test_the_vertical_threshold_of_the_local_y_rule():
    assert importer_module.VERTICAL_SINE == 1e-6
    inside = importer_module.local_y_reference((0, 0, 0), (1e-7, 1.0, 0.0))  # horizontal part 1e-7 <= 1e-6
    assert inside == (-1.0, 0.0, 0.0)
    outside = importer_module.local_y_reference((0, 0, 0), (1e-5, 1.0, 0.0))  # 1e-5 > 1e-6: Y projected
    # x = (s, c, 0) = (1e-5, 1, 0)/sqrt(1 + 1e-10): Y - (Y.x) x = (-c s, s^2, 0), whose unit vector is (-c, s, 0).
    norm = math.sqrt(1 + 1e-10)
    assert outside == pytest.approx((-1.0 / norm, 1e-5 / norm, 0.0), abs=1e-15)


def test_imported_straights_take_the_documented_local_y_and_report_it():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,Y3000", "T40,X1000,Z1000"))
    assert [tuple(pipe["y_reference"].values()) for pipe in result.model["pipe_segments"]] == [
        (0.0, 1.0, 0.0), (-1.0, 0.0, 0.0), pytest.approx((0.0, 1.0, 0.0))]
    defaulted = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-ORIENTATION-DEFAULTED"]
    assert [item["location"] for item in defaulted] == ["pipe:10-20", "pipe:20-30", "pipe:30-40"]
    assert all(item["severity"] == "info" and "CAEPIPE-LOCAL-AXES" in item["message"]
               and "only local-axis results" in item["message"] for item in defaulted)
    assert '"-0.0' not in result.model_bytes().decode()
    bend = run(mbf(*L_BEND))  # the bend pipe's y_reference comes from its geometry, not the default
    assert [item["location"] for item in bend.diagnostics if item["code"] == "MBF-IMPORT-ORIENTATION-DEFAULTED"] == [
        "pipe:10-20A", "pipe:20B-30"]


def test_a_bound_import_restores_straight_orientation_from_the_sidecar():
    model = {"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
             "project": {"id": "invented", "units": {"length": "mm", "force": "N", "temperature": "degC"}},
             "analysis_status": {"mechanics": "x", "rule_check": "x", "professional_acceptance": "x"},
             "nodes": [{"id": "A", "position": {"x": 0, "y": 0, "z": 0}}, {"id": "B", "position": {"x": 1000, "y": 0, "z": 0}}],
             "materials": [{"id": "steel", "elastic_modulus": {"value": 200000, "unit": "MPa"},
                            "poisson_ratio": {"value": 0.3, "unit": "1"}}],
             "pipe_segments": [{"id": "S", "from": "A", "to": "B", "material": "steel", "y_reference": {"x": 0, "y": 0, "z": 1},
                                "section": {"outside_diameter": {"value": 100, "unit": "mm"},
                                            "wall_thickness": {"value": 5, "unit": "mm"}}}],
             "supports": [{"id": "anchor", "node": "A", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
             "load_cases": [{"id": "c", "primitive_loads": [{"id": "f", "category": "concentrated_force",
                                                             "target": {"type": "node", "node": "B"}, "direction": "global_y",
                                                             "magnitude": {"value": -1, "unit": "N"}, "dimension": "force"}]}]}
    written = render_caepipe_mbf_si(model, title="ORIENTATION")
    assert written.element_map["S"]["y_reference"] == {"x": 0.0, "y": 0.0, "z": 1.0}
    data = written.text.encode()
    sidecar = {**{key: value for key, value in written.to_dict().items() if key.endswith("_map")},
               "mbf_sha256": hashlib.sha256(data).hexdigest()}
    bound = run(data, sidecar=sidecar)
    assert bound.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 0.0, "z": 1.0}
    assert "MBF-IMPORT-ORIENTATION-DEFAULTED" not in {item["code"] for item in bound.diagnostics}
    assert run(data).model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 1.0, "z": 0.0}  # unbound: default
    # A sidecar orientation that is not a usable vector (here zero length) is not restored: the documented local y
    # for a straight along +X is global Y, and the assignment is reported.
    sidecar["element_map"]["S"]["y_reference"] = {"x": 0.0, "y": 0.0, "z": 0.0}
    zero = run(data, sidecar=sidecar)
    assert zero.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 1.0, "z": 0.0}
    assert "MBF-IMPORT-ORIENTATION-DEFAULTED" in {item["code"] for item in zero.diagnostics}


# --- command line (§3, §16.4) -------------------------------------------------------------------------------------------
def test_command_line_refuses_outputs_in_a_git_work_tree_for_an_undeclared_input(tmp_path, capsys):
    repo = tmp_path / "repo"
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    source = tmp_path / "plain.mbf"
    source.write_bytes(mbf())
    assert main([str(source), "--out", str(repo / "out")]) == 2
    assert not (repo / "out").exists()
    declaration = tmp_path / "declaration.json"
    declaration.write_text(json.dumps(INVENTED), encoding="utf-8")
    assert main([str(source), "--out", str(tmp_path / "private"), "--material-provenance", str(declaration)]) == 0
    assert sorted(path.name for path in (tmp_path / "private").iterdir()) == ["id_map.json", "import_report.json", "model.json"]
    declared = json.loads((FIXTURE.with_name("units_si.provenance.json")).read_text(encoding="utf-8"))
    declaration.write_text(json.dumps(declared["material_provenance"]), encoding="utf-8")
    assert main([str(FIXTURE), "--out", str(repo / "fixture-out"), "--material-provenance", str(declaration)]) == 0
    out = capsys.readouterr().out
    assert out.strip().splitlines()[-1].startswith("importer: imported; outputs in ")


# --- binary64 range (independent review VER-002 r1, point 2) -----------------------------------------------------
@pytest.mark.parametrize("change, at, why", [
    # E = 1e303 MPa is 1e309 Pa, beyond the largest binary64 (1.80e308); it raised OverflowError before.
    (dict(material=("M1,0,0.3", "M1,20,1e303,1.2e-05")), "M1,0", "derived E in Pa overflows"),
    # Density 1e-400 underflows to 0 when read; it imported before with no weight.
    (dict(material=("M1,1e-400,0.3", "M1,20,200000,1.2e-05")), "M1,1e-400", "a non-zero value read as 0"),
    # Density 5e-324 (the smallest subnormal) is readable, but 5e-324 x 9.80665 x 3.6e-3 m^2 underflows to 0.
    (dict(material=("M1,5e-324,0.3", "M1,20,200000,1.2e-05")), "T20", "derived weight underflows to 0"),
    # 1e308 + 1e308 = 2e308 mm overflows the node coordinate.
    (dict(layout=("F10,KA,MM1,PP1,X1e308,Y0,Z0,CL=L1", "T20,X1e308")), "T20", "node coordinate overflows"),
    # T1 - T_ref = 1e308 - (-1e308) = 2e308 C overflows the temperature change.
    (dict(loads=("L1,1e308,0,0",), optemp="-1e308"), "T20", "temperature change overflows"),
    # Two forces of 1e308 N on one node sum to 2e308 N.
    (dict(layout=("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(0,1e308,0),FLC=1", "L20,CFOR(0,1e308,0),FLC=1")),
     "T20", "summed load overflows"),
    # A value beyond binary64 in the file itself.
    (dict(pipe=("P1,1e400,7.11",)), "P1,", "a value read as infinity"),
])
def test_values_outside_the_binary64_range_are_refused_by_name(change, at, why):
    layout = change.pop("layout", ())
    data = mbf(*layout, **change)
    refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, at))


def test_positions_are_validated_after_rounding():
    # 1e20 + 1e-10 mm rounds back to 1e20 (the binary64 spacing at 1e20 is 16384): distinct exact positions, zero
    # length once rounded.
    data = mbf("F10,KA,MM1,PP1,X1e20,Y0,Z0,CL=L1", "T20,X1e-10")
    refused(run(data), "MBF-IMPORT-GEOMETRY-ZERO-LENGTH", layout_line(data, "T20"))
    # A 1e-200 mm straight is representable; its unit direction uses a scaled norm (hypot) and no longer divides
    # by zero: length 1e-200, direction +X, documented local y = global Y.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1e-200"))
    assert result.status == "imported", result.diagnostics
    assert result.model["nodes"][1]["position"] == {"x": 1e-200, "y": 0.0, "z": 0.0}
    assert result.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 1.0, "z": 0.0}
    assert importer_module.local_y_reference((0, 0, 0), (0, 1e-200, 0)) == (-1.0, 0.0, 0.0)


def test_a_tiny_but_representable_density_still_gives_weight():
    # 1e-300 kgf/m3 x 9.80665 x A (3.6005e-3 m^2) = 3.53e-302 N/m, a normal binary64: kept, not dropped.
    result = run(mbf(material=("M1,1e-300,0.3", "M1,20,200000,1.2e-05")))
    weight = next(load for load in loads_of(result) if load[1] == "weight")
    assert weight[4] == pytest.approx(-1e-300 * 9.80665 * math.pi / 4 * (0.1683**2 - 0.15408**2), rel=1e-14)


def test_derived_bend_geometry_is_validated_after_rounding():
    # Codex VER-002 r2: R = 5e-324 mm (the smallest subnormal); legs X3000 then X3000,Z1, so theta = atan(1/3000)
    # = 3.33e-4 rad and t = 5e-324 x tan(1.67e-4) = 8e-328 mm, below half the smallest subnormal: it rounds to 0.
    # It raised ZeroDivisionError before.
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B5e-324,X3000,CFF=1", "T30,X3000,Z1")
    diagnostic = refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T20"))
    assert "tangent length" in diagnostic["message"]
    # R = 1e-20 mm at a 90 degree corner: t = 1e-20 mm, but 3000 - 1e-20 rounds to 3000 (the binary64 spacing at 3000
    # is 4.5e-13), so the near tangent node collapses onto the tangent intersection.
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B1e-20,X3000,CFF=1", "T30,Z3000")
    refused(run(data), "MBF-IMPORT-BEND-GEOMETRY-INVALID", layout_line(data, "T20"))
    # A small but resolvable bend still imports: R = 1e-6 mm at x = 3000 gives near = 2999.999999 mm.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B1e-6,X3000,CFF=1", "T30,Z3000"))
    assert result.status == "imported", result.diagnostics
    assert result.model["nodes"][1]["position"]["x"] == 3000 - 1e-6


# --- derived coordinates are exact until their one rounding (independent review VER-002 r3) --------------------
def test_a_tangent_node_coordinate_below_binary64_is_refused_not_rounded_to_zero():
    # Codex VER-002 r3: start (-1, -1e-200, 0), TI (0, 0, 0), next (0, 0, 1); the legs are perpendicular, so
    # t = 1e-200 x tan 45 deg ~ 1e-200 mm. By hand the near node's Y is 0 - t x (1e-200 / |(1, 1e-200, 0)|), about
    # -1e-400 mm: non-zero and below binary64. The binary64 product rounded it to 0 and the import succeeded.
    data = mbf("F10,KA,MM1,PP1,X-1,Y-1e-200,Z0,CL=L1", "T20,KI,B1e-200,X1,Y1e-200,CFF=1", "T30,Z1")
    diagnostic = refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T20"))
    assert "tangent node" in diagnostic["message"] and diagnostic["location"] == "bend 20"


def test_offsets_summed_along_a_run_are_exact_until_one_rounding():
    # By hand: Y1e-200 at the From row, then an offset of -(1e-200 - 1e-400): the exact position is 1e-400 mm,
    # non-zero and below binary64, so it is refused rather than imported as 0.
    offset = "-0." + "9" * 200 + "e-200"  # -(1 - 1e-200) x 1e-200 = -(1e-200 - 1e-400)
    data = mbf("F10,KA,MM1,PP1,X0,Y1e-200,Z0,CL=L1", f"T20,X3000,Y{offset}")
    refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T20"))
    # The same sum that cancels exactly imports at 0: 1e-200 - 1e-200.
    result = run(mbf("F10,KA,MM1,PP1,X0,Y1e-200,Z0,CL=L1", "T20,X3000,Y-1e-200"))
    assert result.model["nodes"][1]["position"] == {"x": 3000.0, "y": 0.0, "z": 0.0}


def test_absolute_coordinates_and_english_fractions_are_read_exactly():
    # An absolute-coordinate row gives its value directly: Y1e-400 is refused when read.
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20*,X3000,Y1e-400,Z0")
    refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T20*"))
    # A feet-inch fraction of 1/10^400 in (0'0-1/1000...0") is exact as a rational and below binary64 once rounded.
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X10", "T30,Y0'0-1/1" + "0" * 400 + '"', options="HGRA")
    refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T30"))
    # Exact fractions sum without rounding: three offsets of 0'0-1/3" make exactly 1 in.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", 'T20,X0\'0-1/3"', 'T30,X0\'0-1/3"', 'T40,X0\'0-1/3"', options="HGRA"))
    assert result.model["nodes"][3]["position"]["x"] == 1.0



# --- CAEPIPE's own MBF export: full positional records (Design §6, §9.1, §10.1) -------------------------------------
# Invented values in the shape the export writes (observed format behaviour): every OPTIONS, OPCODE, OPTEMP, OPPRES
# and OPMISC field present, an empty FCYCLE, full MATERIAL rows with zero strength columns, and LOADS with every
# T1..T10 and pressure slot filled; unused temperature slots hold a default temperature and unused pressures 0.
def export_shaped(*, optemp="15.5,5000,1,0,1", opmisc="33,20,1,1,1,2,2,2,0,386.4000,0.00,-1.00,0.00",
                  loads_record=None, layout=("F10,KA,X-2000,CTAG=,LTAG=", "T20,MM1,PP1,X2000,CL=L1",
                                             "T30,KA,X2500,CTAG=,LTAG=")) -> bytes:
    inactive = ",".join(["17.25,0"] * 7)
    record = loads_record or f"L1,95.5,0,0,15.5,0,15.5,0,0,95.5,0,{inactive},0,0,0,0"
    lines = ["INVENTED EXPORT-SHAPED MODEL", "", "VERSION", "15.10", "OPTIONS", "HGRA,B311,SI", "OPCODE",
             "1,0,1,0,0,0,1,0,1.20", "OPTEMP", optemp, "OPPRES", "1,0,1,1.00,0", "OPMISC", opmisc, "FCYCLE",
             "MATERIAL", "M1,7000,0.300,1.00,1.00,CS,INVENTED-STEEL,0.000,0.000,0.00,0.000,0.000",
             "M1,15.5,190000,11.00E-6,0.000", "M1,115.5,190000,11.00E-6,0.000", "PIPE", "P1,114.3,6.02,0,0.0",
             "LOADS", record, "LAYOUT", *layout]
    return "".join(line + "\r\n" for line in lines).encode("ascii")


def test_an_export_shaped_file_imports_with_inactive_slots_dropped_by_slot():
    result = run(export_shaped())
    assert result.status == "imported", result.diagnostics
    inactive = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-LOAD-SLOT-INACTIVE"]
    assert [item["location"] for item in inactive] == [f"LOADS L1 T{n}/P{n}" for n in range(2, 11)]
    assert all(item["severity"] == "info" and "OPTEMP gives 1 thermal load" in item["message"] for item in inactive)
    # By hand: only T1 is active, so the thermal change is T1 - T_ref = 95.5 - 15.5 = 80 C on each element.
    assert [load[4] for load in loads_of(result) if load[1] == "thermal"] == [80.0, 80.0]
    # The From row gives only X: Y and Z are 0, and the node lies at (-2000, 0, 0).
    assert result.model["nodes"][0]["position"] == {"x": -2000.0, "y": 0.0, "z": 0.0}
    locations = {item["location"] for item in result.loss_report if item["category"] == "delegated"}
    assert {"OPMISC fields 1-8", "OPMISC field 9", "OPMISC gravity", "OPTIONS piping_code", "FCYCLE"} <= locations
    assert any("earth gravity" in note for note in result.assumptions)


def test_the_default_earth_gravity_leaves_the_weights_unchanged():
    # By hand: 0.1143^2 - 0.10226^2 = 0.0026073824 m^2, x pi/4 = 2.04782e-3 m^2; x 7000 kgf/m3 x 9.80665 m/s^2 =
    # 140.577 N/m, with or without the OPMISC default.
    weights = []
    for opmisc in ("33,20,1,1,1,2,2,2,0,386.4000,0.00,-1.00,0.00", "33,20,1,1,1,2,2,2"):
        result = run(export_shaped(opmisc=opmisc))
        weights.append([load[4] for load in loads_of(result) if load[1] == "weight"])
    expected = -7000 * 9.80665 * math.pi / 4 * (0.1143**2 - 0.10226**2)
    assert weights[0] == weights[1] == [pytest.approx(expected, rel=1e-15)] * 2
    assert expected == pytest.approx(-140.577, rel=1e-5)


@pytest.mark.parametrize("opmisc, code", [
    ("33,20,1,1,1,2,2,2,1,386.4000,0.00,-1.00,0.00", "MBF-IMPORT-FIELD-UNDOCUMENTED"),  # field 9 not 0
    ("33,20,1,1,1,2,2,2,0,193.2,0.00,-1.00,0.00", "MBF-IMPORT-GRAVITY-UNSUPPORTED"),  # half gravity
    ("33,20,1,1,1,2,2,2,0,386.4,0.00,0.00,-1.00", "MBF-IMPORT-GRAVITY-UNSUPPORTED"),  # along -Z
    ("33,20,1,1,1,2,2,2,0,,0.00,-1.00,0.00", "MBF-IMPORT-FIELD-MISSING"),  # a direction without an acceleration
    ("33,20,1,1,1,2,2,2,0,386.4,0.00,-1.00", "MBF-IMPORT-FIELD-UNDOCUMENTED"),  # 12 fields
    ("33,20,1,1,1,2,2,2,0,386.4,0.00,-1.00,0.00,0", "MBF-IMPORT-FIELD-UNDOCUMENTED"),  # 14 fields
])
def test_opmisc_in_the_export_layout_reads_only_the_default_setting(opmisc, code):
    data = export_shaped(opmisc=opmisc)
    refused(run(data), code, layout_line(data, opmisc))


def test_slots_follow_the_thermal_load_count_or_else_the_conservative_rule():
    # With OPTEMP's count of 1, an inactive slot is dropped even with a non-zero pressure (no load case uses it),
    # but its fields must still be numbers.
    inactive = ",".join(["17.25,0"] * 6)
    result = run(export_shaped(loads_record=f"L1,95.5,0,0,15.5,0,15.5,0,0,95.5,0,{inactive},17.25,3.5,0,0,0,0"))
    assert result.status == "imported", result.diagnostics
    data = export_shaped(loads_record=f"L1,95.5,0,0,15.5,0,15.5,0,0,95.5,0,{inactive},17.25,x,0,0,0,0")
    refused(run(data), "MBF-IMPORT-NUMBER-INVALID", layout_line(data, "L1,"))
    # Without a count (OPTEMP gives only the reference temperature), a slot is inactive only when its temperature is
    # the reference and its pressure 0; the export's default 17.25 differs, so the slot is ambiguous and refused.
    data = export_shaped(optemp="15.5")
    refused(run(data), "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED", layout_line(data, "L1,"))
    data = export_shaped(optemp="15.5", loads_record="L1,95.5,0,0,15.5,0,15.5,0,0,95.5,0")
    inactive_locations = [item["location"] for item in run(data).diagnostics
                          if item["code"] == "MBF-IMPORT-LOAD-SLOT-INACTIVE"]
    assert inactive_locations == ["LOADS L1 T2/P2", "LOADS L1 T3/P3"]


def test_from_rows_with_some_coordinates_read_the_missing_axes_as_zero():
    result = run(mbf("F10,KA,MM1,PP1,Y300,CL=L1", "T20,X1000", "F30,Z-50", "T40,X10"))
    assert [tuple(node["position"].values()) for node in result.model["nodes"]] == [
        (0.0, 300.0, 0.0), (1000.0, 300.0, 0.0), (0.0, 0.0, -50.0), (10.0, 0.0, -50.0)]
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "F30", "T40,X10")
    refused(run(data), "MBF-IMPORT-NODE-POSITION-MISSING", layout_line(data, "F30"))



# Codex VER-002 r5 (invented probe): the documented 10-field OPMISC layout gives the acceleration in the file's unit
# (m/s^2 in SI, in/s^2 in English); the export's 386.4 reading applies only to its 13-field layout.
GRAVITY_PROBE = """INVENTED GRAVITY PROBE
VERSION
15.10
OPTIONS
HGRA,{options}
OPTEMP
20,7000,1
OPMISC
33,20,1,1,1,2,2,2,{g}
MATERIAL
M1,7000,.3,1,1,CS,Invented
M1,20,200000,.000012
PIPE
P1,114.3,6.02
LOADS
L1,20,0,0
LAYOUT
F10,KA,MM1,PP1,CL=L1
T20,X1000
"""


def gravity_probe(g: str, *, si: bool = True) -> bytes:
    return GRAVITY_PROBE.format(options="SI" if si else "B311", g=g).replace("\n", "\r\n").encode("ascii")


def test_the_documented_layout_reads_386_4_in_an_si_file_as_m_per_s2_and_refuses_it():
    # By hand: 386.4 m/s^2 is 386.4 / 9.80665 = 39.4 g, not earth gravity. It imported before as 1 g with a weight of
    # -140.58 N/m; at the documented unit the weight would be 7000 x 386.4 x 2.0478e-3 = -5538.98 N/m.
    assert 386.4 / 9.80665 == pytest.approx(39.4016, rel=1e-5)
    data = gravity_probe("386.4")
    refused(run(data), "MBF-IMPORT-GRAVITY-UNSUPPORTED", layout_line(data, "33,20"))


@pytest.mark.parametrize("g, si", [
    ("9.80665", True), ("9.807", True), ("9.81", True), ("9.8066500", True),  # SI: m/s^2
    ("386.0886", False), ("386.09", False), ("386.1", False), ("386", False),  # English: 9.80665 / 0.0254 in/s^2
])
def test_the_documented_layout_accepts_standard_gravity_in_its_unit(g, si):
    # By hand: 9.80665 / 0.0254 = 386.08858267... in/s^2. Each printed value is that, or 9.80665 m/s^2, correctly
    # rounded to its own digits (|p - g0| <= half a unit in p's last digit), with at least three significant digits.
    data = gravity_probe(g, si=si)
    if not si:  # an English file: lengths in feet-inch, so the layout offset is 10 ft
        data = data.replace(b"T20,X1000", b"T20,X10")
    result = run(data)
    assert result.status == "imported", result.diagnostics
    assert any("earth gravity" in note for note in result.assumptions)


@pytest.mark.parametrize("g, si, why", [
    ("9.8", True, "two significant digits"),
    ("9.82", True, "|9.82 - 9.80665| = 0.01335 > 0.005"),
    ("386.4", False, "|386.4 - 386.0886| = 0.3114 > 0.05 (the export's own value, in the documented layout)"),
    ("32.2", False, "ft/s^2 is not the documented unit"),
    ("9.80665", False, "m/s^2 in an English file"),
    ("386.0886", True, "in/s^2 in an SI file"),
])
def test_the_documented_layout_refuses_other_accelerations(g, si, why):
    data = gravity_probe(g, si=si)
    if not si:
        data = data.replace(b"T20,X1000", b"T20,X10")
    refused(run(data), "MBF-IMPORT-GRAVITY-UNSUPPORTED", layout_line(data, "33,20"))


def test_the_export_layout_alone_reads_386_4():
    assert run(export_shaped()).status == "imported"  # 13 fields: 386.4 with direction (0, -1, 0)
    refused(run(export_shaped(opmisc="33,20,1,1,1,2,2,2,0,9.80665,0.00,-1.00,0.00")), "MBF-IMPORT-GRAVITY-UNSUPPORTED")
