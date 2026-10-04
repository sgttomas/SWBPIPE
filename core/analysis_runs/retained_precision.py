"""Standalone prepared-result statement validation; never producer authentication.

The reader has no solver, native registration, or carrier side effects. Exact
helpers below implement only the finite binary64 decisions named by the policy.
"""
from __future__ import annotations

import json
import hashlib
import math
import re
import struct
from functools import lru_cache
from copy import deepcopy
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PROFILE = "product_preview_retained_w1a_v2"
DEFINITION_ID = "RP-PREPARED-ORDINARY-DUAL-v1"
DEFINITION_HASH = "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349"
TABLE_HASH = "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8"
METHOD = "contribution_preserving_multiprecision_v1"
SAFE = (1 << 53) - 1
MAX_BITS = 0x7FEFFFFFFFFFFFFF
# D-U6-1 (I66 U6a): this flag gates eligibility only, as Rust's
# IMPLEMENTATION_COMPLETE and TypeScript's SUMMARY_COVERAGE_COMPLETE do. It is on
# since U7 (D-U7-5); every gate runs regardless.
_IMPLEMENTATION_COMPLETE = True


class RetainedPrecisionError(ValueError):
    def __init__(self, gate: str, code: str, detail: str | None = None):
        self.gate, self.code, self.detail = gate, code, detail
        super().__init__(code)


def bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def from_bits(value: str) -> float:
    if not isinstance(value, str) or re.fullmatch(r"[0-9a-f]{16}", value) is None:
        raise ValueError("binary64 encoding")
    number = struct.unpack(">d", bytes.fromhex(value))[0]
    if not math.isfinite(number):
        raise ValueError("nonfinite binary64")
    return number


def _positive_parts(value: float) -> tuple[int, int]:
    if not math.isfinite(value) or value < 0:
        raise ValueError("nonnegative finite operand required")
    word = int(bits(value), 16) & ((1 << 63) - 1)
    exponent, mantissa = word >> 52, word & ((1 << 52) - 1)
    return (mantissa, -1074) if exponent == 0 else (mantissa | (1 << 52), exponent - 1075)


def _ceil_dyadic(mantissa: int, exponent: int) -> float:
    if mantissa == 0:
        return 0.0
    top = mantissa.bit_length() - 1 + exponent
    quantum = max(top - 52, -1074)
    shift = quantum - exponent
    if shift > 0:
        if shift >= mantissa.bit_length():
            rounded = 1
        else:
            rounded, remainder = divmod(mantissa, 1 << shift)
            rounded += bool(remainder)
    else:
        rounded = mantissa << -shift
    while rounded >= 1 << 53:
        # A rounding carry is exactly one bit; no second rounding occurs.
        assert rounded == 1 << 53
        rounded >>= 1
        quantum += 1
    top = rounded.bit_length() - 1 + quantum
    if top > 1023:
        raise ValueError("binary64 upper bound overflows")
    if top < -1022:
        word = rounded << (quantum + 1074)
    else:
        shift = 53 - rounded.bit_length()
        word = ((top + 1023) << 52) | ((rounded << shift) - (1 << 52))
    if word > MAX_BITS:
        raise ValueError("binary64 upper bound overflows")
    return struct.unpack(">d", word.to_bytes(8, "big"))[0]


def upward_product(a: float, b: float) -> float:
    ma, ea = _positive_parts(a)
    mb, eb = _positive_parts(b)
    return _ceil_dyadic(ma * mb, ea + eb)


def upward_small_sum(b0: float, rounding: float) -> float:
    """One RU64 of the exact b0 + rounding + minimum-subnormal sum."""
    ma, ea = _positive_parts(b0)
    mb, eb = _positive_parts(rounding)
    return _ceil_dyadic((ma << (ea + 1074)) + (mb << (eb + 1074)) + 1, -1074)


def _scaled_component(value: float, power: int) -> float:
    if power not in (53, 64) or not math.isfinite(value) or value < 0:
        raise ValueError("scaled component input")
    if value == 0:
        return 0.0
    divisor = float(1 << power)
    nearest = value / divisor
    back = nearest * divisor
    return math.nextafter(nearest, math.inf) if back < value else nearest


def absolute_bound(value: float, scale: float) -> float:
    if not math.isfinite(value) or not math.isfinite(scale) or scale < 0:
        raise ValueError("bound input")
    # Helpers accept either sign of zero and return canonical +0; G2 still
    # rejects negative-zero wire scales before any numerical helper is used.
    if scale == 0:
        return 0.0
    b0 = _scaled_component(scale, 64)
    return upward_small_sum(b0, _scaled_component(abs(value), 53)) if 0 < scale < 2.0 ** -988 else b0


def _need(ok: bool, gate: str, suffix: str):
    if not ok:
        raise RetainedPrecisionError(gate, suffix if suffix.startswith("SOURCE_") else "RETAINED_PRECISION_" + suffix)


@lru_cache(maxsize=1)
def _schema():
    return json.loads((ROOT / "schemas/retained_precision_mp_v2.schema.json").read_text())


def _same(a, b):
    if isinstance(a, bool) or isinstance(b, bool):
        return type(a) is type(b) and a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return a == b
    if type(a) is not type(b): return False
    if isinstance(a, dict): return a.keys() == b.keys() and all(_same(v, b[k]) for k, v in a.items())
    if isinstance(a, list): return len(a) == len(b) and all(_same(x, y) for x, y in zip(a, b))
    return a == b


def _shape(value, spec):
    """Only the vocabulary in this one pinned closed receipt schema; G2 is separate."""
    if "$ref" in spec: return _shape(value, _schema()["$defs"][spec["$ref"].split("/")[-1]])
    if "oneOf" in spec: return sum(_shape(value, s) for s in spec["oneOf"]) == 1
    if "const" in spec and not _same(value, spec["const"]): return False
    if "enum" in spec and not any(_same(value, v) for v in spec["enum"]): return False
    kind = spec.get("type")
    if kind == "object":
        return (type(value) is dict and set(spec["required"]) <= value.keys()
                and value.keys() <= spec["properties"].keys()
                and all(_shape(v, spec["properties"][k]) for k, v in value.items()))
    if kind == "array":
        return type(value) is list and len(value) >= spec.get("minItems", 0) and len(value) <= spec.get("maxItems", SAFE) and all(_shape(v, spec["items"]) for v in value)
    if kind == "string": return type(value) is str and len(value) >= spec.get("minLength", 0)
    if kind == "boolean": return type(value) is bool
    if kind == "null": return value is None
    if kind in ("number", "integer"): return type(value) in (int, float)
    return True


def _encoding(value, spec):
    if "$ref" in spec: return _encoding(value, _schema()["$defs"][spec["$ref"].split("/")[-1]])
    if "oneOf" in spec:
        return _encoding(value, next(s for s in spec["oneOf"] if _shape(value, s)))
    tag = spec.get("x-rp-encoding")
    if tag in ("uint", "i32"):
        _need(type(value) in (int, float) and math.isfinite(value) and int(value) == value and spec["minimum"] <= value <= spec["maximum"] and not (value == 0 and math.copysign(1, value) < 0), "G2", "ENCODING_MISMATCH")
    if tag in ("bits", "nonnegative_bits"):
        try: number = from_bits(value)
        except (ValueError, OverflowError): _need(False, "G2", "ENCODING_MISMATCH")
        if tag == "nonnegative_bits": _need(number >= 0 and value != "8000000000000000", "G2", "ENCODING_MISMATCH")
    if tag == "hash": _need(re.fullmatch(r"[0-9a-f]{64}", value) is not None, "G2", "ENCODING_MISMATCH")
    if spec.get("type") == "object":
        for k, v in value.items(): _encoding(v, spec["properties"][k])
    if spec.get("type") == "array":
        for v in value: _encoding(v, spec["items"])


def _hash(domain, payload):
    from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
    return canonical_sha256_checked_v1({"domain": domain, "payload": payload})


def _at(items, index, gate="G5", code="PRODUCT_ATTEMPT_MISMATCH"):
    _need(type(index) in (int, float) and int(index) == index and 0 <= index < len(items), gate, code)
    return items[int(index)]


def _count(record):
    return int(record["value"]) if record["kind"] == "exact" else None


STATUS_FAULTS = {"exact": frozenset(), "overflow": frozenset({"overflow"}), "inconsistent": frozenset({"inconsistent"}),
                 "both": frozenset({"overflow", "inconsistent"})}


def _objects(value):
    if isinstance(value, dict):
        yield value
        for v in value.values(): yield from _objects(v)
    elif isinstance(value, list):
        for v in value: yield from _objects(v)


def _fault_owner(a, path):
    """R3' owner scopes (checkpoint A, D8): a member's PreparationWork, a lane's LaneWork, the values
    completion, else the whole ProofTrace."""
    proof = a["proof"]
    if path[:2] == ("preparation", "members"):
        return a["preparation"]["members"][path[2]]["work"]
    if path[:3] == ("result", "error", "section"):
        members = a["preparation"]["members"]
        return members[-1]["work"] if members else None
    if path[:2] == ("proof", "lanes"):
        return proof["lanes"][path[2]]["work"]
    if path[:3] == ("result", "error", "cause") and a["result"]["error"]["kind"] == "values":
        return None if proof is None else proof["completion"]
    return proof


def _located(value, path=()):
    if isinstance(value, dict):
        yield path, value
        for k, v in value.items(): yield from _located(v, path + (k,))
    elif isinstance(value, list):
        for i, v in enumerate(value): yield from _located(v, path + (i,))


def _statuses(value):
    seen = set()
    for o in _objects(value):
        if o.get("kind") == "unavailable" and o.get("fault") in STATUS_FAULTS: seen |= STATUS_FAULTS[o["fault"]]
        if o.get("sticky_status") in STATUS_FAULTS: seen |= STATUS_FAULTS[o["sticky_status"]]
    return seen


OPERATIONAL_ERROR_PATHS = (("operational", "old"), ("operational", "new"))


def _accounting_rules(a):
    """Checkpoint A rulings (D8: R1', R2', R3', R4), G5 WORK class 4 (C3:304), every product attempt.

    R2' adds OperationalError accounting, which PP returns only for a lost ScalarWork
    (retained_product.rs `ScalarWork::check` through `ScalarWork::operation`), located as a MemberOperational error, a CaptureError prepared_arithmetic cause
    or a G5aError operational/arithmetic cause. R3' covers every fault-bearing spelling
    (work_accounting{fault}, a nested stop/work_accounting, a view work{fault}) against its owner.
    R4: a SectionError accounting needs a non-exact status in that member's PreparationWork
    (FK product_certificate.rs `SectionPreparationWork::check`).

    Ruling 06d (I62 ACCOUNTING_CAUSES R1-R3) for the base classes:

    R1: an adapter overflow is never emittable. AdapterWork::enter (retained_product.rs `AdapterWork::enter`) faults only when
    counts[event] + amount overflows and keeps counts[event]; every amount is < 2^63 + 2^61, so the
    retained prefix is >= 2^62, which C3:233-236 requires to be emitted and forbids above safe-U.
    Every CaptureError/G5aError accounting{event} cause is built from that sticky fault
    (retained_product.rs `AdapterWork::require`, `ProductCapture::solver_observations`, `ProductCapture::g5a`), so it is covered too.
    R2: ScalarTrace.lost is set only when entered/checks is at u64::MAX (retained_product.rs `ScalarWork::check` through `ScalarWork::operation`).
    R3: a work_accounting{fault} cause is raised from the owning trace's status (final_case.rs `ProductCertificateSpent::visit`, `ProductCertificateSpent::f64_op`, `begin_prepared_product`;
    FK product_certificate.rs `NumericWork::begin`, `NumericWork::checked`), so the attempt's emitted Count faults and sticky statuses
    must contain that fault."""
    located = list(_located(a))
    r1 = a["adapter"]["fault"] is None and not any(o.get("kind") == "accounting" and "event" in o for _, o in located)
    r2 = not any(o.get("lost") is True for _, o in located)
    error = a["result"].get("error") if a["result"]["kind"] == "unavailable" else None
    operational_errors = [m["result"].get("error") for side in ("old", "new") for m in a["operational"][side] if m["result"]["kind"] != "ready"]
    for path, o in _located(error):
        if o.get("kind") == "prepared_arithmetic": operational_errors.append(o.get("cause"))
        if o.get("kind") == "g5a": operational_errors.extend(x["cause"] for _, x in _located(o.get("cause")) if x.get("kind") in ("operational", "arithmetic") and isinstance(x.get("cause"), dict))
    for check in ((a["proof"] or {}).get("checks") or {}).values():
        if check.get("kind") == "failed" and (check.get("error") or {}).get("kind") == "g5a":
            operational_errors.extend(x["cause"] for _, x in _located(check["error"].get("cause")) if x.get("kind") in ("operational", "arithmetic") and isinstance(x.get("cause"), dict))
    r2 = r2 and not any(isinstance(e, dict) and e.get("kind") == "accounting" and "event" not in e for e in operational_errors)
    r3 = True
    for path, o in located:
        spelled = (o.get("kind") == "work_accounting" or o.get("tag") == "work_accounting" or o.get("kind") == "work") and "fault" in o
        if spelled:
            owner = _fault_owner(a, path)
            r3 = r3 and owner is not None and o["fault"] in STATUS_FAULTS and STATUS_FAULTS[o["fault"]] <= _statuses(owner)
    r4 = True
    for m in a["preparation"]["members"]:
        e = m["result"].get("error") if m["result"]["kind"] != "prepared" else None
        if isinstance(e, dict) and e.get("kind") == "accounting": r4 = r4 and bool(_statuses(m["work"]))
    section = (error or {}).get("section") if (error or {}).get("kind") == "preparation" else None
    if isinstance(section, dict) and section.get("kind") == "accounting":
        members = a["preparation"]["members"]
        r4 = r4 and bool(members) and bool(_statuses(members[-1]["work"]))
    return r1, r2, r3, r4


def _exact_work(value):
    if isinstance(value, dict):
        if value.get("kind") == "unavailable" and "fault" in value: return False
        if "sticky_status" in value and value["sticky_status"] != "exact": return False
        if value.get("lost") is True: return False
        return all(_exact_work(v) for v in value.values())
    if isinstance(value, list): return all(_exact_work(v) for v in value)
    return True


def _source_hash(source):
    return _hash("retained_precision_source_mp_v2", {k: v for k, v in source.items() if k != "index"})


def _native_source_encoding(source, include_loads):
    """Existing K4SRC/K4STF bytes, source.rs `PrimitiveSource::encoding` and `PrimitiveSource::stiffness_encoding`; no solve or exact ledger."""
    out = bytearray(b"K4SRC\x01" if include_loads else b"K4STF\x01")
    def uint(value):
        if type(value) not in (int, float) or int(value) != value or not 0 <= value <= 0xffffffff:
            raise ValueError("native u32 range")
        out.extend(struct.pack("<I", int(value)))
    def value(word):
        from_bits(word)
        out.extend(bytes.fromhex(word)[::-1])
    def dof(record):
        uint(record["node"])
        out.append(COMPONENTS.index(record["component"]))
    maps = source["id_maps"]
    uint(len(maps["nodes"]))
    for node in maps["nodes"]:
        for word in node["coordinates"]: value(word)
    uint(len(maps["members"]))
    for member in maps["members"]:
        for key in ("kernel_member", "node_i", "node_j"): uint(member[key])
        for key in ("E", "G", "A_K", "Iy_K", "Iz_K", "J_K"): value(member[key])
        for word in member["y_reference"]: value(word)
    uint(len(maps["springs"]))
    for spring in maps["springs"]:
        uint(spring["kernel_spring"]); dof(spring); value(spring["stiffness"])
    # The selected ordinary source family has no directional springs.
    uint(0)
    uint(len(source["constraints"]))
    for constraint in source["constraints"]:
        dof(constraint["dof"])
        if include_loads: value(constraint["value"])
    if include_loads:
        uint(len(source["nodal_terms"]))
        for term in source["nodal_terms"]:
            dof(term["dof"])
            identity = term["source_id"].encode("utf-8")
            uint(len(identity)); out.extend(identity); value(term["value"])
        uint(len(source["stations"]))
        for station in source["stations"]:
            uint(station["id"]); uint(station["member"]); value(station["fraction"])
        uint(len(source["supports"]))
        for support in source["supports"]:
            uint(support["id"]); uint(support["node"])
            out.extend(int(restrained) for restrained in support["restrained"])
            uint(len(support["springs"]))
            for spring in support["springs"]: uint(spring)
            if support["directional_springs"]:
                raise ValueError("unsupported directional spring")
            uint(0)
    return bytes(out)


def _preparation_payload(a):
    return {"definition_id": a["definition_id"], "definition_sha256": DEFINITION_HASH,
            "owner_ref": a["owner_ref"], "ordinary_attempt_ref": a["ordinary_attempt_ref"],
            "material_basis_ref": a["material_basis_ref"], "members": [
                {"member": m["member"], "old_source": m["old_source"], "old_facts": m["old_facts"], "section": m["result"]["section"]}
                for m in a["preparation"]["members"]]}


ESCALATING_STOPS = {"pivot", "condition", "residual_gate"}
PRECISION_SLOTS = [128, 256, 512, 1024]
SLOT_ORDER = ["s128", "s256", "s512", "s1024", "v256", "v512", "v1024"]


def _stop_of(reason):
    """The native AttemptStop carried by a Reason, or None (C1:114 closed translation)."""
    if not isinstance(reason, dict):
        return None
    if reason.get("space") == "stop":
        return reason
    if reason.get("space") == "attempt" and reason.get("tag") == "stop":
        return reason.get("stop")
    return None


_UNRESOLVED_OF_STOP = {"count_range": ("count_range", ("name",)), "work_accounting": ("work_accounting", ("fault",)),
                       "budget": ("budget", ("scope",)), "span": ("exact_sum_span", ()), "exponent": ("exponent_range", ()),
                       "zero_diagonal": ("zero_diagonal", ("global_dof",)), "arithmetic": ("arithmetic", ("error",)),
                       "resolution_scale": ("resolution_scale_unencodable", ("body", "kind")),
                       "publication_certificate": ("publication_certificate", ("index", "issue"))}


def _terminal_of(stop):
    """adaptive.rs `terminal` terminal(): the exact kernel terminal of a terminal stop, or
    None for an escalating stop (unreachable there). The wire omits WorkAccounting's private
    `prior` (schema Unresolved work_accounting {fault}; C3:261-263)."""
    if not isinstance(stop, dict):
        return None
    tag = stop.get("tag")
    if tag == "negative_energy":
        return {"kind": "refused", "reason": {"space": "refusal", "tag": "negative_energy", "i": stop.get("i"), "j": stop.get("j")}}
    if tag == "structure":
        return {"kind": "refused", "reason": {"space": "refusal", "tag": "structure"}}
    if tag in _UNRESOLVED_OF_STOP:
        name, keys = _UNRESOLVED_OF_STOP[tag]
        return {"kind": "unresolved", "reason": dict({"space": "unresolved", "tag": name}, **{k: stop.get(k) for k in keys})}
    return None


def _is_work_accounting(terminal):
    reason = terminal.get("reason") or {}
    return terminal.get("kind") == "unresolved" and reason.get("space") == "unresolved" and reason.get("tag") == "work_accounting"


def _g5_schedule(run, records, attempts, fail, body=None):
    """Checklist N1-N9, N13, N14: replay the actual native ladder (adaptive.rs `run_schedule_inner`).

    Candidates at slots 128/256/512, each verified at 2p. A candidate solve failure with an
    escalating stop (Pivot/Condition/ResidualGate) advances one slot (4542-4552); a failed
    verification solve rejects the candidate (VerificationFailed) and, if escalating, advances
    two slots (4576-4590); a verification-pass failure is terminal (4593-4611); a rejected
    candidate's completed verification becomes the next candidate while c+1<3 (4713-4758);
    Accepted+Verified selects (4696-4711); leaving the loop is the Ceiling (4762-4766).
    """
    fail(all(int(r["corrections"]) <= 3 for r in records))
    terminal = run["kernel_terminal"]
    # A WorkAccounting terminal exists natively only when a work status is not exact
    # (adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`); C1:66-68 forbids emitting such a run, and C2's
    # reachable Reason map (C2:17-40) has no unresolved/work_accounting. It lies outside
    # the emitted domain of the actual native schedule/terminal (C1:148), idle or not.
    fail(not _is_work_accounting(terminal))
    if not attempts:
        # Pre-schedule returns (adaptive.rs `solve_cases_projected`): invocation entry with a meter fault
        # (WorkAccounting, prior None, takes precedence) or exhaustion (Budget(invocation));
        # a refused group (its refusal, checked with C5); a CasePrep failure (LedgerUnavailable).
        fail(not records and int(run["case_charge"]) == 0 and int(run["invocation_increment"]) == 0
             and terminal["kind"] != "selected" and terminal["reason"] is not None)
        if run["origin"]["group"] is None:
            exhausted = body is not None and int(run["invocation_before"]) >= int(body["work"]["invocation_limit"])
            fail(exhausted and terminal == {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "budget", "scope": "invocation"}})
        elif body is not None:
            group = _at(body["groups"], run["origin"]["group"], code="ATTEMPT_MISMATCH")
            if group["preparation"]["kind"] == "ready":
                fail(terminal["kind"] == "refused" and (terminal["reason"] or {}).get("tag") == "ledger_unavailable")
        return
    c, next_record, ended, end_stop, escalated = 0, 0, False, None, False
    for ai, a in enumerate(attempts):
        fail(not ended and c < 3 and a["precision"] == PRECISION_SLOTS[c])
        cr_i = int(a["candidate_record"])
        cr = _at(records, cr_i, code="ATTEMPT_MISMATCH")
        if a["origin"]["kind"] == "fresh":
            fail(cr_i == next_record and cr["role"] == "candidate")
        else:
            prev = attempts[ai - 1] if ai else None
            fail(prev is not None and a["origin"].get("attempt") == ai - 1 and prev["outcome"]["kind"] == "rejected"
                 and prev["verification"] is not None and prev["verification"]["phase"] == "completed"
                 and int(prev["verification"]["record"]) == cr_i == next_record - 1 and cr["role"] == "verification_then_candidate")
        next_record = max(next_record, cr_i + 1)
        v, out = a["verification"], a["outcome"]
        kind = out["kind"]
        fail(kind in ("accepted", "rejected", "failed"))
        if v is not None:
            vi = int(v["record"])
            fail(vi == cr_i + 1 == next_record)
            vr = _at(records, vi, code="ATTEMPT_MISMATCH")
            next_record = vi + 1
            if v["phase"] == "failed":
                fail(vr["role"] == "verification" and vr["outcome"]["kind"] == "failed" and v["reason"] == vr["outcome"].get("reason")
                     and kind == "rejected" and out.get("reason") == {"space": "attempt", "tag": "verification_failed"})
            else:
                fail(v["phase"] == "completed" and v["reason"] is None)
                if vr["role"] == "verification":
                    fail(vr["outcome"]["kind"] == ("verified" if kind == "accepted" else "solved"))
                else:
                    fail(vr["role"] == "verification_then_candidate" and kind == "rejected")
        else:
            fail(kind == "failed" and _stop_of(out.get("reason")) is not None)
        if out.get("reason") == {"space": "attempt", "tag": "verification_failed"}:
            # D5c (adaptive.rs `run_schedule_inner`; C1:27): set only on a failed verification solve.
            fail(v is not None and v["phase"] == "failed")
        last = ai == len(attempts) - 1
        escalated = False
        if kind == "accepted":
            fail(v is not None and v["phase"] == "completed" and last)
            ended = True
        elif v is None:
            stop = _stop_of(out["reason"])
            if stop.get("tag") in ESCALATING_STOPS:
                c += 1; escalated = True
            else:
                ended, end_stop = True, stop
        elif v["phase"] == "failed":
            # A verification *solve* failure with an escalating stop skips two slots; a
            # verification-pass failure is terminal and never escalating (terminal() would be
            # unreachable for it, adaptive.rs `terminal`).
            stop = _stop_of(v["reason"]) or {}
            if stop.get("tag") in ESCALATING_STOPS:
                c += 2; escalated = True
            else:
                ended, end_stop = True, stop
        elif kind == "rejected":
            c += 1
            if c < 3:
                fail(not last and attempts[ai + 1]["origin"]["kind"] == "reused_verification")
        else:
            ended, end_stop = True, _stop_of(out["reason"])  # decision/pair/certificate stop after verification
    fail(next_record == len(records))
    ceiling = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}
    if attempts[-1]["outcome"]["kind"] == "accepted":
        fail(terminal["kind"] == "selected" and terminal["reason"] is None)
    elif ended:
        # N5: a terminal stop ends on its exact terminal() translation (adaptive.rs `terminal`).
        fail(terminal == _terminal_of(end_stop))
    elif escalated and c < 3:
        # N9: an escalating last stop with slots left ends only through a work fault
        # (adaptive.rs `run_schedule_inner`), which is never emitted (C1:66-68).
        fail(False)
    elif escalated:
        fail(terminal == ceiling)
    else:
        fail(terminal == ceiling)  # rejected p512 candidate leaves the loop (4762-4766)


def _g5_cache(body, run, records, fail, wf):
    """Checklist C1-C3 (adaptive.rs `obtain` obtain): a cached slot is reused, never
    rebuilt; a non-budget failure is cached and reused with the same build id; a budget
    failure is not cached; cache_after = cache_before plus this run's cacheable builds; a
    failed build fails the requesting record with the same stop."""
    cache = {e["slot"]: int(e["build"]) for e in run["cache_before"]}
    fail_ok = True
    for r in records:
        for field, flag in (("shared_build_ref", "shared_built_here"), ("verification_shared_build_ref", "verification_shared_built_here")):
            bi = r[field]
            if bi is None:
                continue
            b = _ref(body["builds"], bi)
            if b is None:
                wf(False)  # D16: a dangling build reference is a C1 build-provenance (WORK) defect
                continue
            if r["work"][flag]:
                wf(b["slot"] not in cache)
                if b["state"] != "budget_failure":
                    cache[b["slot"]] = int(bi)
            else:
                wf(cache.get(b["slot"]) == int(bi))
            wf((b["state"] == "success") == (b["reason"] is None))
            wf((b["state"] == "budget_failure") == ((b["reason"] or {}).get("tag") == "budget"))
            if b["state"] != "success":
                fail(r["outcome"]["kind"] == "failed" and _stop_of(r["outcome"].get("reason")) == b["reason"])
    wf(run["cache_after"] == sorted([{"slot": k, "build": v} for k, v in cache.items()], key=lambda e: SLOT_ORDER.index(e["slot"])))


def _integral(value):
    """D32: an integer by value (finite, integral, not -0, not a bool), else None."""
    if type(value) is int: return value
    if type(value) is float and math.isfinite(value) and value == int(value) and not (value == 0 and math.copysign(1, value) < 0): return int(value)
    return None


def _negative_zero(value):
    """D34: any JSON number equal to -0 anywhere in the receipt (C1 s4; C1 G2 row)."""
    if type(value) is float: return value == 0 and math.copysign(1.0, value) < 0
    if isinstance(value, dict): return any(_negative_zero(v) for v in value.values())
    if isinstance(value, list): return any(_negative_zero(v) for v in value)
    return False


def _normalize_integrals(value):
    """D32: after G2 every receipt number is a U or I32 (schema), so integral floats become int once."""
    if isinstance(value, dict):
        for k, v in value.items():
            if type(v) is float: value[k] = int(v)
            else: _normalize_integrals(v)
    elif isinstance(value, list):
        for i, v in enumerate(value):
            if type(v) is float: value[i] = int(v)
            else: _normalize_integrals(v)


def _ref(items, index):
    """A reference that resolves, else None (D16: the caller reports its own check's code)."""
    return items[int(index)] if type(index) in (int, float) and int(index) == index and 0 <= index < len(items) else None


def _g5_native(body):
    """G5 class 1 (C3:304; D3): native schedule/origin. Run ids and the execution-order bijection
    are G3 (D1). Class-1 ATTEMPT defects win; native WORK predicates (including computation faults
    inside a WORK equation) are collected and reported only at the end of class 1 (D3, settled
    reading 3)."""
    runs = [c["run"] for c in body["cases"] if c.get("run") is not None]
    runs.sort(key=lambda r: r["id"])
    native_work = []
    fail = lambda ok, code="ATTEMPT_MISMATCH": native_work.append(ok) if code == "WORK_MISMATCH" else _need(ok, "G5", code)
    wf = native_work.append
    _g5_native_checks(body, runs, fail, wf)
    for ok in native_work:
        _need(ok, "G5", "WORK_MISMATCH")


def _g5_native_checks(body, runs, fail, wf):
    # Kernel scope (checkpoint A, D8; C1:66-68): a work_accounting stop or reason anywhere in a
    # Run, a build or a group refusal is outside the emitted domain (adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`).
    for item in runs + body["builds"] + [g["preparation"] for g in body["groups"]]:
        fail(not any(o.get("tag") == "work_accounting" for o in _objects(item)))
    current = 0
    for call_id, call in enumerate(body["calls"]):
        fail(call["id"] == call_id and call["invocation_before"] == current, "WORK_MISMATCH")
        fail(len(call["run_refs"]) == len(call["source_refs"]) == len(call["owner_refs"]))
        for position, (ri, si, oi) in enumerate(zip(call["run_refs"], call["source_refs"], call["owner_refs"])):
            run = _at(runs, ri, code="ATTEMPT_MISMATCH")
            fail(run["origin"] == {"call": call_id, "position": position, "group": run["origin"]["group"], "source_ref": si, "owner_ref": oi})
            case = _at(body["cases"], oi["index"], code="ATTEMPT_MISMATCH")
            fail(oi["kind"] == "case" and case.get("run") == run and case.get("source_ref") == si)
            source = _at(body["sources"], si, code="ATTEMPT_MISMATCH")
            fail(source["owner"]["case_index"] == oi["index"])
            fail(run["invocation_before"] == current, "WORK_MISMATCH")
            records, attempts = run["records"], run["attempts"]
            fail(len(records) <= 4 and [r["index"] for r in records] == list(range(len(records))))
            # C1 s1 items 1-5 / G5 "actual logical/native schedule": the native ladder
            # always opens with a fresh p128 candidate in record 0 (adaptive.rs `run_schedule_inner`);
            # later slots advance only by the stated failure/reuse rules.
            fail(len(attempts) <= 3 and (not records) == (not attempts))
            if attempts: fail(attempts[0]["precision"] == 128 and attempts[0]["candidate_record"] == 0 and attempts[0]["origin"] == {"kind": "fresh"})
            fail([r["precision"] for r in records] == sorted(set(r["precision"] for r in records)))
            _g5_schedule(run, records, attempts, fail, body)
            _g5_cache(body, run, records, fail, wf)
            layout = source["layout"]
            for item in records + attempts:
                reason = item["outcome"].get("reason") or {}
                if reason.get("space") == "attempt" and "quantity" in reason:
                    # D33 (verify.rs `verify_state`): the verification estimate exists only for force/moment rows.
                    fail(reason.get("tag") != "verification_estimate" or reason["kind"] in ("force", "moment"))
                    # D5d/D28 (C2:22-24, :54; C1:114; adaptive.rs `rejection_reason`, `run_schedule_inner`): stop_rule,
                    # verification_estimate, charge and publication_enclosure quantities resolve to a
                    # layout row of the Run's source with the same body and kind.
                    fail(any(row["quantity"] == reason["quantity"] and row["body"] == reason["body"] and row["kind"] == reason["kind"] for row in layout))
            amounts = []
            for r in records:
                w = r["work"]
                if r["role"] == "candidate":
                    # D5a (C1:105; adaptive.rs `solve_precision`, `verify_precision`): only verification passes write these.
                    fail(r["verification"] is None and r["verification_shared_build_ref"] is None and w["verification_lme"] == 0)
                stop = _stop_of(r["outcome"].get("reason")) if r["outcome"]["kind"] == "failed" else None
                if r["role"] == "verification" and stop is not None and stop.get("tag") in ESCALATING_STOPS:
                    # D5b (adaptive.rs `run_schedule_inner`, `terminal`): an escalating stop is a verification *solve*
                    # failure; the verification pass never ran.
                    # D21 (widened): pass evidence is verification_lme > 0, a verification shared build
                    # (adaptive.rs `verify_precision`) or a verification summary (set only by verify_precision, 4333).
                    fail(w["verification_lme"] == 0 and r["verification_shared_build_ref"] is None and r["verification"] is None)
                fail(r["residual_basis"] == (1024 if r["precision"] == 1024 else r["precision"] + 64))
                fail(r["storage"]["limbs_per_entry"] == (4 if r["precision"] <= 256 else 8 if r["precision"] == 512 else 16))
                own = int(w["wide_lme"]) + int(w["exact_sum_lme"])
                fail(own == w["own_lme"] == sum(map(int, w["own_stages"].values())), "WORK_MISMATCH")
                fail(w["stop_rule_lme"] == w["own_stages"]["stop_rule"], "WORK_MISMATCH")
                fail(w["verification_lme"] == sum(w["own_stages"][k] for k in ("scale", "estimate", "charge", "bound", "shift")), "WORK_MISMATCH")
                fail(w["stop_rule_lme"] + w["verification_lme"] <= own, "WORK_MISMATCH")
                fail(sum(map(int, w["shared_stages"].values())) == w["shared_lme"] + w["verification_shared_lme"], "WORK_MISMATCH")
                amounts.append((own + w["shared_lme"] + w["verification_shared_lme"], own + (w["shared_lme"] if w["shared_built_here"] else 0) + (w["verification_shared_lme"] if w["verification_shared_built_here"] else 0)))
                shared_stages = {k: 0 for k in w["shared_stages"]}
                for field, part, built, cost in [("shared_build_ref", "shared", w["shared_built_here"], w["shared_lme"]), ("verification_shared_build_ref", "verification_shared", w["verification_shared_built_here"], w["verification_shared_lme"])]:
                    bi = r[field]
                    if bi is None: fail(not built and cost == 0, "WORK_MISMATCH"); continue
                    build = _ref(body["builds"], bi)
                    if build is None:
                        wf(False)  # D16: dangling build reference (WORK, C1 build provenance)
                        continue
                    fail(build["work"] == cost and build["group"] == run["origin"]["group"], "WORK_MISMATCH")
                    fail(build["slot"] == ("s" if part == "shared" else "v") + str(r["precision"]), "WORK_MISMATCH")
                    for key, count in build["stages"].items(): shared_stages[key] += count
                    if built: fail(build["origin"] == {"call": call_id, "run": ri, "physical_record": r["index"], "phase": part}, "WORK_MISMATCH")
                    else: fail(build["origin"]["run"] < ri or (build["origin"]["run"] == ri and build["origin"]["physical_record"] < r["index"]), "WORK_MISMATCH")
                fail(shared_stages == w["shared_stages"], "WORK_MISMATCH")
            fragments = []
            for ai, attempt in enumerate(attempts):
                cr = _at(records, attempt["candidate_record"], code="ATTEMPT_MISMATCH")
                fail(cr["precision"] == attempt["precision"] and cr["role"] in ("candidate", "verification_then_candidate") and cr["outcome"] == attempt["outcome"])
                if attempt["origin"]["kind"] == "reused_verification":
                    prior = _at(attempts[:ai], attempt["origin"]["attempt"], code="ATTEMPT_MISMATCH")
                    fail(prior["verification"] is not None and prior["verification"]["record"] == cr["index"] and cr["role"] == "verification_then_candidate")
                else: fail(cr["role"] == "candidate")
                verification = attempt["verification"]
                if verification is not None:
                    vr = _at(records, verification["record"], code="ATTEMPT_MISMATCH")
                    fail(vr["precision"] == verification["precision"] == 2 * attempt["precision"] and vr["index"] > cr["index"] and vr["role"] in ("verification", "verification_then_candidate"))
                    if vr["role"] == "verification_then_candidate":
                        fail(verification["phase"] == "completed" and verification["reason"] is None)
                    else:
                        fail((verification["phase"] == "failed") == (vr["outcome"]["kind"] == "failed"))
                charge = debit = 0
                for fragment in attempt["charges"]:
                    pair = (fragment["record"], fragment["part"]); fail(pair not in fragments); fragments.append(pair)
                    fr = _at(records, fragment["record"], code="ATTEMPT_MISMATCH"); w = fr["work"]
                    if fragment["part"] == "candidate_stop":
                        fail(fragment["record"] == cr["index"])
                        charge += w["stop_rule_lme"]; debit += w["stop_rule_lme"]
                    else:
                        fail(fragment["record"] == cr["index"] and cr["role"] == "candidate" or verification is not None and fragment["record"] == verification["record"])
                        charge += amounts[int(fr["index"])][0] - w["stop_rule_lme"]
                        debit += amounts[int(fr["index"])][1] - w["stop_rule_lme"]
                fail(charge == attempt["case_charge"] and debit == attempt["invocation_increment"], "WORK_MISMATCH")
            required = [(r["index"], "solve_and_verification") for r in records] + [(r["index"], "candidate_stop") for r in records if r["role"] != "verification"]
            fail(sorted(fragments) == sorted(required))
            for r in records:
                if r["role"] == "verification": fail(r["work"]["stop_rule_lme"] == 0, "WORK_MISMATCH")
            charge = sum(x[0] for x in amounts); debit = sum(x[1] for x in amounts)
            fail(charge == run["case_charge"] == sum(a["case_charge"] for a in attempts), "WORK_MISMATCH")
            fail(debit == run["invocation_increment"] == sum(a["invocation_increment"] for a in attempts), "WORK_MISMATCH")
            if run["invocation_before"] >= body["work"]["invocation_limit"]:
                fail(not attempts and run["origin"]["group"] is None
                     and run["kernel_terminal"]["reason"] == {"space": "unresolved", "tag": "budget", "scope": "invocation"})
            budget = run["kernel_terminal"]["reason"] if run["kernel_terminal"]["kind"] == "unresolved" else None
            if isinstance(budget, dict) and budget.get("tag") == "budget" and attempts:
                # N17: the budget test checks case room first (adaptive.rs `StageGuard::test`).
                if budget.get("scope") == "case":
                    fail(charge > body["work"]["case_limit"], "WORK_MISMATCH")
                else:
                    fail(charge <= body["work"]["case_limit"] and current + debit > body["work"]["invocation_limit"], "WORK_MISMATCH")
            current += debit
            fail(current == run["invocation_after"] and current <= SAFE, "WORK_MISMATCH")
            if run["kernel_terminal"]["kind"] == "selected":
                fail(bool(attempts) and attempts[-1]["outcome"]["kind"] == "accepted" and attempts[-1]["verification"] is not None)
                vr = records[int(attempts[-1]["verification"]["record"])]
                fail(vr["outcome"]["kind"] == "verified" and attempts[-1]["verification"]["reason"] is None)
                fail(charge <= body["work"]["case_limit"] and current <= body["work"]["invocation_limit"], "WORK_MISMATCH")
                if case["status"] == "selected":
                    s = case["selection"]; last = attempts[-1]
                    fail(s["precision"] == last["precision"] and s["verification_precision"] == last["verification"]["precision"])
                    cr = records[int(last["candidate_record"])]
                    fail(all(s[k] == cr[k] for k in ["pivot_margin_min", "rcond", "residual_worst", "corrections"]))
                    fail(vr["verification"] is not None)
                    fail(s["resolution_scale"] == vr["verification"]["resolution"] and s["theta"] == vr["verification"]["theta"])
                    fail(s["certified_bound"] == [v for v in vr["verification"]["bound"] if v["value"] is not None])
        fail(call["invocation_after"] == current, "WORK_MISMATCH")
    fail([r for call in body["calls"] for r in call["run_refs"]] == list(range(len(runs))))
    for group in body["groups"]:
        # D5e (C2:119, :135): a group's call exists; its sources are unique and listed in that call.
        call = _ref(body["calls"], group["call"])
        fail(call is not None and len(set(group["source_refs"])) == len(group["source_refs"]) and all(si in call["source_refs"] for si in group["source_refs"]))
    fail(body["work"]["charged"] == current, "WORK_MISMATCH")
    for call_id, call in enumerate(body["calls"]):
        # C2:143 call-local groups at first equality of full stiffness bytes, first-seen order;
        # an idle (group-null) run never formed a group (adaptive.rs `solve_cases_projected`).
        call_groups = [g for g in body["groups"] if g["call"] == call_id]
        order, members, of_run = [], {}, []
        for ri, si in zip(call["run_refs"], call["source_refs"]):
            run = runs[int(ri)]
            if run["origin"]["group"] is None:
                continue
            key = body["sources"][int(si)]["stiffness_sha256"]
            if key not in members:
                order.append(key); members[key] = []
            members[key].append(si); of_run.append((run, order.index(key)))
        fail([(g["stiffness_sha256"], g["source_refs"]) for g in call_groups] == [(k, members[k]) for k in order])
        for run, gi in of_run:
            fail(int(run["origin"]["group"]) == call_groups[gi]["id"])
            if call_groups[gi]["preparation"]["kind"] == "refused":
                # N11: a refused group preparation returns before any attempt.
                fail(not run["attempts"] and run["kernel_terminal"] == {"kind": "refused", "reason": call_groups[gi]["preparation"]["reason"]})
    for i, group in enumerate(body["groups"]):
        fail(group["id"] == i and bool(group["source_refs"]) and group["first_source_ref"] == group["source_refs"][0])
        for si in group["source_refs"]: fail(_at(body["sources"], si)["stiffness_sha256"] == group["stiffness_sha256"])
    for i, build in enumerate(body["builds"]):
        fail(build["id"] == i and build["work"] == sum(map(int, build["stages"].values())), "WORK_MISMATCH")
    for run in runs:
        for snapshot in [run["cache_before"], run["cache_after"]]:
            slots = [x["slot"] for x in snapshot]
            order = ["s128", "s256", "s512", "s1024", "v256", "v512", "v1024"]
            fail(slots == sorted(set(slots), key=order.index), "WORK_MISMATCH")
            for entry in snapshot:
                b = _ref(body["builds"], entry["build"])
                if b is None:
                    wf(False)
                    continue
                fail(b["slot"] == entry["slot"] and b["state"] != "budget_failure" and b["group"] == run["origin"]["group"] and b["origin"]["run"] <= run["id"], "WORK_MISMATCH")


def _g5_coverage(a, case, source, fail):
    """I57 s3/s4 G5: proof-owned summary coverage binding and stage implications.

    Null means no complete vector was retained; it is never all-false coverage.
    A completed certificate, passed G5a or Ready product requires the complete
    vector; a failed certificate may carry null. A complete vector requires the
    attempt's own source/Run, a selected native Run, both lanes completed in
    order, completed proof stages through aliases and an entered certificate.
    Complete coverage never implies certificate success.
    """
    proof = a["proof"]
    if proof is None:
        return
    coverage, stages, checks = proof["summary_coverage"], a["stages"], proof["checks"]
    if (a["result"]["kind"] == "ready" or stages["certificate"] == "completed" or checks["certificate"]["kind"] == "passed"
            or stages["g5a"] == "completed" or checks["g5a"]["kind"] == "passed"):
        fail(coverage is not None)
    if coverage is None:
        return
    run = case.get("run")
    fail(source is not None and a["run_ref"] is not None and run is not None and run["id"] == a["run_ref"]
         and case["source_ref"] == a["source_ref"] and run["origin"]["source_ref"] == a["source_ref"]
         and run["origin"]["owner_ref"] == a["owner_ref"] and run["kernel_terminal"]["kind"] == "selected")
    fail([lane["law"] for lane in proof["lanes"]] == ["admitted_k", "annular_source"] and all(lane["state"] == "completed" for lane in proof["lanes"]))
    fail(all(stages[k] == "completed" for k in ("proof_start", "projection", "maxima", "values", "aliases")))
    fail(stages["certificate"] in ("completed", "failed") and checks["certificate"]["kind"] in ("passed", "failed"))


def _conversion_kind_ok(outcome):
    """C3:182-185: Normal holds a normal value or exact +/-0; Subnormal a nonzero subnormal."""
    if outcome["kind"] == "normal":
        value = from_bits(outcome["value"])
        return value == 0 or abs(value) >= 2.0 ** -1022
    if outcome["kind"] == "subnormal":
        return 0 < abs(from_bits(outcome["value"])) < 2.0 ** -1022
    return True


STAGE_ORDER = ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"]


def _g5_stages(a, case, fail):
    """Checklist P2, P6, P11 (C3:196-201, 253-257; retained_receipt.rs `PreparedTrace::enter` through `PreparedTrace::checked`, `project`)."""
    st, proof = a["stages"], a["proof"]
    pipeline = [st[k] for k in STAGE_ORDER[:8]]
    seen_end = False
    for state in pipeline:
        if seen_end:
            fail(state == "not_entered")
        elif state != "completed":
            seen_end = True
    obs, g5a = st["observables"], st["g5a"]
    fail((obs == "not_entered") == (g5a == "not_entered"))
    if obs != "not_entered":
        fail(st["certificate"] in ("completed", "failed"))
    run = case.get("run")
    if st["native"] == "not_entered":
        fail(a["run_ref"] is None)
    else:
        fail(a["run_ref"] is not None and run is not None and (st["native"] == "completed") == (run["kernel_terminal"]["kind"] == "selected"))
    fail((st["preparation"] == "completed") == (a["source_ref"] is not None) or st["preparation"] == "not_entered" and a["source_ref"] is None)
    if proof is None:
        fail(st["proof_start"] == "not_entered")
        return
    fail(st["proof_start"] != "not_entered")
    checks = proof["checks"]
    for stage, check in (("certificate", "certificate"), ("observables", "observables"), ("g5a", "g5a")):
        fail({"not_entered": "not_entered", "completed": "passed", "failed": "failed"}[st[stage]] == checks[check]["kind"])
    completion = proof["completion"]["kind"]
    if st["values"] == "failed":
        fail(completion == "separate_failure")
    elif st["certificate"] != "not_entered" or "failed" in (st["maxima"], st["aliases"]) or (st["aliases"] == "completed"):
        fail(completion == "merged")
    elif st["projection"] != "completed":
        fail(completion == "not_entered")


def _g5_typed(a, fail):
    """Checklist P9 (C3:279-287): failed checks carry their own PublicFailure wrapper, and the
    attempt result error matches the first failing stage (retained_product.rs `PreparedCase::project_candidate`, S06:30-45)."""
    proof, st, result = a["proof"], a["stages"], a["result"]
    if proof is not None:
        for check, kinds in (("certificate", ("proof",)), ("observables", ("observable",)), ("g5a", ("g5a",))):
            c = proof["checks"][check]
            if c["kind"] == "failed":
                fail(c["error"]["kind"] in kinds)
    if result["kind"] != "unavailable":
        return
    # D37 (D35 widened; retained_product.rs `ProductCapture::prepare_owned_case` through `PreparedCase::solve_native`, `PreparedCase::project_candidate`; S06 s1): the error kind and the
    # whole stage record agree in both directions. ERROR_STAGE_RECORDS lists every record the native
    # sequence can leave for each kind (stage completed <=> its check passed is checked in class 2).
    fail(tuple(st[k] for k in STAGE_ORDER) in ERROR_STAGE_RECORDS.get(result["error"]["kind"], ()))


def _error_stage_records():
    C, F, N = "completed", "failed", "not_entered"
    done = lambda n, rest: tuple([C] * n + list(rest) + [N] * (10 - n - len(rest)))
    return {
        "preparation": {done(0, [F])},                                   # prepare_owned_case fails (3140, 3257)
        "native": {done(1, [F])},                                        # solve_native, nonselected Run (3276, 3290)
        "capture": {done(1, [F]),                                        # solve_native before any Run (3279-3286)
                    done(2, []),                                         # project_candidate before ProofStart (3461-3466)
                    done(8, []),                                         # after a passed certificate (3517-3525)
                    done(10, [])},                                       # the commit (3546-3549)
        "proof": {done(2, [F]), done(3, [F])}                            # begin_prepared_product, project (3469, 3473)
                 | {done(7, [F, x, y]) for x, y in ((N, N), (C, C), (C, F), (F, C), (F, F))},  # certify_final (3493-3515)
        "values": {done(5, [F])},                                        # complete_maxima (3480-3481)
        "abandoned": {done(4, [F]), done(6, [F]), done(7, [])},           # maxima, aliases, bind_rows_view (3476, 3484-3491)
        "numeric": {done(10, [])},                                       # both checks passed, pass false (3538-3543)
        "observable": {done(8, [F, C]), done(8, [F, F])},                # observables failed (3528-3543)
        "g5a": {done(9, [F])},                                           # observables passed, G5a failed (3528-3543)
    }


ERROR_STAGE_RECORDS = _error_stage_records()


RCOND_LABEL = "sensitivity to matrix-entry perturbation, not to authored parameters"


def _g5_ordinary(body, cases, diags, quality):
    """G5 class 2 ordinary pass (C3:304; D3, D6): checklist O2-O5 (C2:149-161; S06:51), plus the
    ordinary-list, not_attempted, report, not_required, selected-quality and rcond checks that
    previously ran after the product WORK list (RV79-S2)."""
    fail = lambda ok: _need(ok, "G5", "ATTEMPT_MISMATCH")
    by_id = {d["id"]: d for d in diags}
    ids = set(by_id)
    for i, c in enumerate(cases):
        o = body["ordinary_attempts"][i]
        refs = o["diagnostic_refs"]
        # D6a (C1:100, C1:148, C2:166): untyped refs are unique and resolve.
        fail(len(set(refs)) == len(refs) and all(x in by_id for x in refs))
        # F5 (D-U6-7; decision 2, A2, RR:8821, amending checkpoint A's D6a, RR:8117): the list is
        # exactly the diagnostics whose affected_refs name the case, once each, in envelope order,
        # excluding RETAINED_PRECISION_* (a T1 (a)-omitted disclosure is absent from the envelope).
        name = c["basis_ref"]["ref_id"]
        # A non-array affected_refs names no case (S1, RV90: parity with Rust list() and TS Array.isArray).
        fail(refs == [d["id"] for d in diags if isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]
                      and not str(d.get("code")).startswith("RETAINED_PRECISION_")])
        fail(o["initial"]["kind"] != "not_attempted" if c["status"] in ("selected", "not_required") else True)
        if o["initial"]["kind"] == "report":
            fail(o["initial"]["report_diagnostic_ref"] in by_id and o["initial"]["outcome"] == quality[i]["solve_quality"])
        if c["status"] == "not_required":
            fail(c["product_attempt_ref"] is None and quality[i]["solve_quality"] == "checks_passed")
        if c["status"] == "selected":
            # D6b (C1:101; C2:153, :164; source_receipt.rs `OrdinaryAttempt::wire`): only an attempted trigger selects.
            fail(quality[i]["solve_quality"] in ("sensitive", "unresolved", "failed"))
            fail(c["selection"]["rcond_label"] == RCOND_LABEL)
        listed = set(refs)
        cid = c["basis_ref"]["ref_id"]
        resolves = lambda ref: ref is None or (ref in listed and ref in by_id and cid in (by_id[ref].get("affected_refs") or []))
        if o["initial"]["kind"] == "report":
            fail(resolves(o["initial"]["report_diagnostic_ref"]))
        initial, w2 = o["initial"], o["w2"]
        if initial["kind"] == "structural_failure":
            fail(resolves(initial["diagnostic_ref"]))
        # W2 runs only after an actual Formation/Structural failure and never erases it.
        fail((w2["kind"] == "not_triggered") or initial["kind"] in ("structural_failure", "formation_failure"))
        if w2["kind"] != "not_triggered":
            # D6c (C2:157-158): the trigger is the preserved initial failure's kind and error.
            fail(w2["trigger"]["tag"] == ("formation" if initial["kind"] == "formation_failure" else "evaluation") and w2["trigger"]["error"] == initial.get("error"))
        if w2["kind"] == "published":
            fail(w2["report_diagnostic_ref"] is not None and resolves(w2["report_diagnostic_ref"]) and int(w2["force_scale_exponent"]) != 0)
        if w2["kind"] == "failed":
            fail(w2["diagnostic_ref"] is not None and resolves(w2["diagnostic_ref"]))
        finding = o["formation"]["load_row_finding"]
        fail(resolves(o["formation"]["d5_diagnostic_ref"]) and (finding is None or resolves(finding["diagnostic_ref"])))
        fail(resolves(o["legacy_source"]["diagnostic_ref"]))
        if o["legacy_source"]["work_ref"] is not None:
            # D6d (C2:166): a reference check, so ATTEMPT.
            w = _at(body.get("legacy_source_work") or [], o["legacy_source"]["work_ref"], code="ATTEMPT_MISMATCH")
            fail(w["case_index"] == i)
        decline = c.get("source_decline")
        if decline is not None:
            owner = decline["input_owner"]
            mb = _at(body["material_bases"], owner["material_basis_ref"], code="ATTEMPT_MISMATCH")
            fail(c["status"] == "unavailable" and c.get("source_ref") is None and c.get("run") is None
                 and owner["case_index"] == i and owner["case_id"] == c["basis_ref"]["ref_id"] and i in mb["case_indices"])


def _g5_products(body, rows_by_case):
    fail = lambda ok: _need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    work_checks = []
    wf = work_checks.append
    for case in body["cases"]:
        cause = (case.get("reason") or {}).get("cause") or {}
        if case["status"] == "selected":
            fail(case["product_attempt_ref"] is not None)  # D20 (C3:165): class 2, after the ordinary pass (D17)
        if case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure":
            # D4c (S06 s1; C3:165): the case's own attempt, resolved once.
            fail(case["product_attempt_ref"] is not None and case["product_attempt_ref"] == cause["product_attempt_ref"])
    for ai, a in enumerate(body["product_attempts"]):
        case = _at(body["cases"], a["owner_ref"]["index"])
        fail(a["id"] == ai and case["product_attempt_ref"] == ai and a["ordinary_attempt_ref"] == case["ordinary"]["attempt_ref"])
        cause = (case.get("reason") or {}).get("cause") or {}
        if a["result"]["kind"] == "unavailable":
            # D19 (S06 s1): an actual unavailable C3 attempt is carried by prepared_product_failure.
            fail(case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure" and cause.get("product_attempt_ref") == ai)
        else:
            # D19: a Ready attempt is selected, or unavailable only by a later receipt failure.
            fail(case["status"] == "selected" or (case["status"] == "unavailable" and cause.get("kind") == "receipt_failure"))
        ordinary = _at(body["ordinary_attempts"], a["ordinary_attempt_ref"])
        fail(a["material_basis_ref"] == ordinary["material_basis_ref"])  # D4b (C3:165)
        if a["run_ref"] is not None: fail(case.get("run") is not None and case["run"]["id"] == a["run_ref"] and a["source_ref"] == case["source_ref"])
        else: fail(case.get("run") is None)  # D4e (C3:167): run_ref null iff no native call
        source = None if a["source_ref"] is None else _at(body["sources"], a["source_ref"])
        if source is not None:
            fail(source["owner"]["case_index"] == a["owner_ref"]["index"] and source["material_basis_ref"] == a["material_basis_ref"])
            fail(source["preparation"] is not None and source["preparation"]["attempt_ref"] == ai)  # D4a (C3:146-148)
        pm = a["preparation"]["members"]
        old, new = a["operational"]["old"], a["operational"]["new"]
        fail(len(new) <= len(pm) <= len(old))
        fail([x["member"] for x in pm] == [x["member"] for x in old[:len(pm)]])
        fail([x["member"] for x in new] == [x["member"] for x in pm[:len(new)]])
        if a["operational"]["old_coverage"] == "captured_prefix": fail(not pm and not new and a["source_ref"] is None and a["run_ref"] is None and a["result"]["kind"] == "unavailable")
        props = [(p, side) for p in ("area", "second_moment", "polar_moment", "section_modulus") for side in ("lo", "hi")] + [("radius", "exact")]
        for j, m in enumerate(pm):
            ready = m["result"]["kind"] == "prepared"
            if not ready: fail(j == len(pm) - 1 and j >= len(new))
            conversions = m["conversions"]
            fail([(x["property"], x["endpoint"]) for x in conversions] == props[:len(conversions)])
            for conversion in conversions:
                fail(_conversion_kind_ok(conversion["outcome"]))
            if _count(m["work"]["conversions"]) is not None: wf(_count(m["work"]["conversions"]) == len(conversions))
            if ready:
                fail(len(conversions) == 9)
                for k, conversion in enumerate(conversions):
                    outcome = conversion["outcome"]
                    idx = k // 2 if k < 8 else 4
                    fail(outcome["kind"] == "normal" and outcome["value"] == m["result"]["section"][idx] and from_bits(outcome["value"]) >= 2.0 ** -1022)
        proof = a["proof"]
        if proof is not None:
            lanes = proof["lanes"]
            fail([x["law"] for x in lanes] == ["admitted_k", "annular_source"][:len(lanes)])
            fail(len(lanes) <= 2)
            for i, lane in enumerate(lanes):
                fail((lane["error"] is None) == (lane["state"] == "completed"))
                if lane["state"] == "failed": fail(i == len(lanes) - 1)
                calls = _count(lane["work"]["correction"]["calls"])
                if calls is not None: wf(calls <= 1)
                wf(lane["work"]["data_capacity"] == lane["work"]["view"]["data_capacity"])
            if a["stages"]["projection"] != "not_entered": fail(len(lanes) == 2 and all(l["state"] == "completed" for l in lanes))
            outcomes = proof["projection_outcomes"]
            wf(_count(proof["projection_conversions"]) in (None, len(outcomes)))
            fail([x["row_index"] for x in outcomes] == sorted(set(x["row_index"] for x in outcomes)))
            rows = rows_by_case[case["basis_ref"]["ref_id"]]
            for x in outcomes:
                row = _at(rows, x["row_index"])
                outcome = x["outcome"]
                fail(_conversion_kind_ok(outcome))
                if outcome["kind"] == "normal": fail(from_bits(outcome["value"]) == 0 or abs(from_bits(outcome["value"])) >= 2.0 ** -1022)
                if outcome["kind"] == "subnormal": fail(0 < abs(from_bits(outcome["value"])) < 2.0 ** -1022)
                if a["result"]["kind"] == "ready":
                    fail(outcome["kind"] != "overflow")
                    value = 0.0 if outcome["kind"] == "underflow" else from_bits(outcome["value"])
                    fail(bits(float(row["value"])) == bits(value if value != 0 else 0.0))
        _g5_coverage(a, case, source, fail)
        _g5_stages(a, case, fail)
        for ok in _accounting_rules(a): wf(ok)
        if a["result"]["kind"] == "ready":
            fail(source is not None and a["run_ref"] is not None and case["run"]["kernel_terminal"]["kind"] == "selected")
            fail(source["preparation"] is not None and source["preparation"]["attempt_ref"] == ai)
            fail(all(v == "completed" for v in a["stages"].values()) and len(pm) == len(old) == len(new) and all(m["result"]["kind"] == "prepared" for m in pm))
            fail(a["operational"]["old_coverage"] == "complete" and proof is not None and all(v["kind"] == "passed" for v in proof["checks"].values()))
            fail(all(m["result"]["kind"] == "ready" for m in new))
            wf(_exact_work(proof) and all(_exact_work(m["work"]) for m in pm) and a["adapter"]["fault"] is None and not a["g5a_work"]["lost"] and not a["overlay_work"]["lost"])
            expected = [i for i,r in enumerate(rows_by_case[case["basis_ref"]["ref_id"]]) if r["kind"] not in NONQUANTITY | {"support_reaction_force_magnitude_v2","support_reaction_moment_magnitude_v2","pipe_elastic_normal_stress_maximum_v2"}]
            fail([x["row_index"] for x in proof["projection_outcomes"]] == expected)
        if case["status"] == "selected": fail(a["result"]["kind"] == "ready")
        if case["status"] == "unavailable" and case["reason"]["cause"].get("kind") == "prepared_product_failure":
            fail(case["reason"]["cause"]["product_attempt_ref"] == ai and a["result"]["kind"] == "unavailable")
            error = a["result"]["error"]["kind"]
            run = case.get("run")
            if error == "preparation":
                # D4d (S06:37): no native Run was entered, and the preparation stage failed.
                fail(run is None and a["stages"]["preparation"] == "failed")
            if error == "native":
                # D4d (S06:38): a nonselected Run, and the same Run.
                fail(run is not None and run["kernel_terminal"]["kind"] != "selected" and a["result"]["error"]["run_ref"] == run["id"] == a["run_ref"])
            if error == "preparation" or error == "capture" and run is None: expected = ("source_unavailable", "preparation")
            elif error == "native" or error == "capture" and run["kernel_terminal"]["kind"] != "selected":
                fail(run is not None and run["kernel_terminal"]["kind"] in ("unresolved", "refused"))
                expected = ("kernel_" + run["kernel_terminal"]["kind"], "kernel")
            else:
                fail(run is not None and run["kernel_terminal"]["kind"] == "selected")
                expected = ("facade_certificate", "facade")
            fail((case["reason"]["code"], case["reason"]["phase"]) == expected)
    for a in body["product_attempts"]:
        _g5_typed(a, fail)
    for ok in work_checks:
        _need(ok, "G5", "WORK_MISMATCH")


COMPONENTS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
NONQUANTITY_KINDS = {"linear_solver_mode_basis", "sparse_live_path_dense_parity_relative_delta", "modulus_basis_record", "combination_modulus_basis_record"}
# C3:178-181: hull projection covers quantity rows except observed ancillary rows, support norms and maxima.
HULL_EXCLUDED = NONQUANTITY_KINDS | {"support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2", "pipe_elastic_normal_stress_maximum_v2"}
NONQUANTITY = {"linear_solver_mode_basis", "sparse_live_path_dense_parity_relative_delta", "modulus_basis_record", "combination_modulus_basis_record"}
INPUT_KINDS = {"pipe_lame_hoop_stress_v2", "pipe_lame_radial_stress_v2", "pipe_section_pressure_hoop_stress", "pipe_section_pressure_longitudinal_stress", "constant_effort_support_applied_load", "component_user_stress_multiplier_review", "component_user_stiffness_macro_element_review", "constant_effort_user_input_review", "spring_hanger_user_input_review", "expansion_joint_pressure_thrust_load_review"}
FORCE = {"element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "reaction_resultant", "support_reaction_force_magnitude_v2"}
MOMENT = {"element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z", "support_reaction_moment_magnitude_v2"}
STRESS = {"element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z", "element_local_torsional_shear_stress", "pipe_axial_membrane_stress_v2", "pipe_elastic_normal_stress_maximum_v2", "component_equal_factor_intensified_bending_stress_v1", "open_formula_stress_summary"}


def _normalized(row):
    y, unit = float(row["value"]), row["unit"]
    if unit == "mm": return y / 1000.0
    if unit in ("kN", "kN*m"): return y * 1000.0
    if unit == "MPa": return y * 1000000.0
    return y


def _row_kind(row):
    k, u = row["kind"], row["unit"]
    if k in NONQUANTITY: return "non_quantity"
    if k in INPUT_KINDS: return "input_derived"
    if k in {"global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z", "displacement_magnitude"} and u in ("m", "mm"): return "translation"
    if k in {"global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"} and u == "rad": return "rotation"
    if k in FORCE and u in ("N", "kN"): return "force"
    if k in MOMENT and u in ("N*m", "kN*m"): return "moment"
    if k in ("support_reaction_component_v2", "pipe_wall_endpoint_action_v2"):
        if u in ("N", "kN"): return "force"
        if u in ("N*m", "kN*m"): return "moment"
    if k in STRESS and u in ("Pa", "MPa"): return "stress"
    return "not_covered"


def _row_body(row, source):
    entity = row["entity_ref"]; maps = source["id_maps"]
    node = next((x["kernel_node"] for x in maps["nodes"] if x["id"] == entity), None)
    member = next((x for x in maps["members"] if x["id"] == entity), None)
    support = next((x for x in maps["support_ids"] if x["id"] == entity), None)
    if member is not None: node = member["node_i"]
    if support is not None: node = support["node"]
    if node is None: return None, member
    body = next((b["body"] for b in source["body_membership"] if node in b["nodes"]), None)
    return body, member


def _extent(nodes):
    d = [max(p[j] for p in nodes) - min(p[j] for p in nodes) for j in range(3)]
    return math.sqrt(((d[0] * d[0]) + (d[1] * d[1])) + (d[2] * d[2]))


def _e_hat(e, length):
    """verify.rs `e_hat` e_hat: a single-node body (L=0) keeps E."""
    if length == 0:
        return list(e)
    fo, mo = e
    return [max(fo, mo / length), max(mo, length * fo)]


def _phi_512(e_hat):
    """verify.rs `phi_512`: Phi = fl-up(2^-438 * e_hat), nearest then next up when below."""
    nearest = e_hat * float.fromhex("0x1p-438")
    return math.nextafter(nearest, math.inf) if nearest * float.fromhex("0x1p+438") < e_hat else nearest


def _canonical_layout(source, need):
    """Full canonical layout rebuilt from the bound source maps (recover.rs `layout` order).

    Constraints must be unique and prescribe exact +0 in this C3 scope (D=false).
    """
    maps = source["id_maps"]
    def body_of(n):
        hits = [b["body"] for b in source["body_membership"] if n in b["nodes"]]
        need(len(hits) == 1)
        return hits[0]
    fixed = set()
    for c in source["constraints"]:
        key = (c["dof"]["node"], COMPONENTS.index(c["dof"]["component"]))
        need(key not in fixed and c["value"] == "0000000000000000")
        fixed.add(key)
    layout = []
    def add(q, kind, n, inp=False):
        layout.append({"index": len(layout), "quantity": q, "kind": kind, "body": body_of(n), "input_derived": inp})
    count = len(maps["nodes"])
    for n in range(count):
        for j, c in enumerate(COMPONENTS):
            add({"tag": "displacement", "dof": {"node": n, "component": c}}, "translation" if j < 3 else "rotation", n, (n, j) in fixed)
    for n in range(count):
        add({"tag": "displacement_magnitude", "node": n}, "translation", n)
    for m in maps["members"]:
        for end in ("i", "j"):
            for j, c in enumerate(COMPONENTS):
                add({"tag": "end_action", "member": m["kernel_member"], "end": end, "component": c}, "force" if j < 3 else "moment", m["node_i"])
    for st in source["stations"]:
        m = maps["members"][int(st["member"])]
        for j, c in enumerate(COMPONENTS):
            add({"tag": "station_action", "station": st["id"], "component": c}, "force" if j < 3 else "moment", m["node_i"])
    for spring in maps["springs"]:
        add({"tag": "spring_action", "spring": spring["kernel_spring"], "component": spring["component"]},
            "force" if COMPONENTS.index(spring["component"]) < 3 else "moment", spring["node"])
    for c in source["constraints"]:
        add({"tag": "reaction", "dof": c["dof"]}, "force" if COMPONENTS.index(c["dof"]["component"]) < 3 else "moment", c["dof"]["node"])
    for g in source["supports"]:
        add({"tag": "support_force_magnitude", "support": g["id"]}, "force", g["node"])
        add({"tag": "support_moment_magnitude", "support": g["id"]}, "moment", g["node"])
    return layout


def _coupled(s, length):
    tr, ro, fo, mo = s
    return list(s) if length == 0 else [max(tr, length * ro), max(ro, tr / length), max(fo, mo / length), max(mo, length * fo)]


def _g5a_coverage(body, case, source, s, need):
    """I57 s2/s4 G5a from the proof-owned coverage only.

    Selected case (s is its Selection), in order: native p/P and the selected
    verification record; canonical source layout and extent; compact-flag Boolean
    feasibility; estimate/charge rederivation; exact summary rosters (PP
    validate_summary_shape items 1-4); directly derivable data facts.

    Unavailable case that keeps a complete vector (s is None): the same source,
    feasibility, verification-record and direct data checks, with the p512 floor
    positivity derived from the selected Run's verification record (Phi > 0 iff
    e-hat > 0, adaptive.rs `rule`); no Selection rosters and no selected pass
    condition. Native coverage is computed before any certificate verdict
    (final_case.rs `summary_coverage_data`, assigned once every body completes), so every complete vector meets these.
    Final rows never supply a private nonzero or data fact.
    """
    names = ["translation", "rotation", "force", "moment"]
    bodies = source["body_membership"]; ids = [b["body"] for b in bodies]
    coverage = body["product_attempts"][int(case["product_attempt_ref"])]["proof"]["summary_coverage"]
    need(coverage is not None and [x["body"] for x in coverage] == ids)
    last = case["run"]["attempts"][-1]
    p = s["precision"] if s is not None else last["precision"]
    need(p in (128, 256, 512))
    record = case["run"]["records"][int(last["verification"]["record"])]
    verification = record["verification"]
    need(record["precision"] == 2 * p and verification is not None and (s is None or s["verification_precision"] == 2 * p))
    # Reader parity rule 1: the full canonical layout rebuilt from the bound source
    # maps must equal the receipt layout (relabelled, dropped, reordered or foreign-body
    # rows fail). Only constrained displacement/rotation rows are input-derived and every
    # prescription is exact +0 in this C3 scope, so D=false.
    need(_same(source["layout"], _canonical_layout(source, need)))
    fixed = {(c["dof"]["node"], c["dof"]["component"]) for c in source["constraints"]}
    floors = {} if s is None or s["floor"] is None else {x["body"]: x for x in s["floor"]}
    # Reader parity rule: the record's resolution and theta list bodies 0..n-1 in order.
    need([x["body"] for x in verification["resolution"]] == ids and [x["body"] for x in verification["theta"]] == ids)
    resolution = {x["body"]: x for x in (verification["resolution"] if s is None else s["resolution_scale"])}
    expected = {"stop_rule": [], "verification_estimate": [], "verification_charge": []}
    for b, entry in zip(bodies, coverage):
        bi = b["body"]
        rows = [r for r in source["layout"] if r["body"] == bi]
        present = [any(r["kind"] == k for r in rows) for k in names]
        non_input = [any(r["kind"] == k and not r["input_derived"] for r in rows) for k in names]
        coords = [[from_bits(v) for v in source["id_maps"]["nodes"][int(i)]["coordinates"]] for i in b["nodes"]]
        need(bool(coords))
        length = _extent(coords)  # adaptive::body_extent operation order
        floor = floors.get(bi)
        floor_positive = [False, False] if floor is None else [from_bits(floor["force"]) > 0, from_bits(floor["moment"]) > 0]
        if s is None and p == 512:
            hat = _e_hat([from_bits(resolution[bi]["force"]), from_bits(resolution[bi]["moment"])], length)
            floor_positive = [_phi_512(hat[0]) > 0, _phi_512(hat[1]) > 0]
        stop = entry["stop"]
        # Necessary public consistency: some permitted private A reproduces the attested
        # stop bits (final_case.rs coverage formula). No A is claimed as the actual one.
        feasible = False
        for mask in range(16):
            a = [bool(mask >> k & 1) for k in range(4)]
            if any(a[k] and not non_input[k] for k in range(4)): continue
            positive = list(a) if length == 0 else [a[0] or a[1], a[0] or a[1], a[2] or a[3], a[2] or a[3]]
            positive[2] = positive[2] or floor_positive[0]
            positive[3] = positive[3] or floor_positive[1]
            if [present[k] and (positive[k] or a[k]) for k in range(4)] == stop:
                feasible = True
                break
        need(feasible)
        e = resolution[bi]
        hats = [from_bits(e["force"]) > 0, from_bits(e["moment"]) > 0]
        if length != 0: hats = [hats[0] or hats[1]] * 2
        estimate = [present[2] and hats[0], present[3] and hats[1]]
        # Native p512: every force/moment row is non-input-derived, so charge is stop.
        charge = [stop[2], stop[3]] if p == 512 else estimate
        expected["stop_rule"] += [(bi, k) for k, bit in zip(names, stop) if bit]
        expected["verification_estimate"] += [(bi, k) for k, bit in zip(names[2:], estimate) if bit]
        expected["verification_charge"] += [(bi, k) for k, bit in zip(names[2:], charge) if bit]
    has_data = {x["body"]: x["has_data"] for x in coverage}
    if s is not None:
        need([(x["body"], x["kind"]) for x in s["stop_rule"]] == expected["stop_rule"])
        need([(x["body"], x["kind"]) for x in s["verification_estimate"]] == expected["verification_estimate"])
        need([(x["body"], x["kind"]) for x in s["verification_charge"]] == expected["verification_charge"])
        need([x["body"] for x in s["certified_bound"]] == [bi for bi in ids if has_data[bi]])
    # Parity rule 3: one record bound per body, in order, non-null iff has_data.
    need([x["body"] for x in verification["bound"]] == ids and all((x["value"] is not None) == has_data[x["body"]] for x in verification["bound"]))
    theta = {x["body"]: x["value"] for x in verification["theta"]}
    need(all(has_data[bi] or theta[bi] == "0000000000000000" for bi in ids))
    true_count = sum(1 for bi in ids if has_data[bi])
    need((verification["data_blocks"] == 0) == (true_count == 0) and verification["data_blocks"] >= true_count)
    # Directly derivable data facts from separate original contributions; a netted load
    # or a zero final row never implies has_data=false.
    for b in bodies:
        free = {(n, c) for n in b["nodes"] for c in COMPONENTS} - fixed
        if not free: need(not has_data[b["body"]])
        if any(t["dof"]["node"] in b["nodes"] and (t["dof"]["node"], t["dof"]["component"]) in free and from_bits(t["value"]) != 0
               for t in source["nodal_terms"]):
            need(has_data[b["body"]])


def _g5_numeric(body, rows_by_case, phase=None):
    """G5a, then G5b, then G5c, each across all cases in case order.

    C3_DELTA s4 keeps C1's gate order G0..G8 ("within a gate ... ascending attempt
    index; first failure wins") and C1 s6 has every reader execute G0->G8 in the same
    order, so every case's G5a precedes any case's G5b (reader parity rule).
    """
    classes = []
    states = []
    for case in body["cases"]:
        if case["status"] != "selected":
            # I57 s4: an unavailable attempt that keeps a complete vector still meets the
            # structural/source-consistency checks; no Selection or selected pass condition.
            ai = case.get("product_attempt_ref")
            a = None if ai is None else body["product_attempts"][int(ai)]
            if case["status"] == "unavailable" and a is not None and a["proof"] is not None and a["proof"]["summary_coverage"] is not None:
                _g5a_coverage(body, case, body["sources"][int(a["source_ref"])], None,
                              lambda ok, suffix="SCALE_MISMATCH": _need(ok, "G5a", suffix))
            continue
        source = body["sources"][int(case["source_ref"])]; s = case["selection"]
        rows = rows_by_case[case["basis_ref"]["ref_id"]]
        bodies = source["body_membership"]; names = ["translation", "rotation", "force", "moment"]
        need = lambda ok, suffix="SCALE_MISMATCH": _need(ok, "G5a", suffix)
        need(s["verification_precision"] == 2 * s["precision"] and s["floor_ratio"] == "3dd0000000000000")
        need(from_bits(s["pivot_margin_min"]) > 0 and from_bits(s["rcond"]) > 0)
        # Existing summary encodings/ranges. The exact rosters are the actual native
        # summary coverage below (I57 s4), never a Cartesian body x kind product.
        for key, limit in [("stop_rule", 2.0 ** -64), ("verification_estimate", .25), ("verification_charge", 1.)]:
            need(all(0 <= from_bits(x["value"]) <= limit for x in s[key]))
        for key in ["resolution_scale", "theta", "body_scales"]: need([x["body"] for x in s[key]] == [b["body"] for b in bodies])
        need(all(0 <= from_bits(x["value"]) <= .5 for x in s["theta"]))
        need(len({x["body"] for x in s["certified_bound"]}) == len(s["certified_bound"]) and all(x["body"] in [b["body"] for b in bodies] and from_bits(x["value"]) > 0 for x in s["certified_bound"]))
        need((s["floor"] is not None) == (s["precision"] == 512))
        if s["floor"] is not None: need([x["body"] for x in s["floor"]] == [b["body"] for b in bodies])
        _g5a_coverage(body, case, source, s, need)
        prescribed = {(x["node_id"], x["component"]) for x in s["input_derived_dofs"]}
        need(len(prescribed) == len(s["input_derived_dofs"]))
        actual = {(source["id_maps"]["nodes"][int(c["dof"]["node"])]["id"], c["dof"]["component"]) for c in source["constraints"]}
        deferred_class_checks = [(prescribed == actual, "INPUT_DOF_MISMATCH")]
        values = {}; extents = {}; raw_scales = {}
        for b in bodies:
            bi = b["body"]
            coords = [[from_bits(v) for v in source["id_maps"]["nodes"][int(i)]["coordinates"]] for i in b["nodes"]]
            need(bool(coords))
            extent = _extent(coords); extents[bi] = extent
            maxima = [0., 0., 0., 0.]
            for ri, row in enumerate(rows):
                kind = _row_kind(row); rb, member = _row_body(row, source); n = _normalized(row)
                need(math.isfinite(n))
                component = None
                if row["kind"].startswith("global_nodal_displacement_"): component = "U" + row["kind"][-1].upper()
                if row["kind"].startswith("global_nodal_rotation_"): component = "R" + row["kind"][-1].upper()
                input_derived = component is not None and (row["entity_ref"], component) in prescribed
                values[ri] = (kind, rb, member, n, input_derived)
                if rb == bi and kind in names and not input_derived: maxima[names.index(kind)] = max(maxima[names.index(kind)], abs(n))
            raw_scales[bi] = _coupled(maxima, extent)
        # G5a resolution sanity and stiffness/displacement lower checks use the
        # original coupled maxima, before native-p512 floors are applied.
        for bi, scale in raw_scales.items():
            resolution = s["resolution_scale"][int(bi)]; e = [from_bits(resolution[k]) for k in ["force", "moment"]]
            length = extents[bi]
            hats = e if length == 0 else [max(e[0], e[1] / length), max(e[1], length * e[0])]
            upper = [x * float.fromhex("0x1.0000000001000p+0") for x in hats]
            need(upper[0] >= scale[2] and upper[1] >= scale[3])
            for ri, (kind, rb, member, n, inp) in values.items():
                if rb == bi and kind in ("force", "moment") and e[0 if kind == "force" else 1] == 0: need(bits(n) == "0000000000000000")
            for section in source["section_terms"]:
                member = next(m for m in source["id_maps"]["members"] if m["kernel_member"] == section["member"])
                if member["node_i"] not in bodies[int(bi)]["nodes"]: continue
                for k in (0, 1):
                    total = 0.
                    for node in (member["node_i"], member["node_j"]):
                        entity = source["id_maps"]["nodes"][int(node)]["id"]
                        components = []
                        for axis in "xyz":
                            name = "global_nodal_" + ("displacement_" if k == 0 else "rotation_") + axis
                            hits = [r for r in rows if r["entity_ref"] == entity and r["kind"] == name]
                            need(len(hits) == 1); components.append(abs(_normalized(hits[0])))
                        endpoint = (components[0] + components[1]) + components[2]
                        total = total + endpoint
                    threshold = (2.0 ** -59) * scale[k]
                    lower = 0. if total <= threshold else from_bits(section["axial_stiffness" if k == 0 else "torsional_stiffness"]) * (total - (2.0 ** -60) * scale[k])
                    need(upper[k] >= lower)
        states.append((source, s, rows, bodies, names, values, extents, raw_scales, deferred_class_checks))
    if phase is not None: phase[0] = "G5b"
    for source, s, rows, bodies, names, values, extents, raw_scales, deferred_class_checks in states:
        final_scales = {}
        for bi, scale in raw_scales.items():
            result = list(scale)
            if s["precision"] == 512:
                floor = s["floor"][int(bi)]
                resolution = s["resolution_scale"][int(bi)]
                hat = _e_hat([from_bits(resolution["force"]), from_bits(resolution["moment"])], extents[bi])
                _need([bits(_phi_512(hat[0])), bits(_phi_512(hat[1]))] == [floor["force"], floor["moment"]], "G5b", "SCALE_MISMATCH")
                result[2] = max(result[2], from_bits(floor["force"])); result[3] = max(result[3], from_bits(floor["moment"]))
            _need([bits(x) for x in result] == [s["body_scales"][int(bi)][k] for k in names], "G5b", "SCALE_MISMATCH")
            final_scales[bi] = result
        _need(len(s["section_terms"]) == len(source["section_terms"]), "G5b", "SECTION_MISMATCH")
        for left, right in zip(s["section_terms"], source["section_terms"]):
            member = next(m for m in source["id_maps"]["members"] if m["kernel_member"] == right["member"])
            # D18 (retained_product.rs `ScalarWork::operation`, `evaluate_operational`; endpoint_maximum.rs `endpoint_maximum`): each echoed term equals the source's and is positive.
            _need(left["member_id"] == member["id"] and all(left[k] == right[k] and from_bits(left[k]) > 0 for k in ["area", "section_modulus", "length", "axial_stiffness", "torsional_stiffness"]), "G5b", "SECTION_MISMATCH")
        absolute = []; uncovered = []
        for ri, row in enumerate(rows):
            kind, bi, member, n, inp = values[ri]; scale = None; bound = None
            if kind == "non_quantity": classification = "non_quantity"
            elif inp or kind == "input_derived":
                classification = "input_derived"
                if inp: deferred_class_checks.append((bits(n) == "0000000000000000", "INPUT_DOF_MISMATCH"))
            elif kind == "not_covered" or bi is None:
                classification = "not_covered"; uncovered.append(row["id"])
            else:
                if kind in names: scale = final_scales[bi][names.index(kind)]
                else:
                    if member is None:
                        classification = "not_covered"; uncovered.append(row["id"])
                        classes.append({"result_id":row["id"],"basis_ref":row["basis_ref"],"normalized_bits":bits(n),"scale_bits":None,"class":classification,"bound_bits":None}); continue
                    section = next((x for x in source["section_terms"] if x["member"] == member["kernel_member"]), None)
                    _need(section is not None, "G5b", "SECTION_MISMATCH")
                    k = float.fromhex("0x1.6a09e667f3bcdp+1") if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" else 4. if row["kind"] == "open_formula_stress_summary" else 1.
                    if row["kind"] == "component_equal_factor_intensified_bending_stress_v1":
                        _need(False, "G5b", "SECTION_MISMATCH")
                    scale = (final_scales[bi][2] / from_bits(section["area"])) + (k * (final_scales[bi][3] / from_bits(section["section_modulus"])))
                _need(math.isfinite(scale) and scale >= 0, "G5b", "SCALE_MISMATCH")
                if scale >= 2.0 ** -988 and not abs(n) < ((2.0 ** -34) * scale): classification = "relative_verified"
                else:
                    classification = "absolute_verified"; bound = absolute_bound(n, scale)
                    absolute.append({"result_id":row["id"],"bound":bits(bound)})
            classes.append({"result_id":row["id"],"basis_ref":row["basis_ref"],"normalized_bits":bits(n),"scale_bits":None if scale is None else bits(scale),"class":classification,"bound_bits":None if bound is None else bits(bound)})
        deferred_class_checks.append((s["absolute_verified"] == absolute and s["not_covered"] == uncovered, "CLASSIFICATION_MISMATCH"))
    if phase is not None: phase[0] = "G5c"
    for *_, deferred_class_checks in states:
        for ok, code in deferred_class_checks:
            _need(ok, "G5c", code)
    return classes


def _g8(body, source, invocation):
    need = lambda ok, code="PREPARATION_MISMATCH": _need(ok, "G8", code)
    try: need(_hash("source_blocks_invocation_v1", invocation) == body["invocation"]["value"], "INVOCATION_MISMATCH")
    except RetainedPrecisionError: raise
    except (ValueError, RuntimeError): need(False, "INVOCATION_MISMATCH")
    request = invocation["request"]; model = request["model"]
    need(model["project"]["id"] == source["model_ref"], "INVOCATION_MISMATCH")
    need(model.get("schema_version") in ("0.1.0", "0.2.0", "0.3.0") and not model.get("pressure_contract") and not model.get("combinations"), "INVOCATION_MISMATCH")
    need(not model.get("components"), "INVOCATION_MISMATCH")
    nodes, pipes, supports = model["nodes"], model["pipe_segments"], model["supports"]
    need(len({x["id"] for x in nodes}) == len(nodes) and len({x["id"] for x in pipes}) == len(pipes) and len({x["id"] for x in supports}) == len(supports))
    materials = request.get("materials") or model.get("materials", [])
    from core.units.adapter import convert_quantities_to_canonical
    def unit(q, dimension):
        values = convert_quantities_to_canonical([{"id":"v","value":q["value"],"unit":q["unit"],"dimension":dimension}])
        return float(values[0]["value"])
    def selected_material(material, case):
        base = (unit(material["elastic_modulus"], "stress"), unit(material["shear_modulus"], "stress"))
        need(all(math.isfinite(v) and v > 0 for v in base))
        named, temperature = case.get("modulus_basis_ref"), case.get("modulus_basis_temperature")
        need(named is None or temperature is None)
        if named is None and temperature is None: return base, {"kind":"base"}
        points = material.get("temperature_points", [])
        if named is not None:
            point = next(p for p in points if p["id"] == named)
            return (unit(point["elastic_modulus"], "stress"), unit(point["shear_modulus"], "stress")), {"kind":"named_point","point_id":point["id"]}
        t = unit(temperature, "temperature")
        ordered = sorted([(unit(p["temperature"], "temperature"), p) for p in points if p.get("temperature") is not None], key=lambda p:p[0])
        need(len({p[0] for p in ordered}) == len(ordered))
        bracket = next(((a,b) for a,b in zip(ordered,ordered[1:]) if a[0] < t < b[0]), None)
        need(bracket is not None)  # strict adjacent bracket (lib.rs `materials_for_modulus_basis_observed`); D16: an explicit G8 check
        lo, hi = bracket
        ratio = (t - lo[0]) / (hi[0] - lo[0])
        # lib.rs `materials_for_modulus_basis_observed` requires all three source quantities even for the
        # nonthermal ordinary route, then evaluates lo + f * (hi - lo).
        need(all(p.get(k) is not None for p in (lo[1],hi[1]) for k in ("elastic_modulus","shear_modulus","thermal_expansion_coefficient")))
        result = tuple(unit(lo[1][key], "stress") + ratio * (unit(hi[1][key], "stress") - unit(lo[1][key], "stress")) for key in ("elastic_modulus","shear_modulus"))
        return result, {"kind":"interpolated","lower_point_id":lo[1]["id"],"upper_point_id":hi[1]["id"],"target_kelvin":bits(t)}
    def operational(inputs):
        x = [from_bits(v) for v in inputs]; delta = [x[i+3]-x[i] for i in range(3)]
        length = math.sqrt(((delta[0]*delta[0])+(delta[1]*delta[1]))+(delta[2]*delta[2]))
        need(length > 1e-12 and math.isfinite(length))
        inverse = 1.0 / length
        axial = (x[6]*x[8])/length; torsion = (x[7]*x[9])/length
        need(math.isfinite(axial) and math.isfinite(torsion) and abs(axial) >= 2.0**-1022 and abs(torsion) >= 2.0**-1022)
        return {"kind":"ready","length":bits(length),"axial_stiffness":bits(axial),"torsional_stiffness":bits(torsion),"normalization":[bits(d*inverse) for d in delta]}
    for si, s in enumerate(body["sources"]):
        need(s["index"] == si and s["owner"]["kind"] == "case")
        for include_loads, field in ((True, "kernel_source_sha256"), (False, "stiffness_sha256")):
            need(hashlib.sha256(_native_source_encoding(s, include_loads)).hexdigest() == s[field])
        ci = int(s["owner"]["case_index"]); case = model["load_cases"][ci]
        need(case["id"] == s["owner"]["case_id"] and not case.get("pressure_regions") and case.get("equivalent_static") is None)
        maps = s["id_maps"]
        need(len(maps["nodes"]) == len(nodes) and len(maps["members"]) == len(pipes) and len(maps["support_ids"]) == len(supports))
        need(len(nodes)*6 <= 0xffffffff and len(pipes)*3 <= 0xffffffff)
        for i, (n, raw) in enumerate(zip(maps["nodes"], nodes)):
            coordinates = [bits(unit({"value":raw["position"][a],"unit":model["project"]["units"]["length"]},"length")) for a in "xyz"]
            need(n["model_index"] == n["kernel_node"] == i and n["id"] == raw["id"] and n["coordinates"] == coordinates)
        mb = _at(body["material_bases"], s["material_basis_ref"], "G8", "PREPARATION_MISMATCH")
        selector = {"kind":"named","id":case["modulus_basis_ref"]} if case.get("modulus_basis_ref") is not None else {"kind":"temperature","kelvin":bits(unit(case["modulus_basis_temperature"],"temperature"))} if case.get("modulus_basis_temperature") is not None else {"kind":"base"}
        need(mb["selector"] == selector and ci in mb["case_indices"])
        used = {p["material"] for p in pipes}
        need([m["input_index"] for m in mb["materials"]] == [i for i,m in enumerate(materials) if m["id"] in used])
        for m in mb["materials"]:
            raw = materials[int(m["input_index"])]; pair, selection = selected_material(raw,case)
            need(m["id"] == raw["id"] and m["selection"] == selection and m["shear_origin"] == {"kind":"explicit_g"} and [m["elastic_modulus"],m["shear_modulus"]] == [bits(v) for v in pair])
        need(len(s["section_terms"]) == len(pipes))
        for i, (m, raw, section) in enumerate(zip(maps["members"],pipes,s["section_terms"])):
            need(m["model_index"] == m["kernel_member"] == section["member"] == i and m["id"] == raw["id"])
            need(0 <= m["built_pipe_index"] < len(pipes) and m["node_i"] != m["node_j"])
            need(nodes[int(m["node_i"])]["id"] == raw["from"] and nodes[int(m["node_j"])]["id"] == raw["to"])
            need(m["y_reference"] == [bits(float(raw["y_reference"][a])) for a in "xyz"])
            mat = next(x for x in mb["materials"] if x["input_index"] == m["material_index"])
            need(mat["id"] == raw["material"] and m["E"] == mat["elastic_modulus"] and m["G"] == mat["shear_modulus"])
            geo=section["geometry"]; d=unit(raw["section"]["outside_diameter"],"length"); wall=unit(raw["section"]["wall_thickness"],"length")
            tolerance=unit(raw["section"]["mill_tolerance"],"length") if raw["section"].get("mill_tolerance") is not None else 0.
            need(geo["route"] == "preview" and geo["normalized_od"] == bits(d) and geo["effective_wall"] == bits(wall-tolerance) and 0 < wall-tolerance < d*.5)
            need(geo["actual_radius"] == bits(d*.5) and section["area"] == m["A_K"] and geo["actual_second_moment"] == m["Iy_K"] == m["Iz_K"] and geo["actual_polar_moment"] == m["J_K"])
            need(all(from_bits(m[k]) >= 2.0**-1022 for k in ["E","G","A_K","Iy_K","Iz_K","J_K"]) and from_bits(section["section_modulus"]) > 0)
        need(len({m["built_pipe_index"] for m in maps["members"]}) == len(pipes))
        # Re-derive topology, not native numerical state.
        parent=list(range(len(nodes)))
        def find(x):
            while parent[x] != x: x=parent[x]
            return x
        for m in maps["members"]:
            a,b=find(int(m["node_i"])),find(int(m["node_j"]));parent[max(a,b)]=min(a,b)
        roots=sorted({find(i) for i in range(len(nodes))}); expected=[]
        for bi,rt in enumerate(roots):
            ns=[i for i in range(len(nodes)) if find(i)==rt]
            expected.append({"body":bi,"nodes":ns,"members":[m["kernel_member"] for m in maps["members"] if m["node_i"] in ns]})
        need(s["body_membership"] == expected)
        fixed={}; expected_springs=[]; expected_supports=[]
        for i,raw in enumerate(supports):
            need(raw.get("nonlinear") is None and raw.get("hanger") is None and raw.get("imposed_displacement") is None and raw.get("family") in (None,"anchor","guide","line_stop","vertical_support","spring"))
            node=next(j for j,n in enumerate(nodes) if n["id"]==raw["node"])
            need(maps["support_ids"][i] == {"model_index":i,"kernel_support":i,"id":raw["id"],"node":node})
            restrained=[False]*6; spring_ids=[]
            if raw.get("family") == "spring":
                q=raw["stiffness"]; component=q["dof"]; need(component in COMPONENTS)
                stiffness=unit(q["value"],"linear_stiffness" if COMPONENTS.index(component)<3 else "rotational_stiffness") if isinstance(q.get("value"),dict) else unit(q,"linear_stiffness" if COMPONENTS.index(component)<3 else "rotational_stiffness")
                need(stiffness > 0); sid=len(expected_springs);spring_ids=[sid]
                expected_springs.append({"boundary_index":sid,"kernel_spring":sid,"support_index":i,"node":node,"component":component,"stiffness":bits(stiffness)})
            else:
                for c in raw["restraints"]:
                    need(c in COMPONENTS);restrained[COMPONENTS.index(c)]=True;fixed.setdefault((node,c),[]).append(i)
            expected_supports.append({"id":i,"node":node,"restrained":restrained,"springs":spring_ids,"directional_springs":[]})
        need(maps["springs"] == expected_springs and s["supports"] == expected_supports)
        constraints=[{"dof":{"node":n,"component":c},"value":"0000000000000000","support_indices":ids} for (n,c),ids in sorted(fixed.items(),key=lambda x:(x[0][0],COMPONENTS.index(x[0][1])))]
        need(s["constraints"] == constraints)
        stations=[{"id":3*i+j,"member":i,"location":location,"fraction":bits(fraction)} for i in range(len(pipes)) for j,(location,fraction) in enumerate(zip(["quarter_1","midspan","quarter_3"],[.25,.5,.75]))]
        need(s["stations"] == stations)
        terms=[];directions={**dict(zip(COMPONENTS,COMPONENTS)),**dict(zip(["global_x","global_y","global_z","rotation_x","rotation_y","rotation_z"],COMPONENTS))}
        for i,load in enumerate(case["primitive_loads"]):
            need(load["target"]["type"] == "node" and load.get("category") != "thermal" and load["dimension"] in ("force","moment"))
            node=next(j for j,n in enumerate(nodes) if n["id"]==load["target"]["node"]);component=directions[load["direction"]]
            need((COMPONENTS.index(component)<3)==(load["dimension"]=="force"))
            terms.append({"constructor_ordinal":i,"source_id":load["id"],"primitive_load_index":i,"dof":{"node":node,"component":component},"value":bits(unit(load["magnitude"],load["dimension"]))})
        terms.sort(key=lambda t:(t["dof"]["node"]*6+COMPONENTS.index(t["dof"]["component"]),t["source_id"].encode(),t["value"],t["constructor_ordinal"]))
        need(s["nodal_terms"] == terms)
        def body_of(n):return next(b["body"] for b in expected if n in b["nodes"])
        layout=[]
        def add(q,k,n,inp=False):layout.append({"index":len(layout),"quantity":q,"kind":k,"body":body_of(n),"input_derived":inp})
        for n in range(len(nodes)):
            for j,c in enumerate(COMPONENTS):add({"tag":"displacement","dof":{"node":n,"component":c}},"translation" if j<3 else "rotation",n,(n,c) in fixed)
        for n in range(len(nodes)):add({"tag":"displacement_magnitude","node":n},"translation",n)
        for m in maps["members"]:
            for end in ("i","j"):
                for j,c in enumerate(COMPONENTS):add({"tag":"end_action","member":m["kernel_member"],"end":end,"component":c},"force" if j<3 else "moment",m["node_i"])
        for st in stations:
            m=maps["members"][st["member"]]
            for j,c in enumerate(COMPONENTS):add({"tag":"station_action","station":st["id"],"component":c},"force" if j<3 else "moment",m["node_i"])
        for spring in expected_springs:add({"tag":"spring_action","spring":spring["kernel_spring"],"component":spring["component"]},"force" if COMPONENTS.index(spring["component"])<3 else "moment",spring["node"])
        for constraint in constraints:
            dof=constraint["dof"];add({"tag":"reaction","dof":dof},"force" if COMPONENTS.index(dof["component"])<3 else "moment",dof["node"])
        for support in expected_supports:
            add({"tag":"support_force_magnitude","support":support["id"]},"force",support["node"]);add({"tag":"support_moment_magnitude","support":support["id"]},"moment",support["node"])
        need(s["layout"] == layout)
        if s["preparation"] is not None:
            a=body["product_attempts"][int(s["preparation"]["attempt_ref"])];need(a["source_ref"]==si and s["preparation"]["sha256"]==_hash("retained_precision_preparation_v1",_preparation_payload(a)))
            for j,m in enumerate(a["preparation"]["members"]):
                old=m["old_source"];f=m["old_facts"];new=m["result"]["section"];member=maps["members"][j];section=s["section_terms"][j]
                need(old[:2]==[member["E"],member["G"]] and old[2]==f[2] and old[3]==old[4]==f[3] and old[5]==f[4])
                need(f[:2]==[section["geometry"]["normalized_od"],section["geometry"]["effective_wall"]])
                need(new==[section["area"],section["geometry"]["actual_second_moment"],section["geometry"]["actual_polar_moment"],section["section_modulus"],section["geometry"]["actual_radius"]])
                positions=maps["nodes"][int(member["node_i"])]["coordinates"]+maps["nodes"][int(member["node_j"])]["coordinates"]
                op_old=a["operational"]["old"][j];need(op_old["inputs"]==positions+[old[i] for i in (0,1,2,5)])
                if j<len(a["operational"]["new"]):
                    op=a["operational"]["new"][j];need(op["inputs"]==positions+old[:2]+[new[0],new[2]])
                    if op["result"]["kind"]=="ready":
                        need(op["result"]==operational(op["inputs"]))
                        need(all(op["result"][k]==section[k] for k in ["length","axial_stiffness","torsional_stiffness"]))
    # F1:101-106 / checklist P7-G8: every attempt binds each retained old operational tuple that
    # has a PreparedMember to that member's old_source, and the old tuple to the invocation's
    # selected material and request geometry (C3:155-158). Old entries without a PreparedMember
    # are unattached producer attestations and are not bound.
    for a in body["product_attempts"]:
        case = model["load_cases"][int(a["owner_ref"]["index"])]
        if a["operational"]["old_coverage"] == "complete":
            # D1 (checkpoint A correction): a complete old list equals the invocation's member count.
            need(len(a["operational"]["old"]) == len(pipes))
        for j, member in enumerate(a["preparation"]["members"]):
            op_old = a["operational"]["old"][j]
            old, facts = member["old_source"], member["old_facts"]
            pipe = pipes[int(member["member"])]
            material = next(m for m in materials if m["id"] == pipe["material"])
            pair, _ = selected_material(material, case)
            d = unit(pipe["section"]["outside_diameter"], "length"); wall = unit(pipe["section"]["wall_thickness"], "length")
            tolerance = unit(pipe["section"]["mill_tolerance"], "length") if pipe["section"].get("mill_tolerance") is not None else 0.
            need(int(op_old["member"]) == int(member["member"]) and op_old["inputs"][6:] == [old[i] for i in (0, 1, 2, 5)])
            need(old[:2] == [bits(v) for v in pair] and facts[:2] == [bits(d), bits(wall - tolerance)]
                 and old[2] == facts[2] and old[3] == old[4] == facts[3] and old[5] == facts[4])


def validate_retained_precision(source: Any, invocation: Any = None) -> dict[str, Any]:
    """The accepted ordered reader (G0-G8). D-U6-1: every gate runs; since U7 a valid invocation-bound
    statement of a solved model whose cases are selected or not_required reads eligible. Hashes bind
    the supplied statements; they do not establish producer origin (D-U7-6)."""
    return _validate_draft(source, invocation)


def _validate_draft(source: Any, invocation: Any = None) -> dict[str, Any]:
    """The ordered checks behind the public entry (D-U6-1)."""
    gate="G0"
    try:
        producer=source.get("producer") if type(source) is dict else None;basis=source.get("formulation_basis") if type(source) is dict else None
        _need(type(producer) is dict and type(basis) is dict and producer.get("semantic_contract_id")==CONTRACT_ID and basis.get("profile_id")==PROFILE,"G0","SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        snapshot=deepcopy(source);invocation=deepcopy(invocation);receipt=snapshot.get("retained_precision");schema=_schema()
        _need(snapshot.get("schema_version")=="0.2.0" and snapshot["producer"].get("component_name")=="open_pipe_stress_product_physics" and snapshot["producer"].get("component_version")=="0.2.0",gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        definition=json.loads((ROOT/"fixtures/results/retained_precision_prepared_ordinary_v1.json").read_text())
        _need(_hash("retained_precision_formation_v1",definition)==DEFINITION_HASH,gate,"FORMATION_MISMATCH")
        table_bytes=(ROOT/"fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes()
        table=json.loads(table_bytes)
        inherited_bytes=(ROOT/"fixtures/results/semantic_contract_v0_3_preview_physics_1.json").read_bytes()
        _need(hashlib.sha256(table_bytes).hexdigest()==TABLE_HASH and hashlib.sha256(inherited_bytes).hexdigest()==table["inherited_semantic_contract_sha256"],gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        # D2 + settled readings 1-2: an absent retained_precision or body is an absent G0 field;
        # receipt_version is exactly 1 (C1 s4); thresholds and canonicalization are G0 fields.
        _need(type(receipt) is dict and type(receipt.get("body")) is dict,gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        if True:
            b=receipt["body"]
            for key,value in {"receipt_version":1,"policy":"M03-INTEGRITY-MP-v2","projection_policy":"RP-LOGICAL-ATTEMPTS-v1","work_policy":"W1-LME-20B-60B-v1","facade_policy":"RP-FACADE-SI-v2","canonicalization":"openpipestress_jcs_ijson_v1"}.items():
                _need((_integral(b.get(key))==value) if type(value) is int else (type(b.get(key)) is str and b.get(key)==value),gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
            w=b.get("work")
            _need(type(w) is dict and _integral(w.get("case_limit"))==20_000_000_000 and _integral(w.get("invocation_limit"))==60_000_000_000,gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
            for attempt in b.get("product_attempts",[]) if type(b.get("product_attempts")) is list else []:
                if type(attempt) is dict:_need(attempt.get("definition_id")==DEFINITION_ID,gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        gate="G1";_need(_shape(receipt,schema) and type(snapshot.get("results")) is list and all(_shape(r,schema["$defs"]["RawRow"]) for r in snapshot["results"]),gate,"RECEIPT_MISMATCH")
        body=receipt["body"]
        _need(_hash("retained_precision_receipt_mp_v2",body)==receipt["receipt_sha256"],gate,"RECEIPT_MISMATCH")
        _need(_hash("retained_precision_publication_mp_v2",{k:v for k,v in snapshot.items() if k!="retained_precision"})==body["publication_sha256"],gate,"RECEIPT_MISMATCH")
        # Integrity is G1. Invalid reference representation/coverage is left for
        # G2/G3; only already-addressable records have an integrity comparison.
        for c in body["cases"]:
            si=_integral(c.get("source_ref"))
            if c["status"]=="selected" and si is not None and 0<=si<len(body["sources"]):
                _need(c["source_identity_sha256"]==_source_hash(body["sources"][si]),gate,"RECEIPT_MISMATCH")
        for s in body["sources"]:
            prep=s["preparation"]
            if prep is not None:
                ai=_integral(prep["attempt_ref"])
                if ai is not None and 0<=ai<len(body["product_attempts"]):
                    a=body["product_attempts"][ai]
                    if all(m["result"]["kind"]=="prepared" for m in a["preparation"]["members"]):
                        _need(prep["sha256"]==_hash("retained_precision_preparation_v1",_preparation_payload(a)),gate,"RECEIPT_MISMATCH")
        gate="G2";_encoding(receipt,schema);_need(not _negative_zero(receipt),gate,"ENCODING_MISMATCH");_normalize_integrals(receipt)  # D34, then D32
        gate="G3";cases=body["cases"];quality=snapshot["numerical_quality"]["cases"]
        ids=[c["basis_ref"]["ref_id"] for c in cases]
        _need(len(set(ids))==len(ids) and [c["basis_ref"] for c in cases]==[q["basis_ref"] for q in quality] and any(c["status"]=="selected" for c in cases),gate,"COVERAGE_MISMATCH")
        if invocation is not None:_need(ids==[c["id"] for c in invocation["request"]["model"]["load_cases"]],gate,"COVERAGE_MISMATCH")
        rows={cid:[] for cid in ids};seen=set()
        for row in snapshot["results"]:
            _need(row["id"] not in seen and row.get("basis_ref",{}).get("ref_type")=="load_case" and row["basis_ref"]["ref_id"] in rows,gate,"COVERAGE_MISMATCH")
            seen.add(row["id"]);rows[row["basis_ref"]["ref_id"]].append(row)
        _need(len(body["ordinary_attempts"])==len(cases),gate,"COVERAGE_MISMATCH")
        for s in body["sources"]:_need(len(s["body_membership"])>0,gate,"COVERAGE_MISMATCH")  # D29
        refs=[c["product_attempt_ref"] for c in cases if c["product_attempt_ref"] is not None]
        _need(sorted(refs)==list(range(len(body["product_attempts"]))),gate,"COVERAGE_MISMATCH")
        for ai,a in enumerate(body["product_attempts"]):
            _need(a["id"]==ai and a["owner_ref"]["kind"]=="case" and a["owner_ref"]["index"]<len(cases),gate,"COVERAGE_MISMATCH")
            c=cases[int(a["owner_ref"]["index"])]
            _need(c["product_attempt_ref"]==ai,gate,"COVERAGE_MISMATCH")
            old,new=a["operational"]["old"],a["operational"]["new"];pm=a["preparation"]["members"]
            _need([x["member"] for x in old]==list(range(len(old))) and [x["member"] for x in pm]==list(range(len(pm))) and [x["member"] for x in new]==list(range(len(new))) and len(new)<=len(pm)<=len(old),gate,"COVERAGE_MISMATCH")
            if a["operational"]["old_coverage"]=="captured_prefix":_need(not pm and not new,gate,"COVERAGE_MISMATCH")
            if a["operational"]["old_coverage"]=="complete":
                # D1 (F1:101, 130; C3:302): complete old ids equal the member inventory: the source's
                # map when sourced, else every CaseSource's (one model); without one, G8 binds it.
                si=_integral(a["source_ref"])
                if si is not None and 0<=si<len(body["sources"]):_need([x["member"] for x in old]==[m["kernel_member"] for m in body["sources"][si]["id_maps"]["members"]],gate,"COVERAGE_MISMATCH")  # D23
                elif si is None:
                    for s in body["sources"]:
                        if s["owner"]["kind"]=="case":_need(len(old)==len(s["id_maps"]["members"]),gate,"COVERAGE_MISMATCH")
            if a["proof"] is not None:
                indices=[x["row_index"] for x in a["proof"]["projection_outcomes"]];case_rows=rows[c["basis_ref"]["ref_id"]]
                _need(indices==sorted(set(indices)) and all(x<len(case_rows) and case_rows[int(x)]["kind"] not in HULL_EXCLUDED for x in indices),gate,"COVERAGE_MISMATCH")
                # I57 s4 G3: a complete proof-owned roster lists the attempt's source bodies
                # exactly once, ascending 0..body_count-1. A null/invalid source reference is
                # left to the G5 association pass.
                coverage=a["proof"]["summary_coverage"];si=_integral(a["source_ref"])
                if coverage is not None and si is not None and 0<=si<len(body["sources"]):
                    inventory=[b["body"] for b in body["sources"][si]["body_membership"]]
                    _need(bool(inventory) and [x["body"] for x in coverage]==inventory==list(range(len(inventory))),gate,"COVERAGE_MISMATCH")
        # D1 (C2:117; C1:146): each run id is its execution-order position; the order is a bijection.
        order=body["work"]["execution_order"];with_run=[i for i,c in enumerate(cases) if c.get("run") is not None]
        _need(all(e["kind"]=="case" for e in order) and sorted(e["index"] for e in order)==with_run and all(cases[e["index"]]["run"]["id"]==k for k,e in enumerate(order)),gate,"COVERAGE_MISMATCH")
        for i,c in enumerate(cases):
            _need(c["ordinary"]["attempt_ref"]==i and c["ordinary"]["quality_binding"]=={"kind":"present","index":i} and body["ordinary_attempts"][i]["case_index"]==i and body["ordinary_attempts"][i]["case_id"]==ids[i],gate,"COVERAGE_MISMATCH")
        gate="G4";diags=snapshot["diagnostics"]
        _need(len({d["id"] for d in diags})==len(diags) and not any(d["code"]=="SOURCE_BLOCK_RECOVERY_SELECTED" for d in diags),gate,"DIAGNOSTIC_MISMATCH")
        for i,c in enumerate(cases):
            cid=ids[i];selected=[d for d in diags if d["code"]=="RETAINED_PRECISION_SELECTED" and cid in d.get("affected_refs",[])];unavailable=[d for d in diags if d["code"]=="RETAINED_PRECISION_UNAVAILABLE" and cid in d.get("affected_refs",[])]
            _need(len(selected)==int(c["status"]=="selected") and len(unavailable)==int(c["status"]=="unavailable") and all(d["affected_refs"]==[cid] for d in selected+unavailable),gate,"DIAGNOSTIC_MISMATCH")
            if c["status"]=="unavailable":_need(unavailable[0]["id"]==c["diagnostic_ref"],gate,"DIAGNOSTIC_MISMATCH")
            if c["status"]=="selected":_need(not any(d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE" and cid in d.get("affected_refs",[]) for d in diags),gate,"DIAGNOSTIC_MISMATCH")
        for d in diags:
            # D7 (C1 G4 row; C1:147): every retained diagnostic names exactly one requested case.
            if d["code"] in ("RETAINED_PRECISION_SELECTED","RETAINED_PRECISION_UNAVAILABLE"):_need(type(d.get("affected_refs")) is list and len(d["affected_refs"])==1 and d["affected_refs"][0] in ids,gate,"DIAGNOSTIC_MISMATCH")
        gate="G5";_g5_native(body);_g5_ordinary(body,cases,diags,quality);_g5_products(body,rows)
        gate="G5a";phase=["G5a"];classes=_g5_numeric(body,rows,phase)
        gate="G6"
        for c in cases:
            for row in rows[c["basis_ref"]["ref_id"]]:_need(row.get("recovery_method")==METHOD if c["status"]=="selected" else "recovery_method" not in row,gate,"ROW_METHOD_MISMATCH")
        gate="G7";projected=deepcopy(snapshot);del projected["retained_precision"]
        projected["producer"]["semantic_contract_id"]="openpipestress.result_semantics/0.3.0/preview-physics-1";projected["formulation_basis"]["profile_id"]="product_preview_mechanics_v1"
        for row in projected["results"]:row.pop("recovery_method",None)
        from .compatibility import _source_contract
        try:_source_contract(projected)
        except ValueError as exc:
            text=str(exc);match=re.match(r"[A-Z][A-Z0-9_]*",text)
            error=RetainedPrecisionError("G7",match.group(0) if match else "SOURCE_PREVIEW_PHYSICS_INVALID");error.detail=text
            raise error from exc
        gate="G8"
        if invocation is not None:_g8(body,snapshot,invocation)
        eligible=_IMPLEMENTATION_COMPLETE and invocation is not None and snapshot["status"]["mechanics"]=="MECHANICS_SOLVED" and all(c["status"] in ("selected","not_required") for c in cases)
        return {"invocation_bound":invocation is not None,"numerical_eligible":eligible,"standing":"eligible" if eligible else "needs_recompute","publication_sha256":body["publication_sha256"],"classifications":classes}
    except RetainedPrecisionError:raise
    except (KeyError,IndexError,TypeError,ValueError,OverflowError,ZeroDivisionError,StopIteration,AttributeError) as exc:
        # Fail-closed fallback only (D16): checks report their own codes; G5a/G5b/G5c follow the phase (D10).
        if gate=="G5a":gate=phase[0]
        code={"G0":"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED","G1":"RETAINED_PRECISION_RECEIPT_MISMATCH","G2":"RETAINED_PRECISION_ENCODING_MISMATCH","G3":"RETAINED_PRECISION_COVERAGE_MISMATCH","G4":"RETAINED_PRECISION_DIAGNOSTIC_MISMATCH","G5":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH","G5a":"RETAINED_PRECISION_SCALE_MISMATCH","G5b":"RETAINED_PRECISION_SCALE_MISMATCH","G5c":"RETAINED_PRECISION_CLASSIFICATION_MISMATCH","G6":"RETAINED_PRECISION_ROW_METHOD_MISMATCH","G8":"RETAINED_PRECISION_PREPARATION_MISMATCH"}.get(gate,"SOURCE_PREVIEW_PHYSICS_INVALID")
        raise RetainedPrecisionError(gate,code) from exc
