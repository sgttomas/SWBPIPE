"""The reference system's pending-family ledger: the specification's §10 table (DEL-09-01
`Work/reference-system.md`) equals the fixture generator's ledger and `authoring.json`'s pending flags.

Runs in the repository checks, which have the execution/ records; the product test
`tests/test_reference_system_fixture.py` checks the generator against `authoring.json`.
"""
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "execution" / "PKG-09" / "DEL-09-01" / "Work" / "reference-system.md"
FIXTURE = ROOT / "fixtures" / "reference_system"
HEADER = ["Family or analysis", "State", "Row / lane", "Expected until then"]


def spec_ledger():
    section = SPEC.read_text(encoding="utf-8").split("## 10. Pending-family ledger", 1)[1].split("\n## ", 1)[0]
    lines = [line for line in section.splitlines() if line.startswith("|")]
    if [cell.strip() for cell in lines[0].strip("|").split("|")] != HEADER:
        raise AssertionError(f"unexpected §10 header: {lines[0]}")
    if not set(lines[1].replace("|", "").strip()) <= {"-", " "}:
        raise AssertionError(f"unexpected §10 separator: {lines[1]}")
    rows = []
    for line in lines[2:]:
        cells = [cell.strip() for cell in line.strip().strip("|").split(" | ")]
        if len(cells) != 4:
            raise AssertionError(f"§10 row without four cells: {line}")
        rows.append(tuple(cells))
    return rows


def generator_ledger():
    spec = importlib.util.spec_from_file_location("generate_reference_system", FIXTURE / "generate_reference_system.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return [tuple(row) for row in module.LEDGER]


class ReferenceSystemLedgerTests(unittest.TestCase):
    def test_spec_ledger_equals_the_generator_ledger(self):
        self.assertEqual(spec_ledger(), generator_ledger())

    def test_spec_ledger_equals_the_authoring_pending_flags(self):
        authoring = json.loads((FIXTURE / "authoring.json").read_text(encoding="utf-8"))
        authored = [(row["family"], row["state"], row["row_lane"], row["expected_until_then"])
                    for row in authoring["pending"]]
        self.assertEqual(spec_ledger(), authored)


if __name__ == "__main__":
    unittest.main()
