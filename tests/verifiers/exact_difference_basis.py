"""Verifier of the completed admission policy for a declared operating-minus-sustained difference on the exact
route (DEL-05-02 REQ-012, AC-003), as `validation/references/exact_difference_basis_completed/README.md` states it.

Written from the policy text, separately from that package's generator; it shares no code with it and neither
imports the other. It reads case sides in the package's input format and returns a verdict. It forms no load and
solves nothing: the mechanical entries are not its concern.

A declared subtraction S -> O (S the subtrahend, "sustained"; O the minuend, "operating") is admitted only when
1. restraint: the same restraints and devices, and equal fixed-load inventories;
2. G-1: S's actions are contained in O's, by sameness and with multiplicity;
3. G-2: the same modulus basis for every member;
and the verdict names the first part that fails. Each part also reports its cause: what differs, in the
package's vocabulary (support_law:<support>/<dof> and fixed_load_inventory; the classes primitive,
pressure_region, pressure_elongation, generated_recipe, motion and member_state; <member>:<field>).

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


# Fields that hold quantities (the input format writes them as exact rationals). Only these are compared as
# numbers. A spring's stiffness is an exact quantity that may carry powers of pi, compared by its terms. Every other
# field (a material, a kind, a label, a direction, an identifier, a bits string) is compared as the text it is.
QUANTITY_FIELDS = frozenset({
    # moduli and their selection
    "E", "nu", "G", "selected_E", "selected_nu", "selection_temperature", "interpolation_fraction",
    # section
    "outside_diameter", "wall", "mill_tolerance", "corrosion_allowance",
    # generated-load inputs and mass basis
    "factor", "pressure", "shape_factor", "exposed_diameter", "section_mass_per_length", "contents_mass_per_length",
    # member states and fits
    "strain", "alpha", "interval", "installation_temperature", "operating_temperature", "datum", "constant",
    "points", "change", "reference_length",
})


def _digits(text: str) -> bool:
    return text != "" and all(character in "0123456789" for character in text)


def _signed_digits(text: str) -> bool:
    return _digits(text[1:] if text.startswith("-") else text)


def _pi_terms(text: str) -> frozenset:
    """A spring stiffness text as its non-zero terms {(power of pi, rational coefficient)}.

    The whole text must be terms joined by " + ". A term is a rational coefficient (an optional "-", ASCII digits,
    and optionally "/" and ASCII digits that are not zero), optionally followed by "*pi", optionally followed by
    "^" and an integer (an optional "-" and ASCII digits). No "*pi" is the power 0 and "*pi" alone the power 1;
    terms of one power add. Any other text raises ValueError: it is not read as a value and so equals nothing."""
    terms = {}
    for part in text.split(" + "):
        coefficient, star_pi, exponent = part.partition("*pi")
        numerator, slash, denominator = coefficient.partition("/")
        if not _signed_digits(numerator) or (slash and (not _digits(denominator) or int(denominator) == 0)):
            raise ValueError(f"not a stiffness text: {text!r}; coefficient {coefficient!r}")
        if not star_pi:
            power = 0
        elif exponent == "":
            power = 1
        elif exponent.startswith("^") and _signed_digits(exponent[1:]):
            power = int(exponent[1:])
        else:
            raise ValueError(f"not a stiffness text: {text!r}; after '*pi' comes {exponent!r}")
        terms[power] = terms.get(power, Fraction(0)) + Fraction(int(numerator), int(denominator) if slash else 1)
    return frozenset((k, v) for k, v in terms.items() if v)


def _canon(value, field=None):
    """A value made comparable: a quantity field's scalars as rationals in lowest terms, any other scalar as its
    text, mappings and lists recursively (a list's items take the list's field)."""
    if isinstance(value, dict):
        return frozenset((k, _canon(v, k)) for k, v in value.items())
    if isinstance(value, list):
        return tuple(_canon(v, field) for v in value)
    if value is None or isinstance(value, bool):
        return value
    if field == "stiffness":
        return ("pi", _pi_terms(str(value)))
    if field in QUANTITY_FIELDS:
        return ("q", Fraction(str(value)))
    return ("s", str(value))


def _is_zero(text) -> bool:
    return Fraction(str(text)) == 0


# ---- restraint ------------------------------------------------------------------------------------------

FREE_LAW = ("free",)


def restraint_law(record):
    """The resolved law of one support DOF: free by name (a DOF a side does not list is free as well); rigid, a
    spring or a nonlinear support with its identity, participation and parameters, every resolved field."""
    law = record["law"]
    if law == "free":
        return (law,)
    if law in ("rigid", "spring", "nonlinear"):
        return (law, record["identity"], record["participation"], _canon(record["parameters"]))
    raise ValueError(f"unknown support law {law!r}")


def restraint_cause(sustained, operating) -> list:
    laws_s = {(r["support"], r["dof"]): restraint_law(r) for r in sustained["supports"]}
    laws_o = {(r["support"], r["dof"]): restraint_law(r) for r in operating["supports"]}
    free = FREE_LAW  # a DOF a side does not list
    cause = [f"support_law:{support}/{dof}" for support, dof in sorted(set(laws_s) | set(laws_o))
             if laws_s.get((support, dof), free) != laws_o.get((support, dof), free)]
    inventory = lambda side: sorted((f["kind"], f["id"], f["value_bits"]) for f in side["fixed_loads"])
    if inventory(sustained) != inventory(operating):
        cause.append("fixed_load_inventory")
    return cause


def same_restraint(sustained, operating) -> bool:
    return not restraint_cause(sustained, operating)


# ---- G-1 ------------------------------------------------------------------------------------------------

def primitive_key(record, factor_bits):
    return ("primitive", record["category"], record["target"], record["direction"], record["dimension"],
            record["value_bits"], record["unit"], factor_bits)


def region_key(region):
    return ("pressure_region", tuple(sorted(region["pipe_members"])), tuple(sorted(region["connector_members"])),
            tuple(sorted((t["node"], t["closure_transfer"]) for t in region["terminals"])), region["pressure_bits"])


def generated_key(record):
    start, end = record.get("extent") or ("0", "1")
    return ("generated_recipe", record["kind"], record["target"], (Fraction(start), Fraction(end)),
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
    return ("member_state", record["member"], record["kind"], _canon(record["operands"]), _canon(record.get("fit") or NO_FIT))


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
        found[("pressure_elongation",)] += 1
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


def containment_cause(sustained, operating) -> list:
    """The classes of the subtrahend's actions that the minuend lacks, or holds too few times."""
    held, needed = actions(operating), actions(sustained)
    return sorted({key[0] for key, count in needed.items() if held[key] < count})


# ---- G-2 ------------------------------------------------------------------------------------------------

# The two readings of the stiffness part, defined here once:
#   "S", as specified: the resolved moduli, material, constitutive basis and section, and also how the basis was
#        selected (selection kind, case basis label, selection record);
#   "R": the resolved quantities alone, however they were selected.
RESOLVED_FIELDS = frozenset({"E", "nu", "G", "material", "constitutive_basis", "section"})
SELECTION_FIELDS = frozenset({"selection_kind", "case_basis_label", "selection_record"})
READINGS = {"S": RESOLVED_FIELDS | SELECTION_FIELDS, "R": RESOLVED_FIELDS}


def basis_cause(sustained, operating, reading="S") -> list:
    compared = READINGS[reading]
    if sorted(sustained["basis"]) != sorted(operating["basis"]):
        return ["members"]
    return [f"{member}:{f}" for member in sorted(sustained["basis"]) for f in sorted(compared)
            if _canon(sustained["basis"][member].get(f), f) != _canon(operating["basis"][member].get(f), f)]


def same_basis(sustained, operating, reading="S") -> bool:
    return not basis_cause(sustained, operating, reading)


# ---- the verdict ----------------------------------------------------------------------------------------

def admission(sustained, operating, reading="S") -> dict:
    causes = {"restraint": restraint_cause(sustained, operating),
              "g1": containment_cause(sustained, operating),
              "g2": basis_cause(sustained, operating, reading)}
    gates = {"restraint": same_restraint(sustained, operating),
             "g1": contained(actions(sustained), actions(operating)),
             "g2": same_basis(sustained, operating, reading)}
    assert all(gates[part] == (not causes[part]) for part in gates)
    for part, verdict in (("restraint", "refused_restraint"), ("g1", "refused_g1"), ("g2", "refused_g2")):
        if not gates[part]:
            return {"verdict": verdict, "gates": gates, "causes": causes,
                    "reason": f"the {part} part fails: " + ", ".join(causes[part])}
    return {"verdict": "admitted", "gates": gates, "causes": causes, "reason": "every part holds"}
