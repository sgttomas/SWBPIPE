"""DEL-04-09 I2a D3 (§8.43): source facts the I2a rules rely on.

- D-SW: self_weight's inspect_generated_self_weight is called only by modal's weight-basis check (crate-wide; the
  Rust test checks the modal sources' own call sites at compile time).
- No lib.rs function names the modal module beyond its one private declaration (the retained reach of modal is
  established on the pricing run's own graph; this is a supplementary lexical check).
- MODAL-FB's caller pins and MODAL-ID's sites resolve on the current source (the rules stop the run otherwise).
"""
import os
import re
from pathlib import Path

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
SRC = SOURCE / "core/product_physics/src"


def rust_sources():
    return sorted(p for p in SRC.rglob("*.rs"))


def test_only_modal_weight_basis_calls_the_inspection():
    calls = []
    for path in rust_sources():
        rel = path.relative_to(SRC).as_posix()
        if rel == "modal/tests.rs":
            continue
        for n, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if "inspect_generated_self_weight(" in line and "fn inspect_generated_self_weight(" not in line \
                    and not line.lstrip().startswith("//"):
                calls.append(f"{rel}:{n}")
    assert len(calls) == 1 and calls[0].startswith("modal/weight_basis.rs:"), calls


def test_lib_declares_modal_once_privately_and_calls_nothing_in_it():
    lib = (SRC / "lib.rs").read_text(encoding="utf-8")
    assert re.findall(r"(?m)^mod modal;$", lib) == ["mod modal;"]
    assert not re.search(r"\bmodal\s*::", lib)
