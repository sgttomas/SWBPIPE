"""The canonical evidence view of the M3 judge (refutation round 5): the only data the comparison reads.

The validator (m3_view.py) builds these values from a strictly parsed reference line and a strictly parsed raw
runner record; anything it did not validate into a view does not exist for judging. Every value is immutable
and typed (checked on construction): strings, binary64 values, exact rationals and tuples, no dictionaries and
no raw JSON. This module imports no JSON reader.

Identities are unique by construction of the view: a row identity (case, kind, entity, location, component,
section basis), an extremum identity (case, pipe, section basis), a standing's case and a headline's basis each
occur at most once, or the record is refused before a view exists.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from fractions import Fraction

ROUTES = ("pf", "exact")
SECTION_BASES = ("nominal_section", "reduced_section")


def _num(x):
    return isinstance(x, float) and math.isfinite(x)


def _opt(x, t):
    return x is None or (_num(x) if t is float else isinstance(x, t))


def _tuple_of(xs, t):
    return isinstance(xs, tuple) and all(isinstance(x, t) for x in xs)


def _require(cond, what):
    if not cond:
        raise TypeError(f"view: {what}")


@dataclass(frozen=True, slots=True)
class Row:
    """One published result row: its identity, the unit the contract fixes for its kind and component, its
    binary64 value (None: published null, serde's non-finite) and its published source basis token."""
    case: str
    kind: str
    entity: str
    location: str | None
    component: str | None
    basis: str | None
    unit: str
    value: float | None
    source: str | None

    def __post_init__(self):
        _require(all(isinstance(x, str) for x in (self.case, self.kind, self.entity, self.unit)), "row strings")
        _require(_opt(self.location, str) and _opt(self.component, str) and _opt(self.source, str), "row optional strings")
        _require(self.basis is None or self.basis in SECTION_BASES, "row section basis")
        _require(self.value is None or _num(self.value), "row value")

    @property
    def identity(self):
        return (self.case, self.kind, self.entity, self.location, self.component, self.basis)

    @property
    def key(self):
        """The identity within its case: the reference's row key."""
        return (self.kind, self.entity, self.location, self.component, self.basis)


@dataclass(frozen=True, slots=True)
class Extremum:
    """A pipe's maximum-stress enclosure in one load case and section basis: lower <= upper whenever both are
    published (a null bound is kept as None: a null upper covers nothing)."""
    case: str
    pipe: str
    basis: str | None
    lower: float | None
    upper: float | None

    def __post_init__(self):
        _require(isinstance(self.case, str) and isinstance(self.pipe, str), "extremum strings")
        _require(self.basis is None or self.basis in SECTION_BASES, "extremum section basis")
        _require(_opt(self.lower, float) and _opt(self.upper, float), "extremum bounds")
        _require(self.lower is None or self.upper is None or self.lower <= self.upper, "extremum interval lo <= hi")


@dataclass(frozen=True, slots=True)
class Headline:
    """A summary headline of the maximum stress: basis None is `max_open_formula_stress`, otherwise
    `max_open_formula_stress_<basis>`; is_null when the headline itself is null."""
    basis: str | None
    is_null: bool
    value: float | None

    def __post_init__(self):
        _require(self.basis is None or self.basis in SECTION_BASES, "headline basis")
        _require(isinstance(self.is_null, bool) and _opt(self.value, float) and (not self.is_null or self.value is None), "headline value")


@dataclass(frozen=True, slots=True)
class Standing:
    """The K-D5 standing of one load case (a code of the closed set) and its message."""
    case: str
    code: str
    message: str

    def __post_init__(self):
        _require(all(isinstance(x, str) for x in (self.case, self.code, self.message)), "standing strings")


@dataclass(frozen=True, slots=True)
class RunView:
    """One run (model x entry x mode) as the judge sees it. Exactly one outcome: a runner error, a refusal (one
    code of the route's closed set, no rows) or one standing per load case of the model (with rows)."""
    route: str
    tag: str
    entry: str
    mode: str
    request_sha256: str
    runner_sha256: str
    product_head: str
    error: str | None
    refusal: str | None
    standings: tuple
    rows: tuple
    extrema: tuple
    headlines: tuple
    warnings: frozenset

    def __post_init__(self):
        _require(self.route in ROUTES, "run route")
        _require(all(isinstance(x, str) for x in (self.tag, self.entry, self.mode, self.request_sha256, self.runner_sha256, self.product_head)), "run strings")
        _require(_opt(self.error, str) and _opt(self.refusal, str), "run outcome strings")
        _require(_tuple_of(self.standings, Standing) and _tuple_of(self.rows, Row) and _tuple_of(self.extrema, Extremum)
                 and _tuple_of(self.headlines, Headline), "run tuples")
        _require(isinstance(self.warnings, frozenset) and all(isinstance(x, str) for x in self.warnings), "run warnings")
        outcomes = (self.error is not None) + (self.refusal is not None) + bool(self.standings)
        _require(outcomes == 1, "exactly one outcome")
        _require(not (self.error is not None or self.refusal is not None) or not (self.rows or self.extrema or self.headlines), "no rows without a standing")
        _require(len({r.identity for r in self.rows}) == len(self.rows), "unique row identities")
        _require(len({(e.case, e.pipe, e.basis) for e in self.extrema}) == len(self.extrema), "unique extremum identities")
        _require(len({s.case for s in self.standings}) == len(self.standings), "one standing per case")
        _require(len({h.basis for h in self.headlines}) == len(self.headlines), "unique headlines")

    @property
    def identity(self):
        return (self.tag, self.entry, self.mode)


@dataclass(frozen=True, slots=True)
class SectionProps:
    """Section properties of one basis, exact (m^2, m^3, m^4)."""
    A: Fraction
    Z: Fraction
    J: Fraction

    def __post_init__(self):
        _require(all(isinstance(x, Fraction) for x in (self.A, self.Z, self.J)), "section properties")


@dataclass(frozen=True, slots=True)
class TauInputs:
    """PF's tau_v,max inputs (DEL-04-07 u4b-u5b section 2.4): the model's binary64 E, G, OD, wall and mill
    tolerance; I, Z and the exact tau factor c."""
    E: float
    G: float
    od: float
    wall: float
    mill: float
    I: Fraction
    Z: Fraction
    tau_c: Fraction

    def __post_init__(self):
        _require(all(isinstance(x, float) for x in (self.E, self.G, self.od, self.wall, self.mill)), "tau binary64 inputs")
        _require(all(isinstance(x, Fraction) for x in (self.I, self.Z, self.tau_c)), "tau exact inputs")


@dataclass(frozen=True, slots=True)
class PipeSection:
    pipe: str
    ro: Fraction
    bases: tuple          # ((basis, SectionProps), ...): None on PF, both D-5 bases on the exact route
    tau: TauInputs | None

    def __post_init__(self):
        _require(isinstance(self.pipe, str) and isinstance(self.ro, Fraction), "pipe section")
        _require(isinstance(self.bases, tuple) and all(isinstance(b, tuple) and len(b) == 2 and (b[0] is None or b[0] in SECTION_BASES)
                                                        and isinstance(b[1], SectionProps) for b in self.bases), "pipe section bases")
        _require(_opt(self.tau, TauInputs), "pipe tau inputs")


def _key_pairs(xs, n):
    return isinstance(xs, tuple) and all(isinstance(x, tuple) and len(x) == 2 and isinstance(x[0], tuple) and len(x[0]) == n
                                         and isinstance(x[1], Fraction) for x in xs)


@dataclass(frozen=True, slots=True)
class ExpectedCase:
    """The reference of one load case: row key (kind, entity, location, component, basis) -> exact value; the
    maxima (pipe, basis) -> exact value; and PF's derived rows (no certificate; information only)."""
    case: str
    rows: tuple
    maxima: tuple
    derived: tuple

    def __post_init__(self):
        _require(isinstance(self.case, str) and _key_pairs(self.rows, 5) and _key_pairs(self.maxima, 2) and _key_pairs(self.derived, 5),
                 "expected case")
        for xs in (self.rows, self.maxima, self.derived):
            _require(len({k for k, _ in xs}) == len(xs), "unique expected keys")


@dataclass(frozen=True, slots=True)
class Expected:
    """The reference a model's runs are judged against (its own, its twin's or a control)."""
    variant: str
    Lb: float                 # the node bounding-box diagonal, m
    sections: tuple           # PipeSection, one per pipe
    cases: tuple              # ExpectedCase, one per load case of the model
    bases: tuple              # the section bases the route publishes: (None,) on PF; SECTION_BASES on the exact route
    single_headline: bool     # PF: one max_open_formula_stress; exact: that null and one headline per basis
    expect: str | None        # PF admission: "rows" or "refused"; None on the exact route (no admission expectation)
    refusal: str | None       # PF: the code an expected refusal names
    warnings: frozenset       # PF: expected warning codes
    derived_judged: bool      # PF: C11 on the derived rows

    def __post_init__(self):
        _require(isinstance(self.variant, str) and _num(self.Lb) and _tuple_of(self.sections, PipeSection)
                 and _tuple_of(self.cases, ExpectedCase), "expected")
        _require(isinstance(self.bases, tuple) and all(b is None or b in SECTION_BASES for b in self.bases), "expected bases")
        _require(isinstance(self.single_headline, bool) and isinstance(self.derived_judged, bool), "expected flags")
        _require(self.expect in (None, "rows", "refused") and _opt(self.refusal, str), "expected admission")
        _require(isinstance(self.warnings, frozenset), "expected warnings")


@dataclass(frozen=True, slots=True)
class ModelView:
    """A model of the reference set: its identity, its load cases, its connected-body count and the sha256 of
    its runner-input line; meta is (key, text) pairs for reporting only."""
    route: str
    tag: str
    family: str
    theory: str
    twin: str | None
    cases: tuple
    request_sha256: str
    harness_error: str | None   # a model this judge does not model (more than one body, components): C0
    meta: tuple

    def __post_init__(self):
        _require(self.route in ROUTES and all(isinstance(x, str) for x in (self.tag, self.family, self.theory, self.request_sha256)), "model")
        _require(_opt(self.twin, str) and _opt(self.harness_error, str) and _tuple_of(self.cases, str) and self.cases, "model cases")
        _require(isinstance(self.meta, tuple) and all(isinstance(m, tuple) and len(m) == 2 and all(isinstance(x, str) for x in m) for m in self.meta),
                 "model meta")
