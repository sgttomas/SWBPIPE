"""U6-RP (T4-U6/U8; U4B-RP, T4-S5): the tool's route-E P_final premise is the committed census's rows per member.

Route E's chain charges P_final = 7n + R·m + 8g + 3 and R0 = 7n + R·m + g + 3 at the caps, with R the census's
`rows_per_member`: `lib/census_patch.py` writes R into the base chain's P_final sites (`producer_caps.py`,
`g4_caps.py`, `ordinary_caps.py`, `t25_g4.py`, `g3lib.py`), and `rules/sq2_decisions_E.py`'s U6-RP guard states the
reviewed R (51 + 21 + 30 since U4B-RP) for the census file it names. A change to the census or to the tool's premise alone fails here.
The gate's hand-written premise (`retained_memory.rs` `P_FINAL_EXACT` and route E's G-C evidence caps) is tied to the
same census file by product_physics's `retained_memory::witness_tests::u6_rp_route_e_gate_caps_are_the_priced_census`.
"""
import ast
import importlib.util
import json
from pathlib import Path
import re
import shutil

PRICING = Path(__file__).resolve().parents[1]
CENSUS = PRICING / "data/census_u4b_7c77bf8072.json"   # U4B-RP (T4-S5); D-5: census_d5_79bf8656dc.json
DECISIONS_E = PRICING / "rules/sq2_decisions_E.py"
PATCHER = PRICING / "lib/census_patch.py"
CHAIN_FILES = ("producer_caps.py", "g4_caps.py", "ordinary_caps.py", "t25_g4.py", "g3lib.py")
ASSIGNMENT = re.compile(r"\b(P_final|P|R0)\s*=\s*([^;,#\n]+)")


def number(node):
    """An integer expression of literals, as the guard writes its premise (51 + 21)."""
    if isinstance(node, ast.Constant) and type(node.value) is int:
        return node.value
    if isinstance(node, ast.BinOp) and isinstance(node.op, (ast.Add, ast.Sub, ast.Mult)):
        left, right = number(node.left), number(node.right)
        return left + right if isinstance(node.op, ast.Add) else left - right if isinstance(node.op, ast.Sub) else left * right
    raise AssertionError("the U6-RP premise is not an integer expression: " + ast.dump(node))


def reviewed_premise():
    """The U6-RP guard in route E's decisions: the census file it reads and the rows per member it requires."""
    text = DECISIONS_E.read_text()
    blocks = [node for node in ast.walk(ast.parse(text)) if isinstance(node, ast.If) and isinstance(node.test, ast.Name)
              and node.test.id == "U6_D5" and "rows_per_member" in ast.get_source_segment(text, node)]
    assert len(blocks) == 1
    block = blocks[0]
    files = [node.value for statement in block.body if isinstance(statement, ast.Assign)
             and ast.get_source_segment(text, statement.targets[0]) == 'CENSUS["json"]'
             for node in ast.walk(statement.value) if isinstance(node, ast.Constant) and isinstance(node.value, str)
             and node.value.endswith(".json")]
    guards = [node for node in ast.walk(block) if isinstance(node, ast.Compare) and isinstance(node.left, ast.Subscript)
              and isinstance(node.left.slice, ast.Constant) and node.left.slice.value == "rows_per_member"]
    assert len(guards) == 1 and len(guards[0].ops) == 1 and isinstance(guards[0].ops[0], ast.NotEq)
    return files, number(guards[0].comparators[0])


def per_member(expression, which="caps"):
    """The coefficient of m in a chain's P_final or R0 expression, evaluated as the chain evaluates it."""
    code = compile(expression, "<P_final>", "eval")
    at = lambda m: eval(code, {"__builtins__": {}}, {"n": 0, "m": m, "g": 0, "which": which})
    return at(1) - at(0)


def test_route_e_p_final_premise_is_the_census_rows_per_member(tmp_path):
    census = json.loads(CENSUS.read_text())
    rows = census["rows_per_member"]
    files, reviewed = reviewed_premise()
    assert files == [CENSUS.name], "the U6-RP guard reads the committed U4b census"
    assert reviewed == rows, f"the reviewed premise ({reviewed}) is the census's rows per member ({rows})"
    # Route E's chain: census_patch.apply on the base chain files, with sq2_rules.py's exact-text patch.
    for name in CHAIN_FILES:
        shutil.copyfile(PRICING / "base" / name, tmp_path / name)
    applied = []

    def patch(name, old, new):
        path = tmp_path / name
        text = path.read_text()
        assert text.count(old) == 1, f"{name}: the chain's anchor is not exactly once in the base file"
        path.write_text(text.replace(old, new))
        applied.append((name, old, new))

    spec = importlib.util.spec_from_file_location("census_patch", PATCHER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    assert module.apply(str(tmp_path), census, patch)["rows_per_member"] == rows
    premise = {}
    for name, old, new in applied:
        if "51 * m" not in old:
            continue
        assert new in (tmp_path / name).read_text()
        for symbol, expression in ASSIGNMENT.findall(new):
            premise.setdefault(name, {}).setdefault(symbol, []).append(per_member(expression))
    assert set(premise) == set(CHAIN_FILES), "every P_final site of the base chain takes the census's premise"
    for name, symbols in premise.items():
        assert symbols.get("P_final", symbols.get("P")) == [rows], name
        assert symbols.get("R0", [rows]) == [rows], name
    assert sum(1 for symbols in premise.values() if "R0" in symbols) == 3
    # At the caps (n = m = g = 32) the chain's P_final is the rules' 3,747 rows per result set (D-5: 2,787).
    caps = census["caps"]
    assert 7 * caps["n"] + rows * caps["m"] + 8 * caps["g"] + 3 == 3_747
