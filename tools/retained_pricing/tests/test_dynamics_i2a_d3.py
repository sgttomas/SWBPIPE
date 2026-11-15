"""DEL-04-09 I2a D3 (§8.43): source facts the I2a rules rely on.

- D-SW: self_weight's inspect_generated_self_weight is called only by modal's weight-basis check (crate-wide; the
  Rust test checks the modal sources' own call sites at compile time).
- lib.rs names the modal module only in its one private declaration and in the body of the one public entry,
  `modal_experimental_result_document` (DEL-04-09 M4), its last item, which forwards to the crate-internal seam; and
  no priced crate source calls that entry. The pricing call graph resolves calls by name, so the entry is reached
  from no retained root while no priced source names it as a call. The retained reach of modal is established on
  the pricing run's own graph; this is a supplementary lexical check.
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


MODAL_ENTRY = (
    "pub fn modal_experimental_result_document(request: &serde_json::Value) -> serde_json::Value {\n"
    "    modal::modal_selected_result_document(request)\n"
    "}\n"
)


def test_lib_declares_modal_once_privately_and_calls_it_only_from_its_entry():
    lib = (SRC / "lib.rs").read_text(encoding="utf-8")
    assert re.findall(r"(?m)^mod modal;$", lib) == ["mod modal;"]
    assert re.findall(r"\bmodal\s*::\s*\w+", lib) == ["modal::modal_selected_result_document"]
    assert lib.endswith(MODAL_ENTRY), "the modal entry is not lib.rs's last item in its reviewed form"


def test_no_priced_source_calls_the_modal_entry():
    crate_dirs = (PRICING / "base/crate_dirs.txt").read_text(encoding="utf-8").split()
    calls = []
    for crate in crate_dirs:
        for path in sorted((SOURCE / crate).rglob("*.rs")):
            for n, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
                if re.search(r"\bmodal_experimental_result_document\b", line) and not line.startswith("pub fn "):
                    calls.append(f"{path.relative_to(SOURCE).as_posix()}:{n}")
    assert calls == [], calls
