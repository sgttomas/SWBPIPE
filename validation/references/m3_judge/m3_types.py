"""The canonical evidence view of the M3 judge (refutation round 5): the only data the comparison reads.

The validator (m3_view.py) builds these values from a strictly parsed reference line and a strictly parsed raw
runner record; anything it did not validate into a view does not exist for judging. Every value is immutable
and of an exact type, checked on construction with `type(x) is ...` (the judge's final round: no subclass of str,
float, Fraction, tuple, frozenset or a view type is admitted, so no value can carry mutable state or override
arithmetic or iteration). The values are strings, binary64 values, exact rationals, booleans, None, and tuples and
frozensets of them; no dictionaries and no raw JSON. This module imports no JSON reader.

Identities are unique by construction of the view: a row identity (case, kind, entity, location, component,
section basis), an extremum identity (case, pipe, section basis), a standing's case and a headline's basis each
occur at most once, or the record is refused before a view exists.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from fractions import Fraction

ROUTES = ("pf", "exact")
ENTRIES = ("typed", "captured")
SECTION_BASES = ("nominal_section", "reduced_section")


def _str(x):
    return type(x) is str


def _ostr(x):
    return x is None or type(x) is str


def _num(x):
    return type(x) is float and math.isfinite(x)


def _onum(x):
    return x is None or _num(x)


def _q(x):
    return type(x) is Fraction and type(x.numerator) is int and type(x.denominator) is int


def _tuple_of(xs, ok):
    return type(xs) is tuple and all(ok(x) for x in xs)


def _strs(xs):
    return type(xs) is frozenset and all(type(x) is str for x in xs)


def _basis(x):
    return x is None or (type(x) is str and x in SECTION_BASES)


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
        _require(all(_str(x) for x in (self.case, self.kind, self.entity, self.unit)), "row strings")
        _require(_ostr(self.location) and _ostr(self.component) and _ostr(self.source), "row optional strings")
        _require(_basis(self.basis), "row section basis")
        _require(_onum(self.value), "row value")

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
        _require(_str(self.case) and _str(self.pipe), "extremum strings")
        _require(_basis(self.basis), "extremum section basis")
        _require(_onum(self.lower) and _onum(self.upper), "extremum bounds")
        _require(self.lower is None or self.upper is None or self.lower <= self.upper, "extremum interval lo <= hi")


@dataclass(frozen=True, slots=True)
class Headline:
    """A summary headline of the maximum stress: basis None is `max_open_formula_stress`, otherwise
    `max_open_formula_stress_<basis>`; is_null when the headline itself is null."""
    basis: str | None
    is_null: bool
    value: float | None

    def __post_init__(self):
        _require(_basis(self.basis), "headline basis")
        _require(type(self.is_null) is bool and _onum(self.value) and (not self.is_null or self.value is None), "headline value")


@dataclass(frozen=True, slots=True)
class Standing:
    """The K-D5 standing of one load case (a code of the closed set) and its message."""
    case: str
    code: str
    message: str

    def __post_init__(self):
        _require(all(_str(x) for x in (self.case, self.code, self.message)), "standing strings")


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
    contract: str | None       # the semantic contract whose table every row matched (None without rows); reported only

    def __post_init__(self):
        _require(_str(self.route) and self.route in ROUTES, "run route")
        _require(_ostr(self.contract), "run contract")
        _require(all(_str(x) for x in (self.tag, self.entry, self.mode, self.request_sha256, self.runner_sha256, self.product_head)), "run strings")
        _require(_ostr(self.error) and _ostr(self.refusal), "run outcome strings")
        _require(_tuple_of(self.standings, lambda x: type(x) is Standing) and _tuple_of(self.rows, lambda x: type(x) is Row)
                 and _tuple_of(self.extrema, lambda x: type(x) is Extremum) and _tuple_of(self.headlines, lambda x: type(x) is Headline), "run tuples")
        _require(_strs(self.warnings), "run warnings")
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
        _require(all(_q(x) for x in (self.A, self.Z, self.J)), "section properties")


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
        _require(all(_num(x) for x in (self.E, self.G, self.od, self.wall, self.mill)), "tau binary64 inputs")
        _require(all(_q(x) for x in (self.I, self.Z, self.tau_c)), "tau exact inputs")


@dataclass(frozen=True, slots=True)
class PipeSection:
    pipe: str
    ro: Fraction
    bases: tuple          # ((basis, SectionProps), ...): None on PF, both D-5 bases on the exact route
    tau: TauInputs | None

    def __post_init__(self):
        _require(_str(self.pipe) and _q(self.ro), "pipe section")
        _require(_tuple_of(self.bases, lambda b: type(b) is tuple and len(b) == 2 and _basis(b[0]) and type(b[1]) is SectionProps), "pipe section bases")
        _require(self.tau is None or type(self.tau) is TauInputs, "pipe tau inputs")


def _key_pairs(xs, n):
    return _tuple_of(xs, lambda x: type(x) is tuple and len(x) == 2 and type(x[0]) is tuple and len(x[0]) == n and _q(x[1])
                     and _str(x[0][0]) and _str(x[0][1]) and all(_ostr(y) for y in x[0][2:]))


@dataclass(frozen=True, slots=True)
class ExpectedCase:
    """The reference of one load case: row key (kind, entity, location, component, basis) -> exact value; the
    maxima (pipe, basis) -> exact value; and PF's derived rows (no certificate; information only)."""
    case: str
    rows: tuple
    maxima: tuple
    derived: tuple

    def __post_init__(self):
        _require(_str(self.case) and _key_pairs(self.rows, 5) and _key_pairs(self.derived, 5), "expected case rows")
        _require(_tuple_of(self.maxima, lambda x: type(x) is tuple and len(x) == 2 and type(x[0]) is tuple and len(x[0]) == 2 and _str(x[0][0])
                           and _basis(x[0][1]) and _q(x[1])), "expected case maxima")
        for xs in (self.rows, self.maxima, self.derived):
            _require(len({k for k, _ in xs}) == len(xs), "unique expected keys")


@dataclass(frozen=True, slots=True)
class Expected:
    """The reference a model's runs are judged against (its own, its twin's or a control), read only from a
    reference file whose sha256 the judging pass verifies against the committed REFERENCE_SHA256SUMS."""
    variant: str
    sections: tuple           # PipeSection, one per pipe of the request
    cases: tuple              # ExpectedCase, one per load case of the request
    bases: tuple              # the section bases the route publishes: (None,) on PF; SECTION_BASES on the exact route
    single_headline: bool     # PF: one max_open_formula_stress; exact: that null and one headline per basis
    expect: str | None        # PF admission: "rows" or "refused"; None on the exact route (no admission expectation)
    refusal: str | None       # PF: the code an expected refusal names
    warnings: frozenset       # PF: expected warning codes
    derived_judged: bool      # PF: C11 on the derived rows

    def __post_init__(self):
        _require(_str(self.variant) and _tuple_of(self.sections, lambda x: type(x) is PipeSection)
                 and _tuple_of(self.cases, lambda x: type(x) is ExpectedCase), "expected")
        _require(_tuple_of(self.bases, _basis), "expected bases")
        _require(type(self.single_headline) is bool and type(self.derived_judged) is bool, "expected flags")
        _require((self.expect is None or (_str(self.expect) and self.expect in ("rows", "refused"))) and _ostr(self.refusal), "expected admission")
        _require(_strs(self.warnings), "expected warnings")


@dataclass(frozen=True, slots=True)
class ModelView:
    """A model of the reference set, parsed from its request (which every run's request_sha256 binds): its load
    cases, its node bounding-box diagonal L_b (m) and connected-body count, and the sha256 of its runner-input
    line. family, theory, twin and meta (key, text) come from the hash-verified reference line, for reporting and
    for nonvacuity's families."""
    route: str
    tag: str
    family: str
    theory: str
    twin: str | None
    cases: tuple
    Lb: float
    request_sha256: str
    harness_error: str | None   # a model this judge does not model (more than one body, components): C0
    meta: tuple
    entries: tuple = ENTRIES     # the entries the inventory expects of this model (its reference line's meta; both by default)

    def __post_init__(self):
        _require(_str(self.route) and self.route in ROUTES and all(_str(x) for x in (self.tag, self.family, self.theory, self.request_sha256)), "model")
        _require(_ostr(self.twin) and _ostr(self.harness_error) and _tuple_of(self.cases, _str) and len(self.cases) > 0, "model cases")
        _require(type(self.Lb) is float and math.isfinite(self.Lb) and self.Lb >= 0.0, "model L_b")
        _require(_tuple_of(self.meta, lambda m: type(m) is tuple and len(m) == 2 and _str(m[0]) and _str(m[1])), "model meta")
        _require(_tuple_of(self.entries, _str) and len(self.entries) > 0 and self.entries == tuple(e for e in ENTRIES if e in self.entries),
                 "model entries (a non-empty subset of ENTRIES, in their order)")
