"""DEL-04-11 `Work/hanger-sizing.md` §8.3 against the closed result schema's fixed lists: every refusal code
appears in the design, and every limitation and basis statement is quoted there verbatim.

A document check: it reads the declarative specification and runs on the documentation route. The product
test `tests/test_hanger_sizing_evidence.py::test_the_verifiers_fixed_lists_are_the_schemas` holds the
independent verifier's lists equal to these schema lists, so together the two keep the verifier's lists in
the design. No module outside tests/ may import the verifier (tests/test_verifier_independence.py).
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DESIGN = ROOT / "execution/PKG-04/DEL-04-11/Work/hanger-sizing.md"
SCHEMA = ROOT / "schemas/hanger_sizing_result.schema.json"


def test_the_schemas_fixed_lists_are_the_designs():
    defs = json.loads(SCHEMA.read_text(encoding="utf-8"))["$defs"]
    refusal_codes = defs["RefusalCode"]["enum"]
    statements = [item["const"] for name in ("Limitations", "BasisStatement") for item in defs[name]["prefixItems"]]
    assert refusal_codes and statements
    design = DESIGN.read_text(encoding="utf-8")
    assert all(code in design for code in refusal_codes)
    for text in statements:
        assert f'"{text}"' in design, text
