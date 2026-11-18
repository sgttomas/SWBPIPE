"""DEL-04-11 REQ-014 (variable spring preload), as the test-only verifiers re-derive it.

An independent restatement of the producer's ``spring_preload::dispositions`` (PP
``core/product_physics/src/spring_preload.rs``) from the captured model, the way the verifiers
restate constant-effort consumption. A support whose ``support_hanger_type`` (the explicit
``hanger.hanger_type`` trimmed when non-empty, else the trimmed ``family``) is
``variable_spring_hanger`` or ``spring_hanger`` applies its ``hanger.cold_load`` as a constant force
along the positive axis of its translational stiffness DOF in every solved case, unless (in this
order) it carries a ``nonlinear`` member, it is a skewed record (a non-snubber ``direction`` with two
or more nonzero components), it has no cold load, the cold load is not a finite positive force, no
stiffness DOF can be read (``stiffness.dof``, else ``hanger.stiffness.dof``, else its first
restraint), that DOF is rotational, or its node is not a model node.

The published fact's name (``COLD_LOAD_FACT``) and the applied row's kind (``APPLIED_LOAD``) are
stated here once for every verifier. Standard-library imports only.
"""
from __future__ import annotations

from collections.abc import Callable, Mapping
import math
from typing import Any

# The ``combination_derivation`` member beside ``consuming_constant_effort`` (PP
# ``spring_preload::COLD_LOAD_FACT``): present, and true, only where a spring applies its cold load.
COLD_LOAD_FACT = "applying_variable_spring_cold_load"
# The producer's per-case row of an applying spring (PP ``APPLIED_LOAD_KIND``).
APPLIED_LOAD = "variable_spring_hanger_applied_load"

_RUST_WHITESPACE = frozenset(map(chr, [*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B),
                                       0x2028, 0x2029, 0x202F, 0x205F, 0x3000]))
_TRANSLATION = {"UX": "UX", "Ux": "UX", "ux": "UX", "UY": "UY", "Uy": "UY", "uy": "UY", "UZ": "UZ", "Uz": "UZ", "uz": "UZ"}
_ROTATIONAL = {"RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz"}


def _rust_trim(value: str) -> str:
    start, end = 0, len(value)
    while start < end and value[start] in _RUST_WHITESPACE:
        start += 1
    while end > start and value[end - 1] in _RUST_WHITESPACE:
        end -= 1
    return value[start:end]


def _hanger_type(support: Mapping[str, Any]) -> str | None:
    hanger = support.get("hanger")
    explicit = hanger.get("hanger_type") if isinstance(hanger, Mapping) else None
    reading = _rust_trim(explicit) if isinstance(explicit, str) else ""
    if reading:
        return reading
    family = support.get("family")
    return _rust_trim(family) if isinstance(family, str) else None


def _skewed(support: Mapping[str, Any]) -> bool:
    if support.get("family") == "snubber":
        return False
    direction = support.get("direction")
    if not isinstance(direction, Mapping):
        return False
    components = [direction.get(k) for k in ("x", "y", "z")]
    if not all(isinstance(c, (int, float)) and not isinstance(c, bool) and math.isfinite(c) for c in components):
        return False
    return sum(1 for c in components if c != 0) >= 2


def _dof_token(value: Any) -> str | None:
    return value if isinstance(value, str) and (value in _TRANSLATION or value in _ROTATIONAL) else None


def entered_value(cold: Mapping[str, Any]) -> float | None:
    """A cold load read as its entered value (the preview verifiers' reading of ``constant_load``)."""
    value = cold.get("value")
    return float(value) if isinstance(value, (int, float)) and not isinstance(value, bool) else None


def applying(model: Mapping[str, Any], cold_load: Callable[[Mapping[str, Any]], float | None] = entered_value) -> list[dict[str, Any]]:
    """Every applying spring of ``model``, in document order: its id, node, normalized DOF and force."""
    nodes = {n.get("id") for n in model.get("nodes") or [] if isinstance(n, Mapping)}
    out = []
    for support in model.get("supports") or []:
        if not isinstance(support, Mapping) or _hanger_type(support) not in ("variable_spring_hanger", "spring_hanger"):
            continue
        if support.get("nonlinear") is not None or _skewed(support):
            continue
        hanger = support.get("hanger")
        cold = hanger.get("cold_load") if isinstance(hanger, Mapping) else None
        if not isinstance(cold, Mapping):
            continue
        force = cold_load(cold)
        if force is None or not (math.isfinite(force) and force > 0):
            continue
        stiffness = support.get("stiffness")
        if not isinstance(stiffness, Mapping):
            stiffness = hanger.get("stiffness") if isinstance(hanger.get("stiffness"), Mapping) else {}
        restraints = support.get("restraints") or []
        token = _dof_token(stiffness.get("dof")) or (_dof_token(restraints[0]) if restraints else None)
        if token is None or token not in _TRANSLATION:
            continue
        node = support.get("node")
        if not isinstance(node, str) or node not in nodes or not isinstance(support.get("id"), str):
            continue
        out.append({"id": support["id"], "node": node, "dof": _TRANSLATION[token], "force": force})
    return out


_FIXED_LOAD_CODES = ("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND",
                     "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED")


def fixed_load_gate_message(code: str, constant_effort: bool, springs: bool, existing: str) -> str:
    """The producer's fixed-load gate text (PP ``spring_preload::fixed_load_gate_message``): a model whose only
    fixed-load source is a constant-effort support keeps ``existing``; every other gate keeps its own text."""
    if not springs or code not in _FIXED_LOAD_CODES:
        return existing
    if constant_effort:
        source, acts, each = "a constant-effort support's force and a variable spring hanger's cold load", "act", "each"
    else:
        source, acts, each = "a variable spring hanger's cold load", "acts", "it"
    if code == "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND":
        return (f"Combination withheld: {source} {acts} in every case state; an absolute sum or SRSS of operands whose "
                f"fixed-load count is not exactly 0 would count {each} once per term with its sign discarded.")
    if code == "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED":
        return (f"Combination withheld: an operand's fixed-load count is undefined, so the number of times this "
                f"combination counts {source} cannot be decided.")
    return f"Combination withheld: {source} would be counted by the sum of factors instead of once."
