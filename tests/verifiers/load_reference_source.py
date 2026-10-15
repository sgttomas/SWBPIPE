"""The current retained-source load/reference-state pre-pass (pressure-source-1).

Test verifier (b7-plan R2): the product reader is Rust's (`core/reporting/result_export/src/load_reference_source.rs`), which
product Python reaches only through `openpipestress-runner read`. Tests only: no module outside `tests/` imports
this one, and it imports nothing from product code (`tests/test_verifier_independence.py`).

Mirror of ``core/reporting/result_export/src/load_reference_source.rs``: received records are checked directly
under the current pressure contract (``SourceContract.PRESSURE_V3.check_received``, then the pre-pass in its
``PRESSURE_JOINED`` form); no retired namespace projection, policy substitution or hash resealing exists. B7b
(A4/A5) retired load-reference-source-1 (``resolved_straight_load_state_source_v1``, policy
``LOAD-REFERENCE-SOURCE-1``) with the ``2.0.0/exact_straight_pressure_v2`` contract; it is not read here. A resolved
state remains numerically ineligible: this reader does not independently rederive it from a captured request.
"""
from __future__ import annotations

from collections.abc import Mapping
from typing import Any

from . import load_reference_evidence as lr


def validate_pressure_source_evidence(source: Mapping[str, Any], *, raw: bool) -> None:
    """The current source group's closed received-state checks; no projection or reseal."""
    from .physics_source import SourceContract
    SourceContract.PRESSURE_V3.check_received(source)
    lr._prepass(source, raw, lr.PRESSURE_JOINED)
