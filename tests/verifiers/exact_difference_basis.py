"""Verifier of the completed admission policy for a declared operating-minus-sustained difference on the exact
route (DEL-05-02 REQ-012, AC-003), as `validation/references/exact_difference_basis_completed/README.md` states it.

Written from the policy text, separately from that package's generator; it shares no code with it and neither
imports the other. It reads case sides in the package's input format and returns a verdict. It forms no load and
solves nothing: the mechanical entries are not its concern.

A declared subtraction S -> O (S the subtrahend, "sustained"; O the minuend, "operating") is admitted only when
1. restraint: the same restraints and devices, and equal fixed-load inventories;
2. G-1: S's actions are contained in O's, by sameness and with multiplicity;
3. G-2: the same modulus basis for every member;
and the verdict names the first part that fails.

The key functions are module attributes so that a test can replace one with a deliberately wrong version.
"""
from __future__ import annotations

import struct
from collections import Counter
from fractions import Fraction

UNIT_FACTOR = struct.pack(">d", 1.0).hex()
NO_FIT = {"kind": "none"}


def _float(hex_bits: str) -> float:
    return struct.unpack(">d", bytes.fromhex(hex_bits))[0]


def _canon(value):
    """A value made comparable: rationals in lowest terms, mappings and lists recursively, other text as is."""
    if isinstance(value, dict):
        return frozenset((k, _canon(v)) for k, v in value.items())
    if isinstance(value, list):
        return tuple(_canon(v) for v in value)
    if value is None or isinstance(value, bool):
        return value
    text = str(value)
    try:
        return ("q", Fraction(text))
    except (ValueError, ZeroDivisionError):
        return ("s", text)


def _is_zero(text) -> bool:
    return Fraction(str(text)) == 0


# ---- restraint ------------------------------------------------------------------------------------------

def restraint_law(record):
    """The resolved law of one support DOF: free and rigid by name; a spring or a nonlinear support with its
    identity, participation and parameters."""
    law = record["law"]
    if law in ("free", "rigid"):
        return (law,)
    if law in ("spring", "nonlinear"):
        return (law, record["identity"], record["participation"], _canon(record["parameters"]))
    raise ValueError(f"unknown support law {law!r}")


def same_restraint(sustained, operating):
    laws_s = {(r["support"], r["dof"]): restraint_law(r) for r in sustained["supports"]}
    laws_o = {(r["support"], r["dof"]): restraint_law(r) for r in operating["supports"]}
    free = restraint_law({"law": "free"})
    for dof in sorted(set(laws_s) | set(laws_o)):
        if laws_s.get(dof, free) != laws_o.get(dof, free):
            return False
    inventory = lambda side: sorted((f["kind"], f["id"], f["value_bits"]) for f in side["fixed_loads"])
    return inventory(sustained) == inventory(operating)


# ---- G-1 ------------------------------------------------------------------------------------------------

def primitive_key(record, factor_bits):
    return ("primitive", record["category"], record["target"], record["direction"], record["dimension"],
            record["value_bits"], record["unit"], factor_bits)


def region_key(region):
    return ("region", tuple(sorted(region["pipe_members"])), tuple(sorted(region["connector_members"])),
            tuple(sorted((t["node"], t["closure_transfer"]) for t in region["terminals"])), region["pressure_bits"])


def generated_key(record):
    start, end = record.get("extent") or ("0", "1")
    return ("generated", record["kind"], record["target"], (Fraction(start), Fraction(end)),
            _canon(record["inputs"]), _canon(record["mass_basis"]))


def motion_is_action(record) -> bool:
    """A motion that compares equal to zero, of either sign, authored or defaulted, is no increment."""
    return _float(record["value_bits"]) != 0.0


def motion_key(record):
    return ("motion", record["support"], record["dof"], record["field"], record["value_bits"])


def _thermal_part_is_zero(kind, operands) -> bool:
    if kind == "unchanged_reference":
        return True
    if kind == "explicit_interval_strain":
        return _is_zero(operands["strain"])
    if kind == "constant_alpha_interval":
        return _is_zero(operands["alpha"]) or _is_zero(operands["interval"])
    if kind == "free_length_state":
        return Fraction(operands["installation_temperature"]) == Fraction(operands["operating_temperature"])
    raise ValueError(f"unknown member state kind {kind!r}")


def _fit_is_zero(fit) -> bool:
    if fit["kind"] == "none":
        return True
    if fit["kind"] == "fit_strain":
        return _is_zero(fit["strain"])
    if fit["kind"] == "natural_length_change":
        return _is_zero(fit["change"])
    raise ValueError(f"unknown fit {fit['kind']!r}")


def state_is_action(record) -> bool:
    """Fail-closed: a state is omitted only when its source is established exactly zero."""
    fit = record.get("fit") or NO_FIT
    return not (_thermal_part_is_zero(record["kind"], record["operands"]) and _fit_is_zero(fit))


def state_key(record):
    return ("state", record["member"], record["kind"], _canon(record["operands"]), _canon(record.get("fit") or NO_FIT))


def actions(side) -> Counter:
    """Everything the case side applies, counted."""
    found = Counter()
    if side["ledger"] is None:
        for record in side["primitives"]:
            found[primitive_key(record, UNIT_FACTOR)] += 1
    else:
        by_id = {record["id"]: record for record in side["primitives"]}
        for use in side["ledger"]:
            found[primitive_key(by_id[use["primitive"]], use["factor_bits"])] += 1
    for region in side["pressure_regions"]:
        found[region_key(region)] += 1
    if side["pressure_elongation"] and len(side["pressure_regions"]) > 0:
        found[("elongation",)] += 1
    for record in side["generated"]:
        found[generated_key(record)] += 1
    for record in side["motions"]:
        if motion_is_action(record):
            found[motion_key(record)] += 1
    for record in side["member_states"]:
        if state_is_action(record):
            found[state_key(record)] += 1
    return found


def contained(sustained_actions: Counter, operating_actions: Counter) -> bool:
    return all(operating_actions[key] >= count for key, count in sustained_actions.items())


# ---- G-2 ------------------------------------------------------------------------------------------------

# The two readings of the stiffness part, defined here once:
#   "S", as specified: the resolved moduli, material, constitutive basis and section, and also how the basis was
#        selected (selection kind, case basis label, selection record);
#   "R": the resolved quantities alone, however they were selected.
RESOLVED_FIELDS = frozenset({"E", "nu", "G", "material", "constitutive_basis", "section"})
SELECTION_FIELDS = frozenset({"selection_kind", "case_basis_label", "selection_record"})
READINGS = {"S": RESOLVED_FIELDS | SELECTION_FIELDS, "R": RESOLVED_FIELDS}


def same_basis(sustained, operating, reading="S") -> bool:
    compared = READINGS[reading]
    if sorted(sustained["basis"]) != sorted(operating["basis"]):
        return False
    for member, record in sustained["basis"].items():
        other = operating["basis"][member]
        if any(_canon(record.get(f)) != _canon(other.get(f)) for f in sorted(compared)):
            return False
    return True


# ---- the verdict ----------------------------------------------------------------------------------------

def admission(sustained, operating, reading="S") -> dict:
    gates = {"restraint": same_restraint(sustained, operating),
             "g1": contained(actions(sustained), actions(operating)),
             "g2": same_basis(sustained, operating, reading)}
    for part, verdict in (("restraint", "refused_restraint"), ("g1", "refused_g1"), ("g2", "refused_g2")):
        if not gates[part]:
            return {"verdict": verdict, "gates": gates, "reason": f"the {part} part fails"}
    return {"verdict": "admitted", "gates": gates, "reason": "every part holds"}
