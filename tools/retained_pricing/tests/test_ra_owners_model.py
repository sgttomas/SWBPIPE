"""R-A D3 (retained-pricing-rules.md §8.13).

RA-HS, RA-SC: B1's hint vectors (the plan's, and the replay's) and evaluate's Shortcuts are priced positively in T07
(the plan's hints and the live evaluation's Shortcuts) and T25 (the replay's hints and Shortcuts); a snapshot without
them adds nothing, and a changed or partly present body stops."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
OWNERS = runpy.run_path(str(PRICING / "base/ra_owners_model.py"))
FN = OWNERS["FN"]
SR = OWNERS["SR"]


def copy(tmp_path, rel=None, old=None, new=None):
    for path in (FN, SR):
        target = tmp_path / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / path, target)
    if rel:
        text = (tmp_path / rel).read_text()
        assert text.count(old) >= 1
        (tmp_path / rel).write_text(text.replace(old, new, 1))
    return str(tmp_path)


def test_the_r_a_owners_are_active_and_positive_at_the_caps():
    assert OWNERS["active"](str(SOURCE)) is True
    # Fn = 42m + N + s + 6g = 1,760 and N = 192 at the D1 caps (n = m = g = s = 32).
    assert dict(OWNERS["hints"](1760)) == {"s(Option<u32>)": 1760}
    assert dict(OWNERS["shortcuts"](192, 1760)) == {"1": 192 + 192 + 8 * 192 + 8 * 256 + 1760}


def test_a_snapshot_without_r_a_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert OWNERS["active"](str(tmp_path)) is False


@pytest.mark.parametrize("rel, old, new, match", [
    (FN, "products = vec![0; n];", "products = vec![0; 2 * n];", "shortcuts span"),
    (FN, "Ok((0..count).map(rule).collect())", "Ok((0..2 * count).map(rule).collect())", "build_hints"),
    (SR, "        if index >= self.nodal_start || row < 12 {", "        if row < 12 {", "fn hint"),
])
def test_a_changed_owner_stops(tmp_path, rel, old, new, match):
    with pytest.raises(SystemExit, match=match):
        OWNERS["active"](copy(tmp_path, rel, old, new))


def test_a_partly_present_r_a_stops(tmp_path):
    snapshot = copy(tmp_path, SR, "fn hint(&self, index: usize)", "fn hint_rule(&self, index: usize)")
    with pytest.raises(SystemExit, match="partly present"):
        OWNERS["active"](snapshot)
