#!/usr/bin/env python3
"""Grammar, refusal, loss-entry and exact-conversion tests of the DEL-17-04 MBF importer (Design
`caepipe-mbf-import.md` §4 to §16, §18.1 items 1 and 2).

Every MBF here is invented by this test file for the importer's checks; none is CAEPIPE output, a vendor example or
engineering data. Each named refusal asserts its code, its line and that no model is produced; each loss entry
asserts that the model is produced and the entry is listed.
"""

from __future__ import annotations

import decimal
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import re
import struct
import subprocess
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_mbf import importer as importer_module  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from core.handoff.caepipe_mbf.si_writer import current_model_schema_version, sidecar_self_digest  # noqa: E402
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
def test_unknown_option_tokens_are_refused():
    # The vertical-axis option Z is read since tranche 2 (Z-vertical tests below).
    data = mbf(options="HGRA,SI,QQ")
    refused(run(data), "MBF-IMPORT-OPTION-UNSUPPORTED", layout_line(data, "HGRA"))


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


_EXPONENT_PROBE = r"""
import json, sys
sys.path.insert(0, sys.argv[1])
import test_caepipe_mbf_importer as t
out = []
for optemp in sys.argv[2:]:
    result = t.run(t.mbf(optemp=optemp))
    out.append([result.status, sorted({item["code"] for item in result.diagnostics if item["severity"] == "error"})])
material = t.run(t.mbf(material=("M1,0,0.3,1,1,CS,x,1e-9223372036854775808", "M1,20,200000,1.2e-05")))
zero = t.run(t.mbf(material=("M1,0,0.3,1,1,CS,x,0e-9223372036854775808", "M1,20,200000,1.2e-05")))
out.append([material.status, sorted({item["code"] for item in material.diagnostics if item["severity"] == "error"})])
out.append([zero.status, []])
print(json.dumps(out))
"""


def test_decimal_exponents_beyond_1e5_are_refused_without_computing_the_power():
    # A non-zero numeral with a decimal exponent beyond 10^5 (here at the i64 limits) is refused as unrepresentable
    # before any Fraction or 10^n is formed; computing 10^9223372036854775808 never finishes, so the import runs in a
    # subprocess and a timeout is a failure. A zero mantissa is an exact zero at any exponent. The Rust port refuses
    # the same numerals with the same code (rational.rs `parse_fraction`, a signed range test).
    optemps = ["1e-9223372036854775808", "1e9223372036854775807", "-1e-9223372036854775808", "1e-200000",
               "1e200000", "0e-9223372036854775808"]
    try:
        done = subprocess.run([sys.executable, "-c", _EXPONENT_PROBE, str(Path(__file__).resolve().parent), *optemps],
                              capture_output=True, text=True, timeout=10, cwd=ROOT)
    except subprocess.TimeoutExpired:
        pytest.fail("the importer did not finish within 10 s on a decimal exponent at the i64 limits")
    assert done.returncode == 0, done.stderr
    unrepresentable = ["refused", ["MBF-IMPORT-VALUE-UNREPRESENTABLE"]]
    assert json.loads(done.stdout) == [unrepresentable] * 5 + [
        ["imported", []],  # OPTEMP 0e-9223372036854775808 is 0
        ["refused", ["MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED"]],  # a non-zero value in a protected column
        ["imported", []],  # a zero there is no value
    ]
    assert importer_module.exact_decimal("1.5e-3") == Fraction(3, 2000)
    assert importer_module.exact_decimal("0.0e9223372036854775807") == 0
    # beyond 10^+-400 a non-zero value is refused unformed, as the Rust port's bounded `parse_fraction` does
    assert importer_module.exact_decimal("1e399") == 10 ** 399 and importer_module.exact_decimal("1e400") is None
    assert importer_module.exact_decimal("-1e-401") == -Fraction(1, 10 ** 401)
    assert importer_module.exact_decimal("1e-402") is None
    assert importer_module.exact_decimal("1e100000") is None and importer_module.exact_decimal("1.5e-100000") is None


_DIGIT_RUN_PROBE = r"""
import json, sys
sys.path.insert(0, sys.argv[1])
import test_caepipe_mbf_importer as t
cases = [dict(optemp="0." + "0" * 20_000_000 + "1e-200000"), dict(optemp="1" * 20_000_000),
         dict(optemp="1e-" + "1" * 20_000_000), dict(optemp="1" * 4301), dict(optemp="1" * 4300),
         dict(optemp="0." + "0" * 4299 + "1e-200000"),
         dict(material=("M1,0,0.3,1,1,CS,x," + "1" * 5000, "M1,20,200000,1.2e-05"))]
out = []
for options in cases:
    try:
        result = t.run(t.mbf(**options))
        out.append([result.status, sorted({item["code"] for item in result.diagnostics if item["severity"] == "error"})])
    except ValueError as error:
        out.append(["ValueError", str(error)])
print(json.dumps(out))
"""


def _digit_limit(count: int) -> list:
    return ["ValueError", f"Exceeds the limit (4300 digits) for integer string conversion: value has {count} digits; "
                          "use sys.set_int_max_str_digits() to increase the limit"]


def test_long_digit_runs_are_refused_before_any_power_of_ten_is_formed():
    # Each digit run of a numeral is checked against Python's default int() digit limit before it is converted and
    # before 10^(its length) is formed, in the Rust port's order (rational.rs `parse_fraction_with`); 10^20000000
    # is deliberately avoided. The outcome is int()'s ValueError, as Fraction(text) raises it and as the
    # Rust port returns it (ImportError::Python); within the limit, a value beyond binary64 is refused by name.
    # This is a bounded-completion guard, not a 10-second performance requirement. The unchanged probe takes
    # 2.80 s alone locally but exceeded 10 s in the full hosted xdist import suite (9,950 passing tests).
    # Allow scheduler contention while retaining the 20-million-digit inputs and every refusal assertion.
    try:
        done = subprocess.run([sys.executable, "-c", _DIGIT_RUN_PROBE, str(Path(__file__).resolve().parent)],
                              capture_output=True, text=True, timeout=30, cwd=ROOT)
    except subprocess.TimeoutExpired:
        pytest.fail("the importer did not finish within 30 s on a 20000000-digit numeral")
    assert done.returncode == 0, done.stderr
    unrepresentable = ["refused", ["MBF-IMPORT-VALUE-UNREPRESENTABLE"]]
    assert json.loads(done.stdout) == [
        _digit_limit(20_000_001),  # the fraction digits of the reviewer's input
        _digit_limit(20_000_000),  # a long integer part
        _digit_limit(20_000_000),  # a long exponent
        _digit_limit(4301),
        unrepresentable,  # 4300 digits: within the limit, beyond binary64
        unrepresentable,  # 4300 fraction digits, then the exponent beyond 10^5
        _digit_limit(5000),  # a protected material column's zero test reads the numeral the same way
    ]


_ENGLISH_FEET_PROBE = r"""
import json, sys
sys.path.insert(0, sys.argv[1])
import test_caepipe_mbf_importer as t
from core.handoff.caepipe_mbf.importer import parse_english_length
out = []
for feet in ("0." + "0" * 20_000_000 + "1", "1" * 20_000_000):
    for read in (lambda: parse_english_length(feet),
                 lambda: t.run(t.mbf("F10,KA,MM1,PP1,CL=L1", "T20,X" + feet, options="HGRA")).status):
        try:
            out.append(["value", str(read())])
        except ValueError as error:
            out.append(["ValueError", str(error)])
print(json.dumps(out))
"""


def test_english_decimal_feet_bound_their_digit_runs_before_any_power_of_ten():
    # English decimal feet are read by the bounded decimal path (the Rust port's `fraction_of`): each digit run is
    # checked against the int() digit limit before 10^(its length) is formed. Fraction(text) formed 10^20000000
    # first, far beyond the timeout; the outcome is int()'s ValueError, as the Rust port returns it.
    try:
        done = subprocess.run([sys.executable, "-c", _ENGLISH_FEET_PROBE, str(Path(__file__).resolve().parent)],
                              capture_output=True, text=True, timeout=10, cwd=ROOT)
    except subprocess.TimeoutExpired:
        pytest.fail("English decimal feet of 20000000 digits did not finish within 10 s")
    assert done.returncode == 0, done.stderr
    assert json.loads(done.stdout) == [_digit_limit(20_000_001)] * 2 + [_digit_limit(20_000_000)] * 2
    assert parse_english_length("7.25") == Fraction(87) and parse_english_length("1." + "0" * 4299 + "1") == 12 * (
        1 + Fraction(1, 10 ** 4300))


def test_unicode_digits_count_as_the_rust_port_counts_them():
    # The numeral helpers count a numeral's digits from its value, as the Rust port does
    # (`numerator.to_string().len()`), so a Unicode zero is a leading zero like an ASCII one. The import itself
    # refuses non-ASCII numerals before reading them, on both sides (MBF-IMPORT-CHARACTER-UNSUPPORTED).
    exact_decimal = importer_module.exact_decimal
    assert exact_decimal("٠1e399") == exact_decimal("٠٠1e399") == exact_decimal("01e399") == 10 ** 399
    assert exact_decimal("٠1e400") is None and exact_decimal("١e400") is None
    assert exact_decimal("١٢.5") == Fraction(25, 2) and exact_decimal("٠.٠e-9223372036854775808") == 0
    assert importer_module.fraction_of("٠1e1000") == 10 ** 1000  # fraction_of has no magnitude bound
    for value in (1, 9, 10, 99, 100, 10 ** 399 - 1, 10 ** 399, 10 ** 4000 + 7):
        assert importer_module._decimal_digits(value) == len(str(value))
    result = run(mbf(material=("M1,0,0.3,1,1,CS,x,٠1e399", "M1,20,200000,1.2e-05")))
    refused(result, "MBF-IMPORT-CHARACTER-UNSUPPORTED")


def test_the_gravitational_direction_is_refused_in_a_bound_import_too():
    data = mbf(extra=("OPMISC", "33,20,1,1,1,2,2,1,9.80665,1"))
    sidecar = {"mbf_sha256": hashlib.sha256(data).hexdigest(),
               **{name: {} for name in ("node_map", "element_map", "support_map", "load_case_map", "material_map",
                                        "section_map", "load_map")}}  # a legacy sidecar holds the seven maps
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
    model = {"schema_version": current_model_schema_version(), "document_kind": "openpipestress.product_preview.model",
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
    sidecar = written.sidecar(hashlib.sha256(data).hexdigest())
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
    ("L1,20,-5,0", "MBF-IMPORT-PRESSURE-UNSUPPORTED"),  # negative (external) pressure; positive is read (§9.4)
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
    result = run(mbf("F10,KA,MM1,PP1,CL=L1,LA=0", "T20,X3000,CFIXD(1,0,1),TAG=invented guide-like"))
    assert [(item["id"], item["restraints"], item.get("family"), item["label"]) for item in result.model["supports"]] == [
        ("support:10", ["UX", "UY", "UZ", "RX", "RY", "RZ"], "anchor", "anchor at 10"),
        ("support:20-restraint", ["UX", "UZ"], None, "invented guide-like")]
    # With FIXRY too, the restraint has fewer than six DOFs and no family, which the solve reads as a guide, and a
    # guide restrains no rotation (SUPPORT_INPUT_INVALID, VER-002 r8): refused by support id.
    partial = run(mbf("F10,KA,MM1,PP1,CL=L1,LA=0", "T20,X3000,CFIXD(1,0,1),FIXRY,TAG=invented guide-like"))
    assert refused(partial, "MBF-IMPORT-SUPPORT-UNSUPPORTED")["location"] == "support:20-restraint"
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,X10,CFIXD(1,1,1),FIXR(1,1,1)"))
    assert result.model["supports"][-1]["restraints"] == ["UX", "UY", "UZ", "RX", "RY", "RZ"]  # kept as written
    assert "family" not in result.model["supports"][-1]
    listed(run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CTAG=lonely")), "MBF-IMPORT-TAG-WITHOUT-SUPPORT", "omitted")


@pytest.mark.parametrize("comment, code", [
    ("FIXD(2,0,0)", "MBF-IMPORT-COMMENT-INVALID"),
    ("FIXD(1,0)", "MBF-IMPORT-COMMENT-INVALID"),
    ("F", "MBF-IMPORT-COMMENT-INVALID"),
    ("G,GGAP=5", "MBF-IMPORT-SUPPORT-UNSUPPORTED"),
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


@pytest.mark.parametrize("comment", ["BSIF(2.5,2)", "BSIF (2.5,2.0)"])
def test_a_bend_sif_pair_is_read_from_bsif(comment):
    # Format of .MBF (users_manual/import_mbf.htm), BSIF: a bend's SIFs, in-plane then out-of-plane.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", f"T20,KI,B250,X4000,CFF=1,{comment}", "T30,KA,Z3000"))
    assert result.status == "imported", result.diagnostics
    modifiers = result.model["components"][0]["modifiers"]
    assert modifiers["sif_in_plane_user_value"] == {"value": 2.5, "unit": "none"}
    assert modifiers["sif_out_of_plane_user_value"] == {"value": 2.0, "unit": "none"}
    assert modifiers["source_reference"].endswith("comments FF and BSIF")
    assert not [item for item in result.loss_report if "BSIF" in item["reason"]]


@pytest.mark.parametrize("rows, code, at", [
    (("F10,KA,MM1,PP1,CL=L1", "T20,X4000,CBSIF(2,2)"), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,BSIF=2", "T30,Z3000"), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,BSIF(2)", "T30,Z3000"), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,BSIF(2,2,2)", "T30,Z3000"), "MBF-IMPORT-COMMENT-INVALID",
     "T20"),
    (("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1,BSIF(2,0)", "T30,Z3000"), "MBF-IMPORT-VALUE-OUT-OF-RANGE",
     "T20"),
])
def test_bsif_refusals(rows, code, at):
    data = mbf(*rows)
    refused(run(data), code, layout_line(data, at))


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
               "support_map": {"SA": {"node": "10", "caepipe_kind": "anchor",
                                      "restrained": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "family": "anchor"},
                               "GONE": {"node": "99", "caepipe_kind": "restraint", "restrained": ["UY"],
                                        "family": None}},
               **{name: {} for name in ("element_map", "load_case_map", "material_map", "section_map", "load_map")}}
    plain = run(data, sidecar=sidecar)
    assert plain.source["binding"] == "unbound" and plain.model["nodes"][0]["id"] == "node:10"
    assert any(item["code"] == "MBF-IMPORT-SIDECAR-NOT-APPLIED" for item in plain.diagnostics)
    rebound = run(data, sidecar=sidecar, rebind_by_node_number=True)
    assert rebound.source["binding"] == "rebound_unconfirmed"
    assert [node["id"] for node in rebound.model["nodes"]] == ["A", "T1", "T2", "B"]
    assert {"A", "T1", "T2", "B", "SA"} <= set(rebound.id_map["rebound_unconfirmed"])
    assert any(item["code"] == "MBF-IMPORT-SIDECAR-ENTRY-ABSENT" and "GONE" in item["location"]
               for item in rebound.loss_report)
    conflict = {**sidecar, "node_map": {"A": "10", "B": "10"}}  # not one-to-one: refused by the sidecar parser
    assert refused(run(data, sidecar=conflict, rebind_by_node_number=True),
                   "MBF-IMPORT-SIDECAR-INVALID")["location"] == "node_map[B]"
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
    result = run(mbf(loads=("L1,20,-12.345,0",)))
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
    model = {"schema_version": current_model_schema_version(), "document_kind": "openpipestress.product_preview.model",
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
    sidecar = written.sidecar(hashlib.sha256(data).hexdigest())
    bound = run(data, sidecar=sidecar)
    assert bound.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 0.0, "z": 1.0}
    assert "MBF-IMPORT-ORIENTATION-DEFAULTED" not in {item["code"] for item in bound.diagnostics}
    assert run(data).model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 1.0, "z": 0.0}  # unbound: default
    # A sidecar orientation that is not a usable vector (here zero length) is refused by name with the element, and
    # nothing is substituted (lane manager, after VER-002 r6).
    sidecar["element_map"]["S"]["y_reference"] = {"x": 0.0, "y": 0.0, "z": 0.0}
    sidecar["sidecar_sha256"] = sidecar_self_digest(sidecar)  # a deliberate edit, past the self-digest (§15)
    zero = run(data, sidecar=sidecar)
    assert refused(zero, "MBF-IMPORT-ORIENTATION-UNUSABLE")["location"] == "S"
    assert "MBF-IMPORT-ORIENTATION-DEFAULTED" not in {item["code"] for item in zero.diagnostics}


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


# --- §9.4 pressure (tranche 2) --------------------------------------------------------------------------------------
CLOSURE = {"closure_transfer": "transfers_to_wall", "provenance": "invented importer test declaration"}


def regions_of(result, case: str = "case:mbf-sustained") -> list[tuple]:
    found = next(item for item in result.model["load_cases"] if item["id"] == case)["pressure_regions"]
    return [(region["id"], region["member_pipe_ids"], region["pressure"]["value"], region["pressure"]["unit"],
             [(terminal["node_ref"], terminal["closure_transfer"]) for terminal in region["terminals"]])
            for region in found]


def test_pressure_1_becomes_an_exact_region_in_bar_with_the_file_allowance():
    # 12.5 bar is kept in bar; the catalog reads it as 12.5 x 1e5 = 1.25e6 Pa exactly (1 bar = 1e5 Pa).
    result = run(mbf(*L_BEND, loads=("L1,120,12.5,0",)), pressure_closure=CLOSURE)
    assert result.status == "imported", result.diagnostics
    assert result.model["pressure_contract"] == {"version": "3.0.0", "mode": "exact_pressure_v3",
                                                 "shear_model": "timoshenko_annulus_energy_v1"}
    assert regions_of(result) == [("region:mbf-10-30", ["pipe:10-20A", "pipe:20A-20B", "pipe:20B-30"], 12.5, "bar",
                                   [("node:10", "transfers_to_wall"), ("node:30", "transfers_to_wall")])]
    region = regions_of(result)[0]
    assert Fraction(region[2]) * 10**5 == 1_250_000
    sustained, expansion = result.model["load_cases"]
    assert (sustained["primitive_loads"], expansion["pressure_regions"]) == ([], [])  # pressure only; T1 - 20 = 100
    assert [load[4] for load in loads_of(result)] == [100.0, 100.0, 100.0]
    region_record = sustained["pressure_regions"][0]
    assert region_record["pressure_basis"] == "internal_differential_zero_external_v1"
    assert "LOADS L1 line" in region_record["provenance"]
    assert all(terminal["provenance"] == "closure transfer declared at import: invented importer test declaration"
               for terminal in region_record["terminals"])
    assert all(pipe["section"]["corrosion_allowance"] == {"value": 0.0, "unit": "mm"}
               for pipe in result.model["pipe_segments"])
    assert result.id_map["region_map"] == {"region:mbf-10-30": ["10-20A", "20A-20B", "20B-30"]}
    assert result.summary["pressure_regions"] == 1
    assert any("closure transfer at each real end (free end, tee, pressure or bore change) is the caller's "
               "declaration" in note for note in result.assumptions)


def test_english_pressure_stays_in_psi():
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X10", options="HGRA", optemp="70",
               material=("M1,0,0.3,1,1,CS,invented", "M1,70,29000000,6.5e-06"), pipe=("P1,6.625,0.28,0,0,0,0,0,0",),
               loads=("L1,70,150,0",))
    result = run(data, pressure_closure=CLOSURE)
    assert regions_of(result) == [("region:mbf-10-20", ["pipe:10-20"], 150.0, "psi",
                                   [("node:10", "transfers_to_wall"), ("node:20", "transfers_to_wall")])]
    assert result.model["project"]["units"]["pressure"] == "psi"
    assert result.model["pipe_segments"][0]["section"]["corrosion_allowance"] == {"value": 0.0, "unit": "in"}


def test_a_pressure_free_file_writes_no_contract_regions_or_allowance():
    result = run(mbf(*L_BEND))
    assert "pressure_contract" not in result.model and "region_map" not in result.id_map
    assert all("pressure_regions" not in case for case in result.model["load_cases"])
    assert all("corrosion_allowance" not in pipe["section"] for pipe in result.model["pipe_segments"])


@pytest.mark.parametrize("rows, pipe, loads, expected", [
    # A tee ends three regions at its node.
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X3000", "F20", "T40,KA,Z2000"), None, None,
     [("region:mbf-10-20", ["pipe:10-20"]), ("region:mbf-20-30", ["pipe:20-30"]), ("region:mbf-20-40", ["pipe:20-40"])]),
    # A straight-straight corner ends the region; collinear straights continue it.
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,Z2000"), None, None,
     [("region:mbf-10-20", ["pipe:10-20"]), ("region:mbf-20-30", ["pipe:20-30"])]),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000"), None, None,
     [("region:mbf-10-30", ["pipe:10-20", "pipe:20-30"])]),
    # A pressure change, or a bore change, ends it; the same bore from another OD and wall does not.
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000,CL=L2"), None, ("L1,20,10,0", "L2,20,12,0"),
     [("region:mbf-10-20", ["pipe:10-20"]), ("region:mbf-20-30", ["pipe:20-30"])]),
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000,PP2"), ("P1,168.3,7.11,0,0,0,0,0,0", "P2,168.3,8,0,0,0,0,0,0"),
     None, [("region:mbf-10-20", ["pipe:10-20"]), ("region:mbf-20-30", ["pipe:20-30"])]),
    # 168.3 - 2 x 7.11 = 154.08 = 170.3 - 2 x 8.11 exactly.
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000,PP2"), ("P1,168.3,7.11,0,0,0,0,0,0", "P2,170.3,8.11,0,0,0,0,0,0"),
     None, [("region:mbf-10-30", ["pipe:10-20", "pipe:20-30"])]),
    # An element with Pressure 1 = 0 belongs to no region.
    (("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000,CL=L2"), None, ("L1,20,10,0", "L2,20,0,0"),
     [("region:mbf-10-20", ["pipe:10-20"])]),
])
def test_regions_are_maximal_chains_of_equal_pressure_and_bore(rows, pipe, loads, expected):
    result = run(mbf(*rows, pipe=pipe, loads=loads or ("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert result.status == "imported", result.diagnostics
    assert [(region[0], region[1]) for region in regions_of(result)] == expected


def test_the_closure_transfer_is_declared_never_inferred():
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000", "T30,KA,X2000", loads=("L1,20,10,0",))
    refused(run(data), "MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED", layout_line(data, "LOADS"))
    for declaration in ({"closure_transfer": "capped"}, {"closure_transfer": "transfers_to_wall", "provenance": " "},
                        {**CLOSURE, "nodes": {"30": "open"}}, {**CLOSURE, "extra": 1}, "transfers_to_wall"):
        refused(run(data, pressure_closure=declaration), "MBF-IMPORT-PRESSURE-CLOSURE-INVALID")
    overridden = run(data, pressure_closure={**CLOSURE, "nodes": {"30": "separately_supported_or_compensated"}})
    assert regions_of(overridden)[0][4] == [("node:10", "transfers_to_wall"),
                                            ("node:30", "separately_supported_or_compensated")]
    for node in ("99", "20"):  # unknown, or internal to the region: not a terminal
        refused(run(data, pressure_closure={**CLOSURE, "nodes": {node: "transfers_to_wall"}}),
                "MBF-IMPORT-PRESSURE-CLOSURE-INVALID")
    pressure_free = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000")
    assert run(pressure_free).status == "imported"  # no declaration is needed without pressure


@pytest.mark.parametrize("extra, effects", [
    ((), "no OPPRES section"),
    (("OPPRES", ""), "Bourdon effect (on by the blank-line default), pressure correction for bends (on by the "
                     "blank-line default)"),
    (("OPPRES", "0,1,0,1.00,0"), "OPPRES gives Bourdon effect (on);"),
    (("OPPRES", "1,0,1"), "OPPRES gives pressure correction for bends (on);"),
    (("OPPRES", "0"), "Bourdon effect (unstated), pressure correction for bends (unstated)"),
    (("OPPRES", "0,0,0,1.00,0"), None),
])
def test_caepipe_pressure_effects_swbpipe_does_not_model_are_listed(extra, effects):
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", loads=("L1,20,10,0",), extra=extra),
                 pressure_closure=CLOSURE)
    entries = [item for item in result.loss_report if item["code"] == "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED"]
    if effects is None:
        assert entries == [] and result.status == "imported"
    else:
        assert [item["category"] for item in entries] == ["omitted"] and effects in entries[0]["reason"]
        assert "DEL-04-07 D-3" in entries[0]["reason"]
    without = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", extra=extra))  # no pressure: OPPRES only passes through
    assert "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED" not in {item["code"] for item in without.loss_report}


def test_pressure_refusals():
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", loads=("L1,20,10,0",), extra=("OPPRES", "0,2,0"))
    refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-FIELD-UNDOCUMENTED", layout_line(data, "0,2,0"))
    moved = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000,CD(0,5,0)", loads=("L1,20,10,0",))
    refused(run(moved, pressure_closure=CLOSURE), "MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED")
    assert run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000,CD(0,5,0)")).status == "imported"  # without pressure


# --- §5.1 Z vertical (tranche 2) ------------------------------------------------------------------------------------
def z_file(*rows: str, **options) -> bytes:
    return mbf(*rows, options="HGRA,SI,Z", **options)


def test_z_vertical_coordinates_are_rotated_exactly_and_take_the_z_vertical_local_axes():
    # (X, Y, Z) -> (X, Z, -Y): node 20 at file (1000, 2000, 3000) is SWBPIPE (1000, 3000, -2000); node 30 one
    # vertical 3000 above it is (1000, 6000, -2000).
    # Skew local y by hand, in the file's axes: x = (1, 2, 3)/sqrt 14; local z = Z - (Z.x) x is along (-3, -6, 5);
    # y = z x x = (-3, -6, 5) x (1, 2, 3) = (-28, 14, 0), unit (-2, 1, 0)/sqrt 5; rotated (-2, 0, -1)/sqrt 5.
    # Vertical element: local y is the file's global Y, rotated (0, 0, -1).
    result = run(z_file("F10,KA,MM1,PP1,CL=L1", "T20,X1000,Y2000,Z3000", "T30,KA,Z3000"))
    assert result.status == "imported", result.diagnostics
    assert [tuple(node["position"].values()) for node in result.model["nodes"]] == [
        (0.0, 0.0, 0.0), (1000.0, 3000.0, -2000.0), (1000.0, 6000.0, -2000.0)]
    skew, vertical = (tuple(pipe["y_reference"].values()) for pipe in result.model["pipe_segments"])
    assert skew == pytest.approx((-2 / math.sqrt(5), 0.0, -1 / math.sqrt(5)), abs=1e-15)
    assert vertical == (0.0, 0.0, -1.0)
    assert result.source["vertical_axis"] == "Z"
    defaulted = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-ORIENTATION-DEFAULTED"]
    assert all("global Z vertical" in item["message"] for item in defaulted) and len(defaulted) == 2
    assert any("(X, Y, Z) -> (X, Z, -Y)" in note for note in result.assumptions)


@pytest.mark.parametrize("end, expected", [
    ((3000.0, 0.0, 0.0), (0.0, 0.0, -1.0)),  # along x: local y = Z x X = file Y, rotated -z
    ((0.0, 0.0, 3000.0), (1.0, 0.0, 0.0)),  # along z (file -Y): y = (x_z, 0, -x_x) = (1, 0, 0)
    ((1000.0, 1000.0, 0.0), (0.0, 0.0, -1.0)),  # rising along x: still horizontal, perpendicular to x
    ((0.0, 3000.0, 0.0), (0.0, 0.0, -1.0)),  # vertical: file global Y
    ((0.0, -3000.0, 0.0), (0.0, 0.0, -1.0)),
])
def test_documented_local_y_of_a_straight_under_z_vertical(end, expected):
    # CAEPIPE-LOCAL-AXES, global Z vertical: local z in the plane of local x and global Z toward +Z, y = z x x; a
    # vertical element's y is global Y. In SWBPIPE's rotated axes y = (x_z, 0, -x_x) / |(x_x, x_z)|.
    assert importer_module.local_y_reference((0.0, 0.0, 0.0), end, z_vertical=True) == pytest.approx(expected,
                                                                                                    abs=1e-15)


def test_z_vertical_loads_displacements_and_restraint_axes_are_rotated():
    # Vectors (a, b, c) become (a, c, -b) for forces, moments, displacements and rotations alike (a proper
    # rotation turns axial vectors like polar ones); restraint flags only move: file Y -> z, file Z -> y.
    result = run(z_file("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CFOR(10,20,30),MOM(1,2,3),FLC=1,FIXD(0,1,0)",
                        "T30,X1000,CFIXDZ", "T40,X1000,KA,CD(4,5,6),ROT(0.1,0.2,0.3),FORY=7,FLC=1"))
    assert result.status == "imported", result.diagnostics
    assert loads_of(result) == [
        ("case:mbf-sustained", "concentrated_force", "node:20", "global_x", 10.0, "N"),
        ("case:mbf-sustained", "concentrated_force", "node:20", "global_y", 30.0, "N"),
        ("case:mbf-sustained", "concentrated_force", "node:20", "global_z", -20.0, "N"),
        ("case:mbf-sustained", "concentrated_moment", "node:20", "RX", 1.0, "N*m"),
        ("case:mbf-sustained", "concentrated_moment", "node:20", "RY", 3.0, "N*m"),
        ("case:mbf-sustained", "concentrated_moment", "node:20", "RZ", -2.0, "N*m"),
        ("case:mbf-sustained", "concentrated_force", "node:40", "global_z", -7.0, "N"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "UX", 4.0, "mm"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "UY", 6.0, "mm"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "UZ", -5.0, "mm"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "RX", 0.1, "deg"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "RY", 0.3, "deg"),
        ("case:mbf-expansion-t1", "imposed_displacement", "support:40", "RZ", -0.2, "deg"),
    ]
    restraints = {support["id"]: support["restraints"] for support in result.model["supports"]}
    assert (restraints["support:20-restraint"], restraints["support:30-restraint"]) == (["UZ"], ["UY"])


def test_a_bend_under_z_vertical():
    # The file's L runs along +X then +Y (horizontal); rotated, the legs are +x and -z. By hand: TI (4000, 0, 0),
    # t = 250 tan 45 deg = 250: near (3750, 0, 0), far (4000, 0, -250), node 30 (4000, 0, -3000);
    # y_reference = unit(TI - chord middle) = unit((4000, 0, 0) - (3875, 0, -125)) = (1, 0, 1)/sqrt 2.
    result = run(z_file("F10,KA,MM1,PP1,CL=L1", "T20,KI,B250,X4000,CFF=1", "T30,KA,Y3000"))
    nodes = {node["id"]: tuple(node["position"].values()) for node in result.model["nodes"]}
    assert nodes == {"node:10": (0.0, 0.0, 0.0), "node:20A": pytest.approx((3750.0, 0.0, 0.0), abs=1e-12),
                     "node:20B": pytest.approx((4000.0, 0.0, -250.0), abs=1e-12), "node:30": (4000.0, 0.0, -3000.0)}
    bend_pipe = result.model["pipe_segments"][1]
    assert tuple(bend_pipe["y_reference"].values()) == pytest.approx((math.sqrt(0.5), 0.0, math.sqrt(0.5)), abs=1e-16)


def test_z_vertical_gravity_and_weight():
    # The export layout's earth setting is read in a Z-vertical file only along the file's -Z; the weight per
    # length is the same as in a Y-vertical file and acts along SWBPIPE's -y.
    material = ("M1,7850,0.3,1,1,CS,invented", "M1,20,200000,1.2e-05")
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000")
    down = z_file(*rows, material=material, extra=("OPMISC", "33,20,1,1,1,2,2,2,0,386.4,0.00,0.00,-1.00"))
    z_weight, y_weight = (loads_of(run(data))[0] for data in (down, mbf(*rows, material=material)))
    assert z_weight == y_weight and z_weight[3] == "global_y" and z_weight[4] < 0
    sideways = z_file(*rows, material=material, extra=("OPMISC", "33,20,1,1,1,2,2,2,0,386.4,0.00,-1.00,0.00"))
    refused(run(sideways), "MBF-IMPORT-GRAVITY-UNSUPPORTED", layout_line(sideways, "33,20"))


def test_a_z_vertical_file_and_its_hand_rotated_y_vertical_twin_import_alike():
    # The twin's offsets, force, restraint and displacement are the Z file's, rotated by hand: (a, b, c) -> (a, c, -b).
    z = run(z_file("F10,KA,MM1,PP1,CL=L1", "T20,X1000,Y2000,Z3000,CFOR(10,20,30),FLC=1,FIXD(0,1,0)",
                   "T30,KI,B250,X4000,CFF=1", "T40,KA,Z3000,CD(4,5,6)"))
    y = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1000,Y3000,Z-2000,CFOR(10,30,-20),FLC=1,FIXD(0,0,1)",
                "T30,KI,B250,X4000,CFF=1", "T40,KA,Y3000,CD(4,6,-5)"))
    assert z.status == y.status == "imported", (z.diagnostics, y.diagnostics)

    def strip(model):
        model = json.loads(json.dumps(model))
        for pipe in model["pipe_segments"]:
            if pipe["id"] != "pipe:30A-30B":
                pipe.pop("y_reference")  # straights take each convention's own local axes
        return {key: value for key, value in model.items() if key in (
            "nodes", "pipe_segments", "supports", "components", "materials", "load_cases")}

    def unprovenanced(value):
        if isinstance(value, dict):
            return {key: unprovenanced(item) for key, item in value.items()
                    if key not in ("provenance", "bend_geometry_source_reference", "source_reference",
                                   "source_location")}  # these name each file's own hash
        return [unprovenanced(item) for item in value] if isinstance(value, list) else value

    assert unprovenanced(strip(z.model)) == unprovenanced(strip(y.model))  # bit-equal positions, bend and loads


# --- §9.4 regions by the exact route's own collinearity check (VER-002 r1, item 1) ------------------------------------
# Codex's reproduction: 16 rows each 1000 mm along X, with Y offsets k e-11 mm (k = 1..16). Each adjacent pair turns
# by about 1e-14 rad, below 64 eps = 1.42e-14, but the run bows: its whole-span residual is about 2e-14.
DRIFT = tuple(f"T{10 * (k + 1)},X1000,Y{k}e-11" for k in range(1, 16)) + ("T170,KA,X1000,Y16e-11",)
# Two straights 1000 mm long meeting at 1.5e-11 mm off the line: they turn by 2 x 1.5e-14 = 3e-14 rad (above 64 eps),
# but the middle node lies 1.5e-14 m / 2 m = 7.5e-15 of the span off the chord, within 64 eps.
ZIGZAG = ("T20,X1000,Y1.5e-11", "T30,KA,X1000,Y-1.5e-11")


def test_correctly_rounded_norm_and_the_ported_collinear_run():
    assert importer_module.correct_norm3(3.0, 4.0, 0.0) == 5.0
    assert importer_module.correct_norm3(1.0, 1.0, 1.0) == math.sqrt(3.0)  # sqrt is correctly rounded
    # Squares below the binary64 range: the exact root of 2 x (1e-200)^2, to 60 digits, then rounded once.
    with decimal.localcontext() as context:
        context.prec = 60
        expected = float(decimal.Decimal(1e-200) * decimal.Decimal(2).sqrt())
    assert importer_module.correct_norm3(1e-200, 1e-200, 0.0) == expected
    assert importer_module.REPRESENTATION_GUARD == 64 * 2.0 ** -52
    collinear = importer_module.runtime_collinear
    assert collinear([(0.0, 0.0, 0.0), (1.0, 1.5e-14, 0.0), (2.0, 0.0, 0.0)])  # residual 7.5e-15 <= 1.42e-14
    assert not collinear([(0.0, 0.0, 0.0), (1.0, 3e-14, 0.0), (2.0, 0.0, 0.0)])  # 1.5e-14 > 1.42e-14
    assert not collinear([(0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (1.0, 0.0, 0.0)])  # passes its end: backtracks


def test_a_run_the_exact_route_refuses_whole_is_refused_by_name_not_cut():
    # Manager's rule (VER-002 r2): a region is never cut inside a straight run; DRIFT has no corner.
    data = mbf("F10,KA,MM1,PP1,CL=L1", *DRIFT, loads=("L1,20,10,0",))
    error = refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR")
    assert error["location"] == "elements " + ", ".join(f"{10 * k}-{10 * (k + 1)}" for k in range(1, 17))
    assert error["line"] == layout_line(data, "T20,")


# Codex VER-002 r2: the first two straights alone fail (the middle node 3e-11 mm off a 2000 mm span: 3e-14 m / 2 m =
# 1.5e-14 > 64 eps), but the whole run passes (3e-14 m / 10 m = 3e-15). The route checks the whole run.
NONMONOTONIC = ("T20,X1000,Y3e-11", "T30,X1000,Y-3e-11", "T40,KA,X8000")


def test_a_run_the_exact_route_admits_whole_stays_one_region_though_a_prefix_fails():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", *NONMONOTONIC, loads=("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert [(region[0], region[1]) for region in regions_of(result)] == [
        ("region:mbf-10-40", ["pipe:10-20", "pipe:20-30", "pipe:30-40"])]
    collinear = importer_module.runtime_collinear
    assert not collinear([(0.0, 0.0, 0.0), (1.0, 3e-14, 0.0), (2.0, 0.0, 0.0)])
    assert collinear([(0.0, 0.0, 0.0), (1.0, 3e-14, 0.0), (2.0, 0.0, 0.0), (10.0, 0.0, 0.0)])


@pytest.mark.parametrize("offset, regions", [
    # A direction change of 2e-3 rad (> alpha_tan = 1e-3) is a corner: two regions.
    ("Y2", ["region:mbf-10-20", "region:mbf-20-30"]),
    # 5e-4 rad and 1e-4 rad are not corners, and the run fails the route's check whole: refused, not cut.
    ("Y0.5", None), ("Y0.1", None),
])
def test_a_corner_is_a_direction_change_above_the_routes_mitre_threshold(offset, regions):
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1000", f"T30,KA,X1000,{offset}", loads=("L1,20,10,0",))
    result = run(data, pressure_closure=CLOSURE)
    if regions is None:
        assert refused(result, "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR")["location"] == "elements 10-20, 20-30"
    else:
        assert [region[0] for region in regions_of(result)] == regions


def test_a_run_the_exact_route_admits_stays_one_region():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", *ZIGZAG, loads=("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert [(region[0], region[1]) for region in regions_of(result)] == [
        ("region:mbf-10-30", ["pipe:10-20", "pipe:20-30"])]
    # The adjacent-pair test this replaces split it: the two unit directions differ by about 3e-14 > 64 eps.
    a, b = (1000.0, 1.5e-11, 0.0), (1000.0, -1.5e-11, 0.0)
    d1, d2 = (tuple(value / math.hypot(*v) for value in v) for v in (a, b))
    assert abs(d1[0] * d2[1] - d1[1] * d2[0]) > importer_module.REPRESENTATION_GUARD


# Run 1 (10 -> 30A) is straight along X; the bend at 30 turns to +Z; run 2 (30B -> 200) drifts in X like DRIFT.
BEND_DRIFT = ("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KI,B250,X2000,CFF=1",
              *(f"T{10 * (k + 3)},Z1000,X{k}e-11" for k in range(1, 17)), "T200,KA,Z1000,X17e-11")


def test_straight_runs_between_bends_are_checked_each_whole():
    # Run 1 (10 -> 30A) passes; run 2 (30B -> 200) drifts like DRIFT and has no corner: refused, naming run 2 only.
    data = mbf(*BEND_DRIFT, loads=("L1,20,10,0",))
    error = refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR")
    assert error["location"] == "elements 30B-40, " + ", ".join(f"{10 * k}-{10 * (k + 1)}" for k in range(4, 20))
    straight = run(mbf(*BEND_DRIFT[:3], "T40,KA,Z1000", loads=("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert [region[1] for region in regions_of(straight)] == [["pipe:10-20", "pipe:20-30A", "pipe:30A-30B",
                                                              "pipe:30B-40"]]


# --- §9.4 corrosion allowance: field presence (VER-002 r1, item 2) ------------------------------------------------------
@pytest.mark.parametrize("record", ["P1,168.3,7.11", "P1,168.3,7.11,,0"])
def test_a_section_without_a_corrosion_allowance_is_refused_under_pressure(record):
    # The manual's PIPE record documents no default for an omitted field, so none is supplied.
    data = mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", pipe=(record,), loads=("L1,20,10,0",))
    error = refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-CORROSION-ALLOWANCE-MISSING",
                    layout_line(data, "P1,"))
    assert error["location"] == "PIPE P1 field 4"
    assert run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", pipe=(record,))).status == "imported"  # no pressure


def test_a_stated_allowance_is_carried_and_an_unused_section_needs_none():
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", pipe=("P1,168.3,7.11,0", "P2,100,5"),
                     loads=("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert result.status == "imported", result.diagnostics
    assert result.model["pipe_segments"][0]["section"]["corrosion_allowance"] == {"value": 0.0, "unit": "mm"}
    assert any("0 its PIPE record states in field 4" in note for note in result.assumptions)


# --- VER-002 r2 items 3 and 4 ---------------------------------------------------------------------------------------
def test_an_element_that_collapses_in_metres_is_refused():
    # 31.500000000000004 mm and that plus 3.552713678800501e-15 mm differ in binary64, but x 0.001 (the catalog's
    # mm factor) maps both to one value in m, where the solve reads them.
    start, end = 31.500000000000004, 31.500000000000004 + 3.552713678800501e-15
    assert start != end and start * 0.001 + 0.0 == end * 0.001 + 0.0
    for loads in (("L1,20,10,0",), ("L1,20,0,0",)):  # with or without pressure
        data = mbf("F10,KA,MM1,PP1,X31.500000000000004,CL=L1", "T20,KA,X3.552713678800501e-15", loads=loads)
        error = refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-GEOMETRY-ZERO-LENGTH", layout_line(data, "T20"))
        assert "metres" in error["message"]


def test_a_subnormal_norm_is_rounded_once_on_the_subnormal_lattice():
    # Codex's chord, in m: both components are subnormal. The exact root, to 80 digits, rounded once by the
    # correctly rounded decimal-to-binary conversion; rounding first to 53 bits and then to the lattice was one ulp high.
    x, y = 5.562684977829851e-306 * 0.001 + 0.0, 1.6578092611e-313 * 0.001 + 0.0
    assert 0 < x < 2.0 ** -1022
    with decimal.localcontext() as context:
        context.prec = 80
        expected = float((decimal.Decimal(x) ** 2 + decimal.Decimal(y) ** 2).sqrt())
    assert importer_module.correct_norm3(x, y, 0.0) == expected
    assert importer_module.correct_norm3(2.0 ** -1074, 0.0, 0.0) == 2.0 ** -1074
    assert importer_module.correct_norm3(3 * 2.0 ** -1074, 4 * 2.0 ** -1074, 0.0) == 5 * 2.0 ** -1074
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X5.562684977829851e-306,Y1.6578092611e-313",
                     loads=("L1,20,10,0",)), pressure_closure=CLOSURE)
    assert result.status == "imported", result.diagnostics


# --- VER-002 r3: the route's bore guard and traversal order -------------------------------------------------------------
# Codex's sections: 128.2 - 2 x 63.6 = 1 = 128.3 - 2 x 63.65 mm exactly in decimals, so one chain.
BORE_PIPES = ("P1,128.2,63.6,0", "P2,128.3,63.65,0")
BORE_ROWS = ("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KA,X1000,PP2")


def test_bores_equal_in_decimals_but_not_within_the_routes_guard_in_metres_are_refused_not_cut():
    radius = importer_module.runtime_bore_radius_m
    r1, r2 = radius(128.2, 63.6, 0.001), radius(128.3, 63.65, 0.001)
    # In m: 0.1282 x 0.5 - 0.0636 and 0.1283 x 0.5 - 0.06365, each rounded, differ by about 5.55e-14 relative.
    assert r1 != r2 and abs(r1 - r2) > importer_module.REPRESENTATION_GUARD * max(r1, r2)
    assert not importer_module.runtime_bores_equal([r1, r2]) and not importer_module.runtime_bores_equal([r2, r1])
    data = mbf(*BORE_ROWS, pipe=BORE_PIPES, loads=("L1,20,10,0",))
    error = refused(run(data, pressure_closure=CLOSURE), "MBF-IMPORT-PRESSURE-BORE-MISMATCH", layout_line(data, "T20"))
    assert error["location"] == "elements 10-20, 20-30"
    assert run(mbf(*BORE_ROWS, pipe=BORE_PIPES)).status == "imported"  # without pressure there is no region
    # The same bore from another OD and wall within the guard (168.3/7.11 and 170.3/8.11) stays one region.
    same = run(mbf(*BORE_ROWS, pipe=("P1,168.3,7.11,0", "P2,170.3,8.11,0"), loads=("L1,20,10,0",)),
               pressure_closure=CLOSURE)
    assert [region[1] for region in regions_of(same)] == [["pipe:10-20", "pipe:20-30"]]


# Codex's run: refused by the route traversed 10 -> 30, admitted 30 -> 10.
REVERSED = ("F10,KA,MM1,PP1,CL=L1", "T20,X2,Y3.0000000000924", "T30,KA,X1998,Y2996.9999999999076")


def test_a_region_the_route_admits_only_reversed_is_emitted_reversed():
    positions = {"10": (0.0, 0.0, 0.0), "20": (2.0, 3.0000000000924, 0.0),
                 "30": (2000.0, 3000.0, 0.0)}
    metres = {name: tuple(value * 0.001 + 0.0 for value in point) for name, point in positions.items()}
    assert not importer_module.runtime_collinear([metres["10"], metres["20"], metres["30"]])
    assert importer_module.runtime_collinear([metres["30"], metres["20"], metres["10"]])
    declaration = {**CLOSURE, "nodes": {"30": "separately_supported_or_compensated"}}
    result = run(mbf(*REVERSED, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",)), pressure_closure=declaration)
    assert result.status == "imported", result.diagnostics
    assert [node["position"]["y"] for node in result.model["nodes"]] == [0.0, 3.0000000000924, 3000.0]
    # Reversal changes only the order: the same members, and each terminal keeps its own node's closure transfer.
    assert regions_of(result) == [("region:mbf-30-10", ["pipe:20-30", "pipe:10-20"], 10.0, "bar",
                                   [("node:30", "separately_supported_or_compensated"),
                                    ("node:10", "transfers_to_wall")])]


def test_the_ported_arc_angle_matches_the_runtime_bit_for_bit():
    # The tangency and arc-window decisions (VER-002 r6) read frame_kernel's libm-free 2 atan2(y, x). Its own test
    # table freezes each row's correctly rounded binary64 bits, computed from a 90-digit arctangent; the port must
    # give every row's bits.
    source = (ROOT / "core" / "solver" / "frame_kernel" / "src" / "structural" / "arc_angle.rs").read_text()
    table = source[source.index("FROZEN"):]
    rows = re.findall(r"\(\s*(0x[0-9a-fA-F_]+)\s*,\s*(0x[0-9a-fA-F_]+)\s*,\s*(0x[0-9a-fA-F_]+)\s*\)", table)
    value = lambda text: struct.unpack("<d", struct.pack("<Q", int(text.replace("_", ""), 16)))[0]  # noqa: E731
    assert len(rows) > 40
    for y, x, expected in rows:
        assert importer_module.twice_atan2_nonnegative(value(y), value(x)) == value(expected), (y, x)
    assert importer_module.twice_atan2_nonnegative(0.0, 0.0) is None
    assert importer_module.twice_atan2_nonnegative(0.0, 2.0) == 0.0
    assert importer_module.twice_atan2_nonnegative(2.0, 0.0) == math.pi


# --- tranche 3: guides, limit stops, springs and flexible anchors (§12) ----------------------------------------------
def supports_of(result) -> list[tuple]:
    return [(item["id"], item["restraints"], item.get("family"),
             (item["stiffness"]["dof"], item["stiffness"]["value"]["value"], item["stiffness"]["value"]["unit"])
             if "stiffness" in item else None, item["label"]) for item in result.model["supports"]]


def test_rigid_axis_aligned_guides_and_limit_stops_map_onto_the_solves_rigid_families():
    # Hand-derived (invented layout). A guide restrains its element's local y and z (technical reference, Guide):
    # for an element along global X those are global Y and Z; along Y (vertical), X and Z. At a run's first node the
    # guide takes the following element. LS(0,0) is stopped both ways along DV, whose sign and length do not matter.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CG,GGAP=0,MU=0", "T30,Y2000,CG,TAG=invented riser guide",
                     "T40,Z1500,CLS(0,0),DV(0,-3,0)", "L40,CLS(0.0,-0),DV(0,0,0.5),MU=0",
                     "F50,X9000,CG", "T60,KA,Z1000"))
    assert result.status == "imported", result.diagnostics
    assert supports_of(result) == [
        ("support:10", list(importer_module.DOFS), "anchor", None, "anchor at 10"),
        ("support:20-guide", ["UY", "UZ"], "guide", None, "guide at 20"),
        ("support:30-guide", ["UX", "UZ"], "guide", None, "invented riser guide"),
        ("support:40-stop-y", ["UY"], "line_stop", None, "limit stop at 40"),
        ("support:40-stop-z", ["UZ"], "line_stop", None, "limit stop at 40"),
        ("support:50-guide", ["UX", "UY"], "guide", None, "guide at 50"),  # the following element, along Z
        ("support:60", list(importer_module.DOFS), "anchor", None, "anchor at 60"),
    ]
    assert result.id_map["support_map"]["40"] == ["support:40-stop-y", "support:40-stop-z"]
    assert any(note.startswith("A guide (G) restrains") for note in result.assumptions)
    assert any(note.startswith("A limit stop LS(0,0)") for note in result.assumptions)


def test_springs_keep_their_unit_one_per_dof():
    # SI: N/mm (Format of .MBF: TRAK, STIFF, K in lb/in or N/mm). A flexible anchor's zero TRAK component and
    # ROTK(0,0,0) leave those DOFs free (technical reference, Anchor); a flexible guide is a spring on each lateral
    # translation; a skewed restraint K along an axis-aligned DV a spring on that translation.
    result = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CG,STIFF=125.5", "T30,X1000,CLS(0,0),DV(-1,0,0),STIFF=7",
                     "T40,Y1000,CK=4e3,DV(0,0,9)", "T50,Y500,KA,CTRAK(10,0,0.25),ROTK(0,0,0)"))
    assert result.status == "imported", result.diagnostics
    assert supports_of(result)[1:] == [
        ("support:20-spring-uy", ["UY"], "spring", ("UY", 125.5, "N/mm"), "spring at 20"),
        ("support:20-spring-uz", ["UZ"], "spring", ("UZ", 125.5, "N/mm"), "spring at 20"),
        ("support:30-spring-ux", ["UX"], "spring", ("UX", 7.0, "N/mm"), "spring at 30"),
        ("support:40-spring-uz", ["UZ"], "spring", ("UZ", 4000.0, "N/mm"), "spring at 40"),
        ("support:50-spring-ux", ["UX"], "spring", ("UX", 10.0, "N/mm"), "spring at 50"),
        ("support:50-spring-uz", ["UZ"], "spring", ("UZ", 0.25, "N/mm"), "spring at 50"),
    ]
    assert "Spring stiffnesses keep their MBF unit, N/mm." in result.assumptions
    # English: lb/in, kept as lbf/in (the catalog reads it to N/m).
    english = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X10,KA,CTRAK(0,1500,0),ROTK(0,0,0)", options="HGRA",
                      material=("M1,0,0.3,1,1,CS,invented steel", "M1,70,29000000,6.5e-06"),
                      pipe=("P1,6.625,0.28,0,0,0,0,0,0",), loads=("L1,70,0,0",), optemp="70"))
    assert supports_of(english)[1:] == [("support:20-spring-uy", ["UY"], "spring", ("UY", 1500.0, "lbf/in"),
                                         "spring at 20")]


def test_support_axes_and_stiffnesses_move_with_the_z_vertical_rotation():
    # File (X, Y, Z) -> SWBPIPE (X, Z, -Y): TRAK(1,2,3) gives x 1, z 2, y 3 (magnitudes move with their axes);
    # DV(0,0,1) is the file's vertical, SWBPIPE's y; a guide on a pipe along the file's Z (vertical) restrains the
    # file's X and Y, SWBPIPE's x and z.
    result = run(z_file("F10,KA,MM1,PP1,CL=L1", "T20,Z3000,CG", "T30,X1000,CLS(0,0),DV(0,0,1)",
                        "T40,X1000,KA,CTRAK(1,2,3),ROTK(0,0,0)"))
    assert result.status == "imported", result.diagnostics
    assert supports_of(result)[1:] == [
        ("support:20-guide", ["UX", "UZ"], "guide", None, "guide at 20"),
        ("support:30-stop-y", ["UY"], "line_stop", None, "limit stop at 30"),
        ("support:40-spring-ux", ["UX"], "spring", ("UX", 1.0, "N/mm"), "spring at 40"),
        ("support:40-spring-uy", ["UY"], "spring", ("UY", 3.0, "N/mm"), "spring at 40"),
        ("support:40-spring-uz", ["UZ"], "spring", ("UZ", 2.0, "N/mm"), "spring at 40"),
    ]


@pytest.mark.parametrize("rows, code, at", [
    # guide data
    (("T20,X3000,CG,GGAP=5",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # a clearance: no member carries it
    (("T20,X3000,CG,MU=0.3",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # friction: nonlinear
    (("T20,X3000,CG,MU=-1",), "MBF-IMPORT-VALUE-OUT-OF-RANGE", "T20"),
    (("T20,X3000,CG,STIFF=0",), "MBF-IMPORT-VALUE-OUT-OF-RANGE", "T20"),
    (("T20,X3000,CG,STIFF=1e306",), "MBF-IMPORT-VALUE-UNREPRESENTABLE", "T20"),  # 1e309 N/m as the solve reads it
    (("T20,X3000,CG=1",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CG,G",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CSTIFF=5",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # data without its support
    (("T20,X3000,CGGAP=0",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CDV(1,0,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CK=5,DV(0,1,0),GGAP=0",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CG,LS(0,0),DV(1,0,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # two support types
    (("T20,X3000,Y1,CG",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # skewed element: lateral axes skewed
    (("T20,X3000,CG", "L20,CG"), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "L20"),  # a second guide
    # limit stop data
    (("T20,X3000,CLS",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CLS(0),DV(1,0,0)",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CLS(-1.0,1.5),DV(0,1,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # gaps
    (("T20,X3000,CLS(0,),DV(0,1,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # one-way
    (("T20,X3000,CLS(0,0)",), "MBF-IMPORT-TBD-17-04-010", "T20"),  # no documented default direction
    (("T20,X3000,CLS(0,0),DV(1,1,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # skewed direction
    (("T20,X3000,CLS(0,0),DV(0,0,0)",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CLS(0,0),AXIAL",), "MBF-IMPORT-TBD-17-04-010", "T20"),
    (("T20,X3000,CLS(0,0),DV(0,1,0),SHEARY",), "MBF-IMPORT-TBD-17-04-010", "T20"),
    (("T20,X3000,CLS(0,0),DV(0,1,0),SHEARZ",), "MBF-IMPORT-TBD-17-04-010", "T20"),
    (("T20,X3000,CLS(0,0),DV(0,1,0)", "L20,CLS(0,0),DV(0,2,0)"), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "L20"),
    (("T20,X3000,CG", "L20,CLS(0,0),DV(0,1,0)"), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "L20"),  # UY restrained twice
    # skewed restraint
    (("T20,X3000,CK=500",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # no direction
    (("T20,X3000,CK=0,DV(0,1,0)",), "MBF-IMPORT-VALUE-OUT-OF-RANGE", "T20"),
    (("T20,X3000,CK(1,2),DV(0,1,0)",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CK=500,KR=10,DV(1,0,0)",), "MBF-IMPORT-TBD-17-04-010", "T20"),  # the rotational unit is open
    (("T20,X3000,CK=5,DV(0,1,0)", "L20,CK=6,DV(0,-1,0)"), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "L20"),  # two springs
    (("T20,X3000,CK=5,DV(0,1,0),FIXDY",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # spring on a rigid DOF
    # flexible anchors
    (("T20,X3000,CTRAK(1,2,3)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # without KA
    (("T20,X3000,KA,CTRAK(1,0,0)",), "MBF-IMPORT-TBD-17-04-010", "T20"),  # without ROTK
    (("T20,X3000,KA,CROTK(0,0,0)",), "MBF-IMPORT-TBD-17-04-010", "T20"),  # without TRAK
    (("T20,X3000,KA,CTRAK(1,0,0),ROTK(0,5,0)",), "MBF-IMPORT-TBD-17-04-010", "T20"),  # the rotational unit
    (("T20,X3000,KA,CTRAK(-1,0,0),ROTK(0,0,0)",), "MBF-IMPORT-VALUE-OUT-OF-RANGE", "T20"),
    (("T20,X3000,KA,CTRAK(0,0,0),ROTK(0,0,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # restrains nothing
    (("T20,X3000,KA,CTRAK(1,0,0),ROTK(0,0,0),TRAK(2,0,0)",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,KA,CTRAK(1,0,0),ROTK(0,0,0),FIXDY",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # not alone
    (("T20,X3000,KA,CTRAK(1,0,0),ROTK(0,0,0),D(0,1,0)",), "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED", "T20"),
    (("T20,X3000,KA,CTRAKX=1,ROTK(0,0,0)",), "MBF-IMPORT-COMMENT-UNKNOWN", "T20"),  # only TRAK(kx,ky,kz)
    # hangers stay refused by name (§12): CAEPIPE designs VS and CS, and a hanger's loads act by case
    (("T20,X3000,CVS=2",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CCS",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CUS(2,600,1540)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CUS(1,0,2300)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CHTYP=16",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CCLD=1",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CK=5,DV(0,1,0),CNOD=30",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
])
def test_tranche_3_support_refusals(rows, code, at):
    data = mbf("F10,KA,MM1,PP1,CL=L1", *rows)
    refused(run(data), code, layout_line(data, at))


def test_stiff_rigid_reads_exactly_as_a_row_without_stiff():
    # STIFF=Rigid (observed format behaviour) is the rigid default of guides and limit stops (technical reference,
    # Guide; Limit Stop): the import is the one a row without STIFF gives, comment order included.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CG{}", "T30,Y2000,CTAG=,G{}", "T40,Z1500,CLS(0,0),DV(0,-3,0){}",
            "L40,CLS(0.000,0.000),DV(0.0000,0.0000,1.0000),MU=0.000{}", "T50,KA,X1000")
    plain_data = mbf(*(row.format("") for row in rows))
    rigid_data = mbf(*(row.format(",STIFF=Rigid") for row in rows))
    plain, rigid = run(plain_data), run(rigid_data)
    assert plain.status == rigid.status == "imported", rigid.diagnostics

    def masked(model, data):  # the model names its source file by hash; everything else must be identical
        digest = hashlib.sha256(data).hexdigest()
        return json.dumps(model, sort_keys=True).replace(digest, "<source>").replace(digest[:12], "<source>")

    assert masked(rigid.model, rigid_data) == masked(plain.model, plain_data)
    assert masked(rigid.id_map, rigid_data) == masked(plain.id_map, plain_data)
    assert [(item["code"], item["severity"]) for item in rigid.diagnostics] == \
        [(item["code"], item["severity"]) for item in plain.diagnostics]


@pytest.mark.parametrize("rows, code, at", [
    (("T20,X3000,CG,STIFF=rigid",), "MBF-IMPORT-NUMBER-INVALID", "T20"),  # only the spelling Rigid is read
    (("T20,X3000,CG,STIFF=R",), "MBF-IMPORT-NUMBER-INVALID", "T20"),
    (("T20,X3000,CG,STIFF(Rigid)",), "MBF-IMPORT-COMMENT-INVALID", "T20"),
    (("T20,X3000,CSTIFF=Rigid",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # data without its support
    (("T20,X3000,CK=Rigid,DV(0,1,0)",), "MBF-IMPORT-NUMBER-INVALID", "T20"),  # K is a value, not STIFF
    (("T20,X3000,CG,GGAP=5,STIFF=Rigid",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),  # the gap stays refused
    # None is a blank limit, free on that side (technical reference, Limit Stop): a one-way stop, refused by name
    (("T20,X3000,CLS(0.000,None),DV(0,1,0),STIFF=Rigid",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CLS(None,0),DV(0,1,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CLS(None,None),DV(0,1,0)",), "MBF-IMPORT-SUPPORT-UNSUPPORTED", "T20"),
    (("T20,X3000,CLS(0,NONE),DV(0,1,0)",), "MBF-IMPORT-NUMBER-INVALID", "T20"),  # only the spelling None is read
])
def test_the_rigid_and_blank_limit_spellings(rows, code, at):
    data = mbf("F10,KA,MM1,PP1,CL=L1", *rows)
    result = run(data)
    refused(result, code, layout_line(data, at))
    if rows[0].startswith("T20,X3000,CLS(") and "NONE" not in rows[0]:
        # a None limit is refused as the one-way stop it is, never as an unreadable number
        assert not any(item["code"] == "MBF-IMPORT-NUMBER-INVALID" for item in result.diagnostics)
        assert any("one-way" in item["message"] for item in result.diagnostics if item["severity"] == "error")


def test_a_guide_whose_element_is_not_determined_is_refused():
    # Its first node has no preceding element and two following ones.
    data = mbf("F10,MM1,PP1,CL=L1,G", "T20,KA,X3000", "F10", "T30,KA,Y1000")
    assert refused(run(data), "MBF-IMPORT-SUPPORT-UNSUPPORTED", layout_line(data, "F10,"))["location"] == "node 10 G"


@pytest.mark.parametrize("row, path", [
    ("T20,X3000,CK=1e20,DV(0,1,0),FOR(0,-6000,0),FLC=1", "supports[support:20-spring-uy].stiffness.value.value"),
    ("T20,X3000,CK=9007199254740992,DV(0,1,0),FOR(0,-6000,0),FLC=1",
     "supports[support:20-spring-uy].stiffness.value.value"),  # 2^53: the first integral float refused
    ("T20,X3000,CFOR(0,-1e20,0),FLC=1",
     "load_cases[case:mbf-sustained].primitive_loads[load:concentrated_force:node:20:global_y].magnitude.value"),
    ("T20,X100000000000000000,CFOR(0,-6000,0),FLC=1", "nodes[node:20].position.x"),
])
def test_every_emitted_number_lies_in_the_solves_checked_json_domain(row, path):
    # VER-002 r2 (tranche 3): the solve reads the document under the checked-JSON profile, which refuses an integral
    # float beyond 2^53 - 1 (canonical_json validate_checked_value, CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT). The import
    # refuses each such number by name at its JSON path, whatever field it is; nothing is changed or dropped.
    data = mbf("F10,KA,MM1,PP1,CL=L1", row)
    found = refused(run(data), "MBF-IMPORT-VALUE-UNREPRESENTABLE", layout_line(data, "T20"))
    assert found["location"] == path and "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT" in found["message"]
    edge = run(mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CK=9007199254740991,DV(0,1,0),FOR(0,-6000,0),FLC=1"))
    assert edge.status == "imported"  # 2^53 - 1 is in the domain
    assert edge.model["supports"][1]["stiffness"]["value"]["value"] == 9007199254740991.0
