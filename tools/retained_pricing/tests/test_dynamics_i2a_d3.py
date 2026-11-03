"""DEL-04-09 I2a D3 (§8.43), MODAL-UNR method proposal: the product modal module is isolated, so the pricing snapshot
may omit exactly modal.rs and modal/.

The guard runs on the full unfiltered source and fails closed. These tests mutate realistic bypasses (declarations,
use/re-export/alias/glob, super/self/crate/raw paths, #[path] and include!, macros and exported items, inherent,
trait and blanket impls on non-modal types) and check that comments and strings do not count, that the exclusion
removes nothing else, that the excluded bytes are bound (MODAL_TREE_PIN), and that a failing guard stops the run
rather than excluding anything.
"""
import os
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
SRC = "core/product_physics/src"


def snapshot(tmp_path):
    shutil.copytree(SOURCE / SRC, tmp_path / SRC)
    return tmp_path


def edit(root, file, old=None, new=None, append=None):
    target = root / SRC / file
    text = target.read_text(encoding="utf-8")
    if old is not None:
        assert text.count(old) == 1, (file, old)
        text = text.replace(old, new)
    if append is not None:
        text += append
    target.write_text(text, encoding="utf-8")


def test_current_modal_module_is_isolated():
    assert MODEL["modal_unreachable"](str(SOURCE)) is True


def test_the_current_modal_bytes_match_the_pin():
    assert MODEL["modal_tree_digest"](str(SOURCE)) == MODEL["MODAL_TREE_PIN"]


def test_a_source_without_modal_takes_no_premise(tmp_path):
    root = snapshot(tmp_path)
    edit(root, "lib.rs", "\nmod modal;\n", "\n")
    (root / SRC / "modal.rs").unlink()
    shutil.rmtree(root / SRC / "modal")
    assert MODEL["modal_unreachable"](str(root)) is False


@pytest.mark.parametrize("new", [
    "\npub mod modal;\n", "\npub(crate) mod modal;\n", "\nmod modal;\nmod modal;\n",
    "\n#[path = \"modal.rs\"]\nmod modal;\n", "\n#[cfg(test)]\nmod modal;\n",
])
def test_a_changed_declaration_stops(tmp_path, new):
    root = snapshot(tmp_path)
    edit(root, "lib.rs", "\nmod modal;\n", new)
    with pytest.raises(ValueError, match="modal"):
        MODEL["modal_unreachable"](str(root))


@pytest.mark.parametrize("file,extra", [
    ("lib.rs", "\nuse modal::*;\n"),
    ("lib.rs", "\nuse self::modal as m;\nfn reach() -> fn(&m::request::ModalPreviewRequest) { |_| () }\n"),
    ("lib.rs", "\npub(crate) use modal::request::ModalPreviewRequest;\n"),
    ("lib.rs", "\nuse crate::{modal};\n"),
    ("lib.rs", "\nfn reach() { let f = crate :: modal :: mass::mass_bearing_members; let _ = f; }\n"),
    ("lib.rs", "\nfn reach() { let _ = r#modal::mass::mass_bearing_members; }\n"),
    ("self_weight.rs", "\nfn reach() { let _ = super::modal::mass::mass_bearing_members; }\n"),
    ("self_weight.rs", "\nuse crate::modal as alias;\n"),
    ("retained_memory.rs", "\nuse open_pipe_stress_modal_eigen::dd::Dd;\n"),
    ("lib.rs", "\n#[path = \"modal/mass.rs\"]\nmod other_mass;\n"),
    ("lib.rs", "\nconst X: &str = include_str!(\"modal/mass.rs\");\n"),
    ("operating_conditions.rs", "\nmod inner { mod modal {} }\n"),
])
def test_another_source_reaching_modal_stops(tmp_path, file, extra):
    root = snapshot(tmp_path)
    edit(root, file, append=extra)
    with pytest.raises(ValueError, match="modal"):
        MODEL["modal_unreachable"](str(root))


@pytest.mark.parametrize("extra", [
    "\nimpl crate::PreviewModel { pub(crate) fn hidden(&self) {} }\n",
    "\nimpl std::fmt::Display for crate::PreviewModel {\n    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }\n}\n",
    "\nimpl From<ModalMass> for crate::Diagnostic {\n    fn from(_: ModalMass) -> Self { unimplemented!() }\n}\n",
    "\ntrait Hook { fn hook(&self) {} }\nimpl<T> Hook for T {}\n",
    "\n#[macro_export]\nmacro_rules! hidden { () => {} }\n",
    "\n#[no_mangle]\npub extern \"C\" fn hidden() {}\n",
    "\n#[used]\nstatic HOOK: fn() = || {};\n",
])
def test_a_modal_route_onto_priced_types_or_globals_stops(tmp_path, extra):
    root = snapshot(tmp_path)
    edit(root, "modal/mass.rs", append=extra)
    with pytest.raises(ValueError, match="modal"):
        MODEL["modal_unreachable"](str(root))


def test_comments_and_strings_do_not_count(tmp_path):
    root = snapshot(tmp_path)
    edit(root, "lib.rs", append="\n// use crate::modal::mass; modal::x\n/* use modal::*; */\n")
    edit(root, "self_weight.rs", append="\nconst NOTE: &str = \"crate::modal::mass and use modal::*\";\n")
    assert MODEL["modal_unreachable"](str(root)) is True


def test_a_modal_impl_on_its_own_type_is_isolated(tmp_path):
    root = snapshot(tmp_path)
    edit(root, "modal/mass.rs", append="\nimpl std::fmt::Debug for ModalMass {\n    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }\n}\n")
    assert MODEL["modal_unreachable"](str(root)) is True


def price_module(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING))
    monkeypatch.chdir(PRICING)
    return runpy.run_path(str(PRICING / "price.py"), run_name="price_under_test")


def test_the_exclusion_removes_exactly_the_modal_module(tmp_path, monkeypatch):
    root = snapshot(tmp_path)
    before = {p.relative_to(root).as_posix() for p in (root / SRC).rglob("*") if p.is_file()}
    report = price_module(monkeypatch)["exclude_unreachable_modal"](str(root))
    after = {p.relative_to(root).as_posix() for p in (root / SRC).rglob("*") if p.is_file()}
    removed = before - after
    assert removed and all(r == SRC + "/modal.rs" or r.startswith(SRC + "/modal/") for r in removed)
    assert (root / SRC / "lib.rs").exists() and (root / SRC / "self_weight.rs").exists()
    assert report["modal_tree_sha256"] == MODEL["MODAL_TREE_PIN"]


def test_changed_modal_bytes_stop_the_exclusion(tmp_path, monkeypatch):
    root = snapshot(tmp_path)
    edit(root, "modal/mass.rs", append="\n// changed\n")
    with pytest.raises(ValueError, match="MODAL_TREE_PIN"):
        price_module(monkeypatch)["exclude_unreachable_modal"](str(root))
    assert (root / SRC / "modal.rs").exists()


def test_a_failing_guard_excludes_nothing(tmp_path, monkeypatch):
    root = snapshot(tmp_path)
    edit(root, "lib.rs", append="\nuse modal::*;\n")
    with pytest.raises(ValueError, match="modal"):
        price_module(monkeypatch)["exclude_unreachable_modal"](str(root))
    assert (root / SRC / "modal.rs").exists() and (root / SRC / "modal").is_dir()


def test_the_domain_proof_records_the_premise(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    assert MODEL["inspect"](SOURCE)["modal_unreachable"] is True
