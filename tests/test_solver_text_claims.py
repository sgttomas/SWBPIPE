"""The desktop's table of solver text claims covers the diagnostic code table (release exit criterion 18).

`apps/desktop/src/features/results/solverTextClaims.json` classifies every diagnostic code by whether its
text states a pass, a bound or an established state that rests on the ordinary solve's error bound, which
SWBPIPE does not verify; the desktop puts "Reported by the solver; not verified by SWBPIPE." beside such a
text. The desktop's own test holds the same property, but a change that only adds a diagnostic code does
not select the desktop's tests. This one runs wherever the code table changes, so the change that adds a
code is the one that fails until the code is classified.
"""
from __future__ import annotations

import json
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
CODE_TABLE = PROJECT / "fixtures" / "diagnostics" / "exact_route_codes.json"
CLAIMS = PROJECT / "apps" / "desktop" / "src" / "features" / "results" / "solverTextClaims.json"
CLASSES = ("claim", "quotes_claim", "no_claim")


def _codes() -> set[str]:
    return {entry["code"] for entry in json.loads(CODE_TABLE.read_text(encoding="utf-8"))["codes"]}


def _classified() -> list[str]:
    table = json.loads(CLAIMS.read_text(encoding="utf-8"))
    return [code for name in CLASSES for code in table[name]]


def test_every_diagnostic_code_is_classified():
    missing = sorted(_codes() - set(_classified()))
    assert not missing, (
        f"{len(missing)} diagnostic code(s) of {CODE_TABLE.relative_to(PROJECT)} are not classified in "
        f"{CLAIMS.relative_to(PROJECT)}: {missing}. Read each code's messages and add it to 'claim' (every "
        "message states a pass, a bound or an established state resting on the ordinary solve's error bound), "
        "'quotes_claim' (a message may quote one) or 'no_claim'."
    )


def test_no_code_is_classified_twice_or_without_a_table_entry():
    classified = _classified()
    twice = sorted({code for code in classified if classified.count(code) > 1})
    assert not twice, f"classified in more than one class: {twice}"
    stale = sorted(set(classified) - _codes())
    assert not stale, f"classified but absent from the code table: {stale}"
