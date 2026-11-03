"""RT-A4 (DEL-05-02 Work ``absolute-sum-srss-combinations.md`` sections 2-5): the verifiers' view of a
combination's operands, an independent test-only mirror of ``core/reporting/result_export/src/combination_graph.rs``.

A term names a load case or a combination by kind; a subtraction's or envelope's id names the load case when one
has it, a combination otherwise. Over published records, in their order, this re-derives the fixed-load count
kappa, the load cases each combination reaches, decision OQ-1 and the unsigned row values.

Independence: standard-library imports only. Every value is an exact ``fractions.Fraction``:

- the absolute sum A is the exact sum of |f x|, rounded once (CPython's correctly rounded integer division);
- the SRSS takes z = RN(f x) and rounds the exact root of the exact sum of squares without any candidate or
  midpoint search: at the result's binade quantum Q, n = isqrt(floor(S / Q^2)) and the exact remainder against
  n + 1/4 chooses n or n + 1, ties to even;
- kappa is an exact rational whose odd significand may need at most 4,096 bits, checked after each product and
  each partial sum, as the kernel's representation is.

A value beyond the binary64 range is ``None`` (refused), never an infinity.
"""
from __future__ import annotations

from collections.abc import Mapping
from fractions import Fraction
import math
import sys
from typing import Any

ABSOLUTE_SUM_BASIS = "explicit_user_absolute_sum"
SRSS_BASIS = "explicit_user_srss"
ABSOLUTE_SUM_SIGN = ("unsigned value: the absolute sum of |factor × source result| over explicit user terms; not an "
                     "equilibrium state")
SRSS_SIGN = ("unsigned value: the square root of the sum of squares of factor × source result over explicit user terms; "
             "not an equilibrium state")
SOLVED_STATE_PLUS_INCREMENTS_SIGN = ("positive value follows explicit user linear combination of matching source result "
                                     "sign conventions; a solved state plus derived differences or bounds; not an "
                                     "equilibrium state")
OPERAND_WITHHELD = "LOAD_COMBINATION_OPERAND_WITHHELD"
FIXED_LOAD_COUNT_UNREPRESENTABLE = "COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE"
TERM_BASES = ("mechanics", "absolute_sum", "srss")
MAX_BITS = 4096
UNIT_BOUND = Fraction(1, 2 ** 46)
_MAX = Fraction(sys.float_info.max)
_TINY = Fraction(1, 2 ** 1074)

# kappa's undefined states.
UNREPRESENTABLE, MIXED, UNSIGNED_NONZERO, OPERAND = "unrepresentable", "envelope_mixed", "unsigned_nonzero", "operand"


def fixed_load_count_message(cid: str) -> str:
    return (f"combination {cid}: its fixed-load count (the number of times it carries each fixed load) needs more than "
            "4,096 significand bits exactly, so it is undefined; a gated use never treats an undefined count as passing")


def operand_withheld_message(named: list[tuple[str, str]]) -> str:
    text = ", ".join(f"{cid} ({code})" for cid, code in named)
    return f"Combination withheld: its operand combination(s) {text} are withheld, so it is withheld whole."


# ----------------------------------------------------------------------------- operands

def evidence_term(term: Any) -> tuple[tuple[str, str], float] | None:
    """A term ``{load_case_id, factor}`` or ``{combination_id, factor}`` as ((kind, id), factor)."""
    if not isinstance(term, Mapping) or len(term) != 2 or "factor" not in term:
        return None
    factor = term["factor"]
    if type(factor) not in (int, float) or not math.isfinite(factor):
        return None
    for key, kind in (("load_case_id", "case"), ("combination_id", "combination")):
        if key in term:
            value = term[key]
            return ((kind, value), float(factor)) if isinstance(value, str) and value else None
    return None


def resolve(cid: str, cases: set[str]) -> tuple[str, str]:
    return ("case", cid) if cid in cases else ("combination", cid)


def owns(operand: tuple[str, str], row: Any) -> bool:
    basis = row.get("basis_ref") if isinstance(row, Mapping) else None
    ref_type = "load_case" if operand[0] == "case" else "combination"
    return isinstance(basis, Mapping) and basis.get("ref_type") == ref_type and basis.get("ref_id") == operand[1]


def row_source(operand: tuple[str, str], result_id: str) -> dict:
    key = "load_case_id" if operand[0] == "case" else "combination_id"
    return {key: operand[1], "result_id": result_id}


def model_operands(combination: Mapping[str, Any], cases: set[str]) -> list[tuple[str, str]]:
    basis = combination.get("basis")
    if basis == "result_state_subtraction":
        return [resolve(combination[k], cases) for k in ("minuend_id", "subtrahend_id") if isinstance(combination.get(k), str)]
    if basis == "range_envelope":
        return [resolve(i, cases) for i in sorted(i for i in (combination.get("operand_ids") or []) if isinstance(i, str))]
    out = []
    for term in combination.get("terms") or []:
        if isinstance(term, Mapping):
            if isinstance(term.get("combination"), str):
                out.append(("combination", term["combination"]))
            elif isinstance(term.get("load_case"), str):
                out.append(("case", term["load_case"]))
    return out


def model_order(combinations: list[Any], cases: set[str]) -> list[int]:
    """Topological, the earliest ready combination in the array first; one on a cycle is never ordered."""
    index: dict[str, int] = {}
    for i, c in enumerate(combinations):
        if isinstance(c, Mapping) and isinstance(c.get("id"), str):
            index.setdefault(c["id"], i)
    deps = [[index[o[1]] for o in model_operands(c, cases) if o[0] == "combination" and o[1] in index] for c in combinations]
    done, order = [False] * len(combinations), []
    while True:
        ready = next((i for i in range(len(combinations)) if not done[i] and all(done[d] and d != i for d in deps[i])), None)
        if ready is None:
            return order
        done[ready] = True
        order.append(ready)


def evidence_expression(combination: Mapping[str, Any]) -> dict:
    basis = combination.get("basis")
    if basis in TERM_BASES:
        terms = []
        for t in combination.get("terms") or []:
            if "combination" in t:
                terms.append({"combination_id": t.get("combination"), "factor": t.get("factor")})
            else:
                terms.append({"load_case_id": t.get("load_case"), "factor": t.get("factor")})
        return {"terms": terms}
    if basis == "result_state_subtraction":
        return {"minuend_id": combination.get("minuend_id"), "subtrahend_id": combination.get("subtrahend_id")}
    return {"operand_ids": combination.get("operand_ids"), "mode": combination.get("mode")}


# ----------------------------------------------------------------------------- kappa

def _representable(value: Fraction) -> bool:
    if value == 0:
        return True
    n = abs(value.numerator)
    n >>= (n & -n).bit_length() - 1
    return n.bit_length() <= MAX_BITS


def _defined(kappa: Any) -> bool:
    return isinstance(kappa, Fraction)


class Graph:
    """kappa and the reached load cases of the combinations read so far, in order."""

    def __init__(self) -> None:
        self.kappa: dict[str, Any] = {}
        self.reached: dict[str, list[str]] = {}

    def kappa_of(self, operand: tuple[str, str]) -> Any:
        if operand[0] == "case":
            return Fraction(1)
        return self.kappa.get(operand[1], OPERAND)

    def knows(self, cid: str) -> bool:
        return cid in self.kappa

    def add(self, cid: str, basis: str, operands: list[tuple[str, str]], factors: list[float]) -> Any:
        kappas = [self.kappa_of(o) for o in operands]
        if basis == "mechanics":
            kappa: Any = Fraction(0)
            overflow = False
            for factor, k in zip(factors, kappas):
                if not _defined(k):
                    kappa = OPERAND
                    break
                if overflow:
                    continue
                product = Fraction(factor) * k
                total = kappa + product
                if not (_representable(product) and _representable(total)):
                    overflow = True
                    continue
                kappa = total
            else:
                kappa = UNREPRESENTABLE if overflow else kappa
        elif basis == "result_state_subtraction" and len(kappas) == 2:
            if not all(_defined(k) for k in kappas):
                kappa = OPERAND
            else:
                difference = kappas[0] - kappas[1]
                kappa = difference if _representable(difference) else UNREPRESENTABLE
        elif basis == "range_envelope":
            if not all(_defined(k) for k in kappas):
                kappa = OPERAND
            elif all(k == kappas[0] for k in kappas):
                kappa = kappas[0] if kappas else Fraction(0)
            else:
                kappa = MIXED
        elif basis in ("absolute_sum", "srss"):
            if not all(_defined(k) for k in kappas):
                kappa = OPERAND
            else:
                kappa = Fraction(0) if all(k == 0 for k in kappas) else UNSIGNED_NONZERO
        else:
            kappa = OPERAND
        cases: list[str] = []
        for o in operands:
            for case in ([o[1]] if o[0] == "case" else self.reached.get(o[1], [])):
                if case not in cases:
                    cases.append(case)
        self.reached[cid] = cases
        self.kappa[cid] = kappa
        return kappa

    def solved_state_plus_increments(self, terms: list[tuple[tuple[str, str], float]]) -> bool:
        base = others = 0
        for operand, factor in terms:
            if operand[0] == "case":
                if factor != 1.0:
                    return False
                base += 1
            elif _defined(k := self.kappa_of(operand)) and k == 0:
                others += 1
            else:
                return False
        return base == 1 and others >= 1


def mechanics_gate(own: Any, operands: list[Any]) -> str | None:
    """'operand' when an operand's kappa is undefined, 'unit' when |kappa - 1| > 2^-46 or kappa is undefined."""
    if not all(_defined(k) for k in operands):
        return "operand"
    if _defined(own) and abs(own - 1) <= UNIT_BOUND:
        return None
    return "unit"


def unsigned_gate(operands: list[Any]) -> bool:
    """Whether an absolute sum or SRSS is withheld: some operand's kappa is not exactly 0, or is undefined."""
    return not all(_defined(k) and k == 0 for k in operands)


# ----------------------------------------------------------------------------- unsigned values

def _round(value: Fraction) -> float | None:
    if value == 0:
        return 0.0
    try:
        out = value.numerator / value.denominator
    except OverflowError:
        return None
    return out


def _floor_log2(value: Fraction) -> int:
    e = value.numerator.bit_length() - value.denominator.bit_length()
    while Fraction(2) ** e > value:
        e -= 1
    while Fraction(2) ** (e + 1) <= value:
        e += 1
    return e


def _sqrt_rounded(total: Fraction) -> float | None:
    """RN(sqrt(total)), ties to even, at the binary64 quantum of the result's binade; None beyond the range."""
    if total == 0:
        return 0.0
    e = _floor_log2(total) // 2  # floor(log2(sqrt(total)))
    quantum = Fraction(2) ** (max(e, -1022) - 52)
    scaled = total / (quantum * quantum)
    n = math.isqrt(scaled.numerator // scaled.denominator)
    remainder = scaled - n * n
    half = Fraction(n) + Fraction(1, 4)  # sqrt(scaled) vs n + 1/2  <=>  scaled - n^2 vs n + 1/4
    if remainder > half or (remainder == half and n % 2 == 1):
        n += 1
    value = n * quantum
    if value > _MAX:
        return None
    return float(value)


def unsigned_value(srss: bool, factors: list[float], values: list[float]) -> float | None:
    if len(factors) != len(values) or not values:
        return None
    if not all(math.isfinite(v) for v in values):
        return None
    if not srss:
        return _round(sum((abs(Fraction(f) * Fraction(v)) for f, v in zip(factors, values)), Fraction(0)))
    squares = Fraction(0)
    for f, v in zip(factors, values):
        z = _round(Fraction(f) * Fraction(v))
        if z is None:
            return None
        squares += Fraction(z) * Fraction(z)
    return _sqrt_rounded(squares)
