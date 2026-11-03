"""DEL-04-09 I2a D3 (§8.43): the product modal module stays unreachable from the retained route (MODAL-UNR).

lib.rs declares `mod modal;` once, privately, and no other product_physics source names the module or its
kernel crate, so modal's new callers of self_weight's pub(crate) helpers add no retained reach. The guard checks
the listed lexical forms; the full-file pins of lib.rs and self_weight.rs also stop on any other change.
Restating a pin requires a fresh source audit, not reliance on this guard alone.
"""
import os
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
SRC = "core/product_physics/src"


def snapshot(tmp_path):
    shutil.copytree(SOURCE / SRC, tmp_path / SRC)
    return tmp_path


def test_current_modal_module_is_unreachable():
    assert MODEL["modal_unreachable"](str(SOURCE)) is True


def test_a_source_without_modal_takes_no_premise(tmp_path):
    root = snapshot(tmp_path)
    lib = root / SRC / "lib.rs"
    lib.write_text(lib.read_text(encoding="utf-8").replace("\nmod modal;\n", "\n"), encoding="utf-8")
    (root / SRC / "modal.rs").unlink()
    assert MODEL["modal_unreachable"](str(root)) is False


@pytest.mark.parametrize("old,new", [
    ("\nmod modal;\n", "\npub mod modal;\n"),
    ("\nmod modal;\n", "\npub(crate) mod modal;\n"),
    ("\nmod modal;\n", "\nmod modal;\nmod modal;\n"),
])
def test_a_changed_declaration_stops(tmp_path, old, new):
    root = snapshot(tmp_path)
    lib = root / SRC / "lib.rs"
    text = lib.read_text(encoding="utf-8")
    assert text.count(old) == 1
    lib.write_text(text.replace(old, new), encoding="utf-8")
    with pytest.raises(ValueError, match="modal"):
        MODEL["modal_unreachable"](str(root))


@pytest.mark.parametrize("file,extra", [
    ("lib.rs", "\nfn reach() { let _ = crate::modal::mass::mass_bearing_members; }\n"),
    ("lib.rs", "\nuse crate::modal::prepare;\n"),
    ("self_weight.rs", "\nfn reach() { let _ = super::modal::mass::mass_bearing_members; }\n"),
    ("retained_memory.rs", "\nuse open_pipe_stress_modal_eigen::dd::Dd;\n"),
])
def test_another_source_naming_modal_stops(tmp_path, file, extra):
    root = snapshot(tmp_path)
    target = root / SRC / file
    target.write_text(target.read_text(encoding="utf-8") + extra, encoding="utf-8")
    with pytest.raises(ValueError, match="modal reach changed: " + file):
        MODEL["modal_unreachable"](str(root))


def test_the_domain_proof_records_the_premise(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    assert MODEL["inspect"](SOURCE)["modal_unreachable"] is True
