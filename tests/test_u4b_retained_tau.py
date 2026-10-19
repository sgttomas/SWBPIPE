"""T4-U4b S4 round 2 (DEL-04-07 Design u4b-u5b-implementation.md §4.1, §5.1): the independent retained reader recomputes
route L's τ with the invocation.

The preview base read at G7 is model-free; with the invocation the reader then runs the preview model checks on its
projection, so a τ row one ulp off (its publication and receipt resealed) is refused at G7 with this language's base
code. The inputs are the committed W1-L successors (the milestone successor and the failed-ordinary CX-F cases), as
T4-S7's public regeneration published them with their derived rows; each carries τ rows.
The Rust reader's twin of this check runs on the live successors (product_physics `s4_retained_tests`).
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
"""
import json
import math
from copy import deepcopy
from pathlib import Path

import pytest

from verifiers import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]
SHEAR_MAX = "element_local_transverse_shear_stress_maximum"


def _successors():
    out = []
    for mode in ("sparse_interactive", "dense_scrutiny"):
        fixture = json.loads((ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
        out.append((f"milestone-{mode}", fixture["source"], fixture["invocation"]))
    cxf = json.loads((ROOT / "fixtures/results/retained_precision_failed_ordinary_cxf.json").read_text())
    for case in cxf["cases"]:
        out.append((f"failed-ordinary-cxf-{case['solver_mode']}", case["successor"], {"request": case["request"], "solver_mode": case["solver_mode"]}))
    return out


def _rehash(source):
    body = source["retained_precision"]["body"]
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)


@pytest.mark.parametrize("label,source,invocation", _successors(), ids=lambda v: v if isinstance(v, str) else "")
def test_a_tampered_tau_is_refused_at_g7_with_the_invocation(label, source, invocation):
    at = next((i for i, row in enumerate(source["results"]) if row.get("kind") == SHEAR_MAX), None)
    assert at is not None, f"{label}: the committed successor carries its derived rows"
    doc = deepcopy(source)
    doc["results"][at]["value"] = math.nextafter(doc["results"][at]["value"], math.inf)
    _rehash(doc)
    with pytest.raises(rp.RetainedPrecisionError) as refused:
        rp.validate_retained_precision(doc, invocation)
    assert (refused.value.gate, refused.value.code) == ("G7", "SOURCE_U4B_EVIDENCE_INVALID"), label
    assert rp.validate_retained_precision(doc)["numerical_eligible"] is False
    assert rp.validate_retained_precision(deepcopy(source), invocation)["numerical_eligible"] is True
