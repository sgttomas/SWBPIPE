"""IN-WS (DRAFT): the inverse-norm gate's certificate workspace.

IN-WS prices one certificate run per factor positively in both modes' typed core; a snapshot without the gate adds
nothing; a changed body, a moved allocating construct or a partly present gate stops."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
OWNERS = runpy.run_path(str(PRICING / "base/in_owners_model.py"))
IC = OWNERS["IC"]
FC = OWNERS["FC"]


def copy(tmp_path, rel=None, old=None, new=None):
    for path in (IC, FC):
        target = tmp_path / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / path, target)
    if rel:
        text = (tmp_path / rel).read_text()
        assert text.count(old) >= 1
        (tmp_path / rel).write_text(text.replace(old, new, 1))
    return str(tmp_path)


def test_the_workspace_is_active_and_positive_at_the_caps():
    assert OWNERS["active"](str(SOURCE)) is True
    # F = N = 192 and H = F(F + 1)/2 = 18,528 at the D1 caps (n = 32).
    form = dict(OWNERS["workspace"](192, 18528))
    assert form == {
        "1": 4608 + (1536 + 9216 + 2048) + (6144 + 4096) + (8 * 18528 + 4608) + (2048 + 6144),
        "s(Vec)": 256 + 192,
        "s(CertificateBlockRecord)": 192,
        "s(CertificateAttempt)": 768,
        "s(CertificateFigure)": 8064,
        "s(CertificateFailure)": 192,
        "s(CertificateReasons)": 4,
        "s(CertificateReasonCount)": 12,
        "s(CertificateCandidate)": 1024,
    }
    assert dict(OWNERS["slot"]()) == {}


def test_a_snapshot_without_the_gate_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert OWNERS["active"](str(tmp_path)) is False


@pytest.mark.parametrize("rel, old, new, match", [
    (IC, "let mut work = vec![0.0; n];", "let mut work = vec![0.0; 2 * n];", "fn factor"),
    (IC, "    let mut a_col = vec![0.0; n];\n", "    let mut a_col = vec![0.0; n];\n    let _spare = vec![0.0; n];\n",
     "fn passes"),
    (IC, "        Self { matrix, slot }\n", "        let _held = vec![0u8; 8];\n        Self { matrix, slot }\n", "in production"),
])
def test_a_changed_owner_stops(tmp_path, rel, old, new, match):
    with pytest.raises(SystemExit, match=match):
        OWNERS["active"](copy(tmp_path, rel, old, new))


def test_a_partly_present_gate_stops(tmp_path):
    snapshot = copy(tmp_path, FC, "let operand = verified_inverse_operand(est, inverse)?;",
                    "let operand = verified_inverse_operand(est * 2.0, inverse)?;")
    with pytest.raises(SystemExit, match="call is not the reviewed one"):
        OWNERS["active"](snapshot)
