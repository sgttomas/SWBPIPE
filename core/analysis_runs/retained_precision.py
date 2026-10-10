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
from typing import Any, NamedTuple

ROOT = Path(__file__).resolve().parents[2]
CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PROFILE = "product_preview_retained_w1a_v2"
DEFINITION_ID = "RP-PREPARED-ORDINARY-DUAL-v1"
DEFINITION_HASH = "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349"
TABLE_HASH = "b2b4a54d610aa38c66f5d31921c2d8f3113313e33eb6933e45093ba6f1e3667c"
# B2-C (CONTRACT §8 row 4, REVISION_02 §0): DEF-C's id and H, the preview table's second formation definition.
COMBINATION_DEFINITION_ID = "RP-PREPARED-COMBINATION-DUAL-v1"
COMBINATION_DEFINITION_HASH = "d3fde142aff9c05d709b2fc2a04add42e14c66be3e2b2ba82012da57edf3d957"
PREVIEW_DEFINITIONS = [{"id": DEFINITION_ID, "sha256": DEFINITION_HASH}, {"id": COMBINATION_DEFINITION_ID, "sha256": COMBINATION_DEFINITION_HASH}]
RETAINED_DISPOSITIONS = ("retained_selected", "retained_unavailable")
METHOD = "contribution_preserving_multiprecision_v1"
# B3b (B3-D, final for J1, with REVISION_01): the `<physics-retained>` route, model 0.3.0 with the
# exact_straight_pressure_v2 contract and explicitly empty pressure regions, over physics-1. G0 reads its table's
# bound values (§2.4; decision 31, N-12) and cross-checks them against these constants, so neither drifts alone.
EXACT_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
EXACT_PROFILE = "exact_straight_retained_w1a_v2"
EXACT_DEFINITION_ID = "RP-PREPARED-EXACT-DUAL-v1"
EXACT_DEFINITION_HASH = "5a3bac430df9bbc77484d5419c75880ad40ae209b439e5f928374458025281af"
EXACT_TABLE_HASH = "c4987e874889645ac315b5f55f58690082ad5e7745527f20e3e316efa3e70a3d"
RECEIPT_POLICY = "M03-INTEGRITY-MP-v2"
FACADE_POLICY = "RP-FACADE-SI-v2"
RECEIPT_BINDINGS = {"canonicalization": "openpipestress_jcs_ijson_v1", "method": METHOD, "projection_policy": "RP-LOGICAL-ATTEMPTS-v1",
                    "work": {"case_limit": 20_000_000_000, "invocation_limit": 60_000_000_000}, "work_policy": "W1-LME-20B-60B-v1"}
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


class Route(NamedTuple):
    """The route descriptor (B3-D §6.1): one dispatch on the identity at G0; the route-specific checks (G0's table
    read, G1's and G8's preparation hash, G5b's evidence, G7's projection, G8's namespace and materials) read it."""
    exact: bool
    contract_id: str
    profile: str
    definition_id: str
    definition_hash: str
    base_id: str
    base_profile: str
    base_invalid: str
    metadata_codes: frozenset


PREVIEW_ROUTE = Route(False, CONTRACT_ID, PROFILE, DEFINITION_ID, DEFINITION_HASH,
                      "openpipestress.result_semantics/0.3.0/preview-physics-1", "product_preview_mechanics_v1",
                      "SOURCE_PREVIEW_PHYSICS_INVALID", frozenset({"SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "SOURCE_PREVIEW_PHYSICS_INVALID"}))
EXACT_ROUTE = Route(True, EXACT_CONTRACT_ID, EXACT_PROFILE, EXACT_DEFINITION_ID, EXACT_DEFINITION_HASH,
                    "openpipestress.result_semantics/0.3.0/physics-1", "exact_straight_pressure_v2",
                    "SOURCE_PHYSICS_EVIDENCE_INVALID", frozenset({"SOURCE_PHYSICS_EVIDENCE_INVALID"}))
ROUTES = {PREVIEW_ROUTE.contract_id: PREVIEW_ROUTE, EXACT_ROUTE.contract_id: EXACT_ROUTE}


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


def _preparation_payload(a, definition_hash):
    """C3 §2: `definition_sha256` is the table-bound H(definition) of the route (S-1; B3-D REVISION_01 §2): DEF-O's
    on the preview route, DEF-E's on the exact route. Every caller passes its route's hash (G1, G8); there is no
    default (RV120 N1: "never a module constant")."""
    return {"definition_id": a["definition_id"], "definition_sha256": definition_hash,
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
    # REVISION_01 §4.1 #7: the cases' and the combination entries' Runs, indexed by Run id.
    runs = [c["run"] for c in body["cases"] if c.get("run") is not None] + [c["run"] for c in body.get("combinations") or [] if c.get("run") is not None]
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
    built_refs = set()  # builds referenced by their building record (R-D38 (4b)'s Build conjunct)
    combos = body.get("combinations") or []
    for call_id, call in enumerate(body["calls"]):
        fail(call["id"] == call_id and call["invocation_before"] == current, "WORK_MISMATCH")
        mechanics = call["kind"] == "mechanics_combination"
        if mechanics:
            # REVISION_01 §4.1 #8: one owner; one source and one Run (`runs`) or none (`pre_source_refusal`).
            fail(len(call["owner_refs"]) == 1 and len(call["run_refs"]) == len(call["source_refs"]) <= 1)
        else:
            fail(len(call["run_refs"]) == len(call["source_refs"]) == len(call["owner_refs"]))
        for position, (ri, si, oi) in enumerate(zip(call["run_refs"], call["source_refs"], call["owner_refs"])):
            run = _at(runs, ri, code="ATTEMPT_MISMATCH")
            fail(run["origin"] == {"call": call_id, "position": position, "group": run["origin"]["group"], "source_ref": si, "owner_ref": oi})
            if mechanics:
                # REVISION_01 §4.1 #9: the combination entry's Run and source, owned by its CombinationSource.
                case = _at(combos, oi["index"], code="ATTEMPT_MISMATCH")
                fail(oi["kind"] == "combination" and case.get("run") == run and case.get("source_ref") == si)
                source = _at(body["sources"], si, code="ATTEMPT_MISMATCH")
                fail(source["owner"]["kind"] == "combination" and source["owner"]["combination_index"] == oi["index"])
            else:
                case = _at(body["cases"], oi["index"], code="ATTEMPT_MISMATCH")
                fail(oi["kind"] == "case" and case.get("run") == run and case.get("source_ref") == si)
                source = _at(body["sources"], si, code="ATTEMPT_MISMATCH")
                fail(source["owner"]["case_index"] == oi["index"])
            # REVISION_01 §4.1 #10: a combination Run may reuse its group's imported builds (CONTRACT §2.5).
            group = _ref(body["groups"], run["origin"]["group"]) if run["origin"]["group"] is not None else None
            imported = {(x["slot"], int(x["build"])) for x in (group or {}).get("imports", [])} if mechanics else set()
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
            # A combination's layout is its representative's (FK `CasePrep::combination` keeps operand 0's source).
            layout = source["layout"] if not mechanics else _at(body["sources"], source["operands"][0]["source_ref"], code="ATTEMPT_MISMATCH")["layout"]
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
                    fail(build["work"] == cost and (build["group"] == run["origin"]["group"] or (not built and (build["slot"], int(bi)) in imported)), "WORK_MISMATCH")
                    fail(build["slot"] == ("s" if part == "shared" else "v") + str(r["precision"]), "WORK_MISMATCH")
                    for key, count in build["stages"].items(): shared_stages[key] += count
                    if built:
                        fail(build["origin"] == {"call": call_id, "run": ri, "physical_record": r["index"], "phase": part}, "WORK_MISMATCH")
                        built_refs.add(int(bi))
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
                if case.get("status") == "selected" or case.get("disposition") == "retained_selected":
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
    # Every Build is referenced by its building record (C1 build provenance; WORK), as Rust's
    # `builds_seen` and TypeScript's `seenBuilds` require. So no Build outlives its Run, and none
    # names a case whose native stage failed before any Run (R-D38 (4b), DESIGN_v2 §2).
    fail(built_refs == set(range(len(body["builds"]))), "WORK_MISMATCH")
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
        group = _ref(body["groups"], run["origin"]["group"]) if run["origin"]["group"] is not None else None
        imported = {(x["slot"], int(x["build"])) for x in (group or {}).get("imports", [])}
        for snapshot in [run["cache_before"], run["cache_after"]]:
            slots = [x["slot"] for x in snapshot]
            order = ["s128", "s256", "s512", "s1024", "v256", "v512", "v1024"]
            fail(slots == sorted(set(slots), key=order.index), "WORK_MISMATCH")
            for entry in snapshot:
                b = _ref(body["builds"], entry["build"])
                if b is None:
                    wf(False)
                    continue
                fail(b["slot"] == entry["slot"] and b["state"] != "budget_failure" and (b["group"] == run["origin"]["group"] or (entry["slot"], int(entry["build"])) in imported)
                     and b["origin"]["run"] <= run["id"], "WORK_MISMATCH")
    _g5_combination_native(body, runs, fail)


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


def _d38_capture_before_run(a, case, ai):
    """R-D38 (4b) (DESIGN_v2 §2; RR:8823; C1:103, C3:167): a native capture failure before any Run,
    beside a registered prepared source (C2 §3 registers a CaseSource once it is constructed and its
    maps validate, whatever the outcome). G5 PRODUCT_ATTEMPT. The conjuncts here: the case has no Run
    and the attempt no proof; an unavailable `capture` result; the stage record done(1, [failed]),
    preparation completed, native failed and every later stage not_entered; the case unavailable with
    `prepared_product_failure` naming this attempt and reason (source_unavailable, preparation), D4d's
    mapping for a capture with no Run; and a non-null source reference equal to the case's own
    ([r01: N-6]; TS already requires it). The caller's branch gives `run_ref` null.

    The rest of (4b) holds for every receipt elsewhere: `_g5_products` resolves a non-null source
    reference to a CaseSource whose preparation binds this attempt; G3 binds `execution_order` to
    the cases' Runs; G5's native class binds every Call position to its Run's own case and source,
    every Group source to its Call, and every Build to its building record (`_g5_native_checks`)."""
    st = a["stages"]
    reason = case.get("reason") or {}
    cause = reason.get("cause") or {}
    return (case.get("run") is None and a["proof"] is None
            and a["result"]["kind"] == "unavailable" and a["result"]["error"]["kind"] == "capture"
            and st["preparation"] == "completed" and st["native"] == "failed"
            and all(st[k] == "not_entered" for k in STAGE_ORDER[2:])
            and case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure"
            and cause.get("product_attempt_ref") == ai
            and (reason.get("code"), reason.get("phase")) == ("source_unavailable", "preparation")
            and a["source_ref"] is not None and a["source_ref"] == case.get("source_ref"))


def _g5_stages(a, case, fail, ai):
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
    elif st["native"] == "failed" and a["run_ref"] is None:
        # R-D38 (DESIGN_v2 §2; RR:8823): an entered native stage has a Run (4a, below), except a
        # native capture failure before any Run beside a registered prepared source (4b).
        fail(_d38_capture_before_run(a, case, ai))
    else:
        fail(a["run_ref"] is not None and run is not None and (st["native"] == "completed") == (run["kernel_terminal"]["kind"] == "selected"))
    fail((st["preparation"] == "completed") == (a["source_ref"] is not None) or st["preparation"] == "not_entered" and a["source_ref"] is None)
    _g5_proof_stages(st, proof, fail)


def _g5_proof_stages(st, proof, fail):
    """The proof stages' rules, shared by case and combination attempts (C3:196-201, 253-257)."""
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


def _g5_typed(a, fail, order=STAGE_ORDER, records=None):
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
    fail(tuple(st[k] for k in order) in (ERROR_STAGE_RECORDS if records is None else records).get(result["error"]["kind"], ()))


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
# C2's cause table (the alignment set, item 3): a receipt failure's codes, as a set (C2 does not key them to
# `check`); an unavailable precondition's code, keyed one to one by `precondition`.
RECEIPT_FAILURE_CODES = ("receipt_encoding", "publication_hash_range", "invocation_not_representable")
PRECONDITION_CODES = {"caller": "caller_not_qualified", "resource_admission": "resource_admission_not_available",
                      "upstream_no_wrap": "upstream_no_wrap_not_established", "capture": "source_unavailable",
                      "source_family": "source_unavailable"}


def _g5_ordinary(body, cases, diags, quality, gates=None):
    """G5 class 2 ordinary pass (C3:304; D3, D6): checklist O2-O5 (C2:149-161; S06:51), plus the
    ordinary-list, not_attempted, report, not_required, selected-quality and rcond checks that
    previously ran after the product WORK list (RV79-S2)."""
    fail = lambda ok: _need(ok, "G5", "ATTEMPT_MISMATCH")
    by_id = {d["id"]: d for d in diags}
    ids = set(by_id)
    for i, c in enumerate(cases):
        o = body["ordinary_attempts"][i]
        # The ordinary attempt's material basis resolves to a basis that lists its case (the alignment set,
        # item 1; TS `ordinaryAttempts`), the ordinary class's reference rule, as D4b is the product class's.
        basis = _ref(body["material_bases"], o["material_basis_ref"])
        fail(basis is not None and i in basis["case_indices"])
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
        # RV108 N2: a quality case without `solve_quality` has no published verdict. Each rule
        # below that reads the verdict then fails with its own G5 ATTEMPT code, as Rust's null and
        # TypeScript's undefined do, never through the fail-closed fallback's PRODUCT_ATTEMPT code.
        # A case for which no rule here reads the verdict passes this pass unchanged, and G7's base
        # header then refuses the missing member (SOURCE_NUMERICAL_CASE_INVALID), as before.
        verdict = quality[i]["solve_quality"] if "solve_quality" in quality[i] else None
        if o["initial"]["kind"] == "report":
            fail(o["initial"]["report_diagnostic_ref"] in by_id and o["initial"]["outcome"] == verdict)
        if c["status"] == "not_required":
            fail(c["product_attempt_ref"] is None and verdict == "checks_passed")
        if c["status"] == "selected":
            # D6b (C1:101; C2:153, :164; source_receipt.rs `OrdinaryAttempt::wire`): only an attempted trigger selects.
            fail(verdict in ("sensitive", "unresolved", "failed"))
            fail(c["selection"]["rcond_label"] == RCOND_LABEL)
        cause = c["reason"]["cause"] if c["status"] == "unavailable" else None
        if cause is not None and cause.get("kind") != "prepared_product_failure":
            # C2's cause table (CONTRACT_DELTA:72-74), for every unavailable case whose cause is not a
            # prepared product failure (the alignment set, item 3; TS `ordinaryAttempts`, with the
            # precondition keying). D4d's table covers prepared_product_failure in the product class.
            phase, code, run = c["reason"]["phase"], c["reason"]["code"], c.get("run")
            if cause.get("kind") == "source_error":
                fail(phase == "preparation" and code == "source_unavailable" and run is None
                     and c.get("source_decline") is not None and _same(c["source_decline"]["error"], cause["error"]))
            elif cause.get("kind") == "receipt_failure":
                fail(phase == "receipt" and code in RECEIPT_FAILURE_CODES)
            elif cause.get("kind") == "facade_failure":
                fail(phase == "facade" and code == "facade_certificate" and run is not None and run["kernel_terminal"]["kind"] == "selected"
                     and _same(cause["owner_ref"], {"kind": "case", "index": i}))
            elif cause.get("kind") == "unavailable_precondition":
                fail(phase in ("routing", "preparation") and run is None and code == PRECONDITION_CODES.get(cause["precondition"]))
            else:
                # A kernel reason: the cause is the case's own Run's terminal reason.
                fail(phase == "kernel" and run is not None and code == "kernel_" + run["kernel_terminal"]["kind"]
                     and _same(cause, run["kernel_terminal"]["reason"]))
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
    if body.get("combinations"):
        _g5_dispositions(body, cases, diags, gates, fail)


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
        if a["owner_ref"]["kind"] == "combination":
            _g5_combination_attempt(body, a, ai, rows_by_case, fail, wf)
            continue
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
            fail(source["preparation"] is not None and source["preparation"].get("attempt_ref") == ai)  # D4a (C3:146-148)
        pm = a["preparation"]["members"]
        old, new = a["operational"]["old"], a["operational"]["new"]
        _g5_preparation_members(a, a["operational"]["old_coverage"] != "captured_prefix" or (not pm and not new and a["source_ref"] is None and a["run_ref"] is None and a["result"]["kind"] == "unavailable"), fail, wf)
        proof = a["proof"]
        _g5_proof_rows(a, rows_by_case[case["basis_ref"]["ref_id"]] if proof is not None else None, fail, wf)
        _g5_coverage(a, case, source, fail)
        _g5_stages(a, case, fail, ai)
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
        if a["owner_ref"]["kind"] == "combination":
            _g5_typed(a, fail, COMBINATION_STAGE_ORDER, COMBINATION_ERROR_STAGE_RECORDS)
        else:
            _g5_typed(a, fail)
    _g5_combination_entries(body, fail)
    _g5_operand_preparations(body, fail, wf)
    for ok in work_checks:
        _need(ok, "G5", "WORK_MISMATCH")


def _g5_preparation_members(a, prefix_ok, fail, wf):
    """C3's preparation-stage member rules (a case attempt's or an operand preparation's): the member-prefix order
    against the operational tuples, the captured prefix (`prefix_ok`), and each member's conversions (PRODUCT_ATTEMPT),
    with the conversion count equation (WORK)."""
    pm = a["preparation"]["members"]
    old, new = a["operational"]["old"], a["operational"]["new"]
    fail(len(new) <= len(pm) <= len(old))
    fail([x["member"] for x in pm] == [x["member"] for x in old[:len(pm)]])
    fail([x["member"] for x in new] == [x["member"] for x in pm[:len(new)]])
    fail(prefix_ok)
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


def _g5_proof_rows(a, rows, fail, wf):
    """The proof's lanes and its projection outcomes against the owner's own rows (case or combination)."""
    proof = a["proof"]
    if proof is None:
        return
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


def _g5a_coverage(body, case, source, s, need, data_sources=None):
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
        # A combination's data flags come from each operand term with a nonzero factor (DEF-C `lanes.loads`).
        if any(t["dof"]["node"] in b["nodes"] and (t["dof"]["node"], t["dof"]["component"]) in free and from_bits(t["value"]) != 0
               for d in (data_sources if data_sources is not None else [source]) for t in d["nodal_terms"]):
            need(has_data[b["body"]])


def _g5b_exact_evidence(evidence, case, source):
    """G5b on the exact branch (B3-D §1.4 and §6.2; D2 §4.9.3 G5b): the selected case's own exact_cases entry,
    located by load_case_id, states the prepared section bit for bit. For each source member: As_m2 is the area,
    Z_m3 the section modulus, I_m4 and J_m4 the actual second and polar moments, ro_m the actual radius, and
    outside_diameter_m and effective_wall_thickness_m the normalized OD and effective wall."""
    need = lambda ok: _need(ok, "G5b", "SECTION_MISMATCH")
    entries = evidence.get("exact_cases") if type(evidence) is dict else None
    need(type(entries) is list)
    owner = [e for e in entries if type(e) is dict and e.get("load_case_id") == case["basis_ref"]["ref_id"]]
    need(len(owner) == 1 and type(owner[0].get("pipe_sections")) is list)
    for term in source["section_terms"]:
        member = next(m for m in source["id_maps"]["members"] if m["kernel_member"] == term["member"])
        stated = [p for p in owner[0]["pipe_sections"] if type(p) is dict and p.get("pipe_id") == member["id"]]
        need(len(stated) == 1)
        geometry = term["geometry"]
        for word, key in ((term["area"], "As_m2"), (term["section_modulus"], "Z_m3"), (geometry["actual_second_moment"], "I_m4"),
                          (geometry["actual_polar_moment"], "J_m4"), (geometry["actual_radius"], "ro_m"),
                          (geometry["normalized_od"], "outside_diameter_m"), (geometry["effective_wall"], "effective_wall_thickness_m")):
            value = stated[0].get(key)
            need(type(value) in (int, float) and math.isfinite(value) and bits(float(value)) == word)


def _g5_numeric(body, rows_by_case, phase=None, exact_evidence=None):
    """G5a, then G5b, then G5c, each across all cases in case order. On the exact branch `exact_evidence` is a
    one-tuple holding the envelope's contract_evidence (present or not), for G5b's evidence cross-check.

    C3_DELTA s4 keeps C1's gate order G0..G8 ("within a gate ... ascending attempt
    index; first failure wins") and C1 s6 has every reader execute G0->G8 in the same
    order, so every case's G5a precedes any case's G5b (reader parity rule).
    """
    classes = []
    states = []
    # CONTRACT §10.1 G5a-G5c: a retained combination is one more numeric owner, after the cases, on its representative
    # source's maps and sections (REVISION_01 §3.3 N-9), with its own rows, Selection and prescribed DOFs.
    owners = [("case", c) for c in body["cases"]] + [("combination", c) for c in body.get("combinations") or [] if c["disposition"] in RETAINED_DISPOSITIONS]
    for kind, case in owners:
        if (case["status"] if kind == "case" else case["disposition"]) not in ("selected", "retained_selected"):
            # I57 s4: an unavailable attempt that keeps a complete vector still meets the
            # structural/source-consistency checks; no Selection or selected pass condition.
            ai = case.get("product_attempt_ref")
            a = None if ai is None else body["product_attempts"][int(ai)]
            if (kind == "combination" or case["status"] == "unavailable") and a is not None and a["proof"] is not None and a["proof"]["summary_coverage"] is not None:
                source, data = _numeric_source(body, kind, a["source_ref"])
                _g5a_coverage(body, case, source, None,
                              lambda ok, suffix="SCALE_MISMATCH": _need(ok, "G5a", suffix), data)
            continue
        source, data = _numeric_source(body, kind, case["source_ref"]); s = case["selection"]
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
        _g5a_coverage(body, case, source, s, need, data)
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
        states.append((case, source, s, rows, bodies, names, values, extents, raw_scales, deferred_class_checks))
    if phase is not None: phase[0] = "G5b"
    for case, source, s, rows, bodies, names, values, extents, raw_scales, deferred_class_checks in states:
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
    if exact_evidence is not None:
        # DESIGN §6.2's G5b row: the shared checks over every case first, then each selected case's exact evidence,
        # as RS's `g5b_exact_evidence` and TS run it (RV120 F1).
        for case, source, *_ in states:
            if "status" in case: _g5b_exact_evidence(exact_evidence[0], case, source)
    if phase is not None: phase[0] = "G5c"
    for *_, deferred_class_checks in states:
        for ok, code in deferred_class_checks:
            _need(ok, "G5c", code)
    return classes + _r_comb_1(body, rows_by_case)


def _pressure_contract_is(model, expected):
    """Exactly the JSON object `expected`: the two keys, each value that exact string (B3-D REVISION_01 §3)."""
    value = model.get("pressure_contract")
    return type(value) is dict and value.keys() == expected.keys() and all(value[k] == expected[k] for k in expected)


def _legacy_namespace(model):
    """G8's namespace on the preview successor, type-strict (B3-D REVISION_01 §3, N-4): branch L, schema 0.1.0 or
    0.2.0 with `pressure_contract` absent or JSON null. Anything else is refused: any 0.3.0 model, the retired
    {"version": "1.0.0", "mode": "legacy_pressure_v1"} included (B3a's branch L3 is dropped; the owner retired that
    contract product-wide), and B3D-10's tightenings (0.3.0 without a contract; `{}` and other falsy values) stay."""
    version = model.get("schema_version")
    return type(version) is str and version in ("0.1.0", "0.2.0") and model.get("pressure_contract") is None


EXACT_PRESSURE_CONTRACT = {"version": "2.0.0", "mode": "exact_straight_pressure_v2"}


def _exact_namespace(model):
    """G8's namespace on the exact branch (B3-D §4.2 D1.3 branch E; REVISION_01 §3, type-strict): schema 0.3.0 with
    exactly {"version": "2.0.0", "mode": "exact_straight_pressure_v2"}. 0.4.0 and load states stay out."""
    version = model.get("schema_version")
    return type(version) is str and version == "0.3.0" and _pressure_contract_is(model, EXACT_PRESSURE_CONTRACT)


def _g8_exact_materials(body, source, invocation, case_bases):
    """G8 on the exact branch, after the material bases (B3-D §6.2; REVISION_01 §4.2 steps 5 and 6). S-C: physics-
    source-1's actual-material check over every exact_cases entry, through its own `_actual_materials` and
    `_canonical_inputs`, used as they are (B3D-12; N-9), with its code as detail only (B3D-13). N-6: each entry's
    published G_pa is, bit for bit, the receipt's shear modulus for that material."""
    from .physics_source import _actual_materials, _canonical_inputs
    model = invocation["request"]["model"]
    entries = source["contract_evidence"]["exact_cases"]
    try:
        canonical = _canonical_inputs(invocation)
        for entry in entries:
            actual = next(c for c in model["load_cases"] if c["id"] == entry["load_case_id"])
            _actual_materials(invocation, actual, entry, canonical)
    except ValueError as exc:
        raise RetainedPrecisionError("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH", str(exc)) from exc
    ids = [c["id"] for c in model["load_cases"]]
    for entry in entries:
        basis = body["material_bases"][case_bases[ids.index(entry["load_case_id"])]]
        for published in entry["pipe_materials"]:
            receipt = [m for m in basis["materials"] if m["id"] == published["material_id"]]
            _need(len(receipt) == 1 and type(published["G_pa"]) in (int, float) and math.isfinite(published["G_pa"])
                  and bits(float(published["G_pa"])) == receipt[0]["shear_modulus"], "G8", "PREPARATION_MISMATCH")


def _g8(body, source, invocation, route=PREVIEW_ROUTE):
    need = lambda ok, code="PREPARATION_MISMATCH": _need(ok, "G8", code)
    exact = route.exact
    # The invocation is exactly {request, solver_mode} with a known solver mode (I91 repair 01, findings d1
    # and d2), as Rust's `g8` (its first two `need`s) and TS's `invocationBinding` (its first `fail`) require.
    need(type(invocation) is dict and set(invocation) == {"request", "solver_mode"}
         and invocation["solver_mode"] in ("sparse_interactive", "dense_scrutiny"), "INVOCATION_MISMATCH")
    try: need(_hash("source_blocks_invocation_v1", invocation) == body["invocation"]["value"], "INVOCATION_MISMATCH")
    except RetainedPrecisionError: raise
    except (ValueError, RuntimeError): need(False, "INVOCATION_MISMATCH")
    request = invocation["request"]; model = request["model"]
    need(model["project"]["id"] == source["model_ref"], "INVOCATION_MISMATCH")
    # The model scope, as PP accepts it (the alignment set, item 2): no reference_configurations member (null
    # included); the route's namespace (branch L; B3b's exact one); combinations and components absent or [] (on the
    # exact route a combination is ruling 4's expected refusal).
    # B2-C §10.1 G8 invocation (REVISION_01 §4.1 #12): on the preview route model combinations are admitted, the entries
    # being the invocation's in order, id and expression. The exact route still admits none (B3b; RR "PR-B1 cut …", 2).
    need((_exact_namespace(model) if exact else _legacy_namespace(model)) and (model.get("combinations", []) == [] if exact else _model_combinations_match(body, model)), "INVOCATION_MISMATCH")
    need(model.get("components", []) == [] and "reference_configurations" not in model, "INVOCATION_MISMATCH")
    nodes, pipes, supports = model["nodes"], model["pipe_segments"], model["supports"]
    need(len({x["id"] for x in nodes}) == len(nodes) and len({x["id"] for x in pipes}) == len(pipes) and len({x["id"] for x in supports}) == len(supports))
    materials = request.get("materials") or model.get("materials", [])
    from core.units.adapter import convert_quantities_to_canonical
    def unit(q, dimension):
        values = convert_quantities_to_canonical([{"id":"v","value":q["value"],"unit":q["unit"],"dimension":dimension}])
        return float(values[0]["value"])
    def poisson(material):
        q = material.get("poisson_ratio")
        need(type(q) is dict and q.get("unit") == "1" and type(q.get("value")) in (int, float) and math.isfinite(q["value"]) and -1 < q["value"] < .5)
        return float(q["value"])
    def shear_origin(material):
        # B3-D §1.2 (B3D-7): on the exact route G is derived from E and nu; otherwise the authored G.
        return {"kind":"derived_e_nu","poisson_ratio":bits(poisson(material)),"constitutive_basis":"homogeneous_isotropic_E_nu_v1"} if exact else {"kind":"explicit_g"}
    def selected_material(material, case):
        if exact:
            # D1.5-exact: the base common E/nu only. G_hat is the producer's RN64(E/(2*RN64(1+nu))), positive and
            # normal (B3-D §1.2, B3D-7; RV116 N-1); binary64 evaluates exactly that expression.
            need(case.get("modulus_basis_ref") is None and case.get("modulus_basis_temperature") is None)
            e = unit(material["elastic_modulus"], "stress"); g = e / (2.0 * (1.0 + poisson(material)))
            need(math.isfinite(e) and e > 0 and math.isfinite(g) and g >= 2.0 ** -1022)
            return (e, g), {"kind":"base"}
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
    # DESIGN_v2 §3.3 (decision 9) and F-1 text B (§3.2, decision 8): after the invocation, project and
    # model-scope checks, one loop in request order over every case (selected, unavailable or
    # not_required: every case's rows come from the one ordinary run). G3 binds case i to ordinary
    # attempt i and to the invocation's case i. Each check is PREPARATION_MISMATCH.
    mode = invocation["solver_mode"]
    mode_code = {"sparse_interactive": 1, "dense_scrutiny": 2}[mode]
    selectors, case_bases = [], []
    for i, case in enumerate(body["cases"]):
        o = body["ordinary_attempts"][i]
        # 1. The requested mode.
        need(o["requested_mode"] == mode)
        # 2. The ordinary attempt's material basis is its case's selector, numbered in first-seen order
        # over the request's cases (I91 repair 01, finding b), as Rust's `g8` (`case_bases`) and TS's
        # `invocationBinding` (`knownSelectors`) number it. This binds every case, sourced or not.
        raw_case = model["load_cases"][i]
        named, temperature = raw_case.get("modulus_basis_ref"), raw_case.get("modulus_basis_temperature")
        need(named is None or temperature is None)
        selector = {"kind":"named","id":named} if named is not None else {"kind":"temperature","kelvin":bits(unit(temperature,"temperature"))} if temperature is not None else {"kind":"base"}
        need(not exact or selector == {"kind":"base"})  # B3-D REVISION_01 §4.2 step 3
        if selector not in selectors: selectors.append(selector)
        case_bases.append(selectors.index(selector))
        need(o["material_basis_ref"] == case_bases[i])
        case_rows = [r for r in source["results"] if r["basis_ref"]["ref_id"] == case["basis_ref"]["ref_id"]]
        modes = [r for r in case_rows if r["kind"] == "linear_solver_mode_basis"]
        # 3. P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        need(len(modes) == 1 and type(modes[0]["value"]) in (int, float) and modes[0]["value"] == mode_code)
        parity = sum(1 for r in case_rows if r["kind"] == "sparse_live_path_dense_parity_relative_delta")
        # 4. P2: at most one parity row. P3: none in sparse_interactive. P4: none when W2 published
        # (b != 0; OQ5). A dense b = 0 case may lack it (a failed observation lane); that deletion
        # is the disclosed limit.
        need(parity <= 1 and (parity == 0 or mode == "dense_scrutiny") and (parity == 0 or o["w2"]["kind"] != "published"))
    # One material basis per distinct selector, in first-seen order, each listing exactly the cases
    # that use it (I91 repair 01, finding c), as Rust's `g8` and TS's `invocationBinding` require.
    need(len(body["material_bases"]) == len(selectors))
    used = {p["material"] for p in pipes}
    for mi, mb in enumerate(body["material_bases"]):
        need(mb["selector"] == selectors[mi] and mb["case_indices"] == [i for i, k in enumerate(case_bases) if k == mi])
        # Every basis's materials, whether or not a CaseSource uses it (I91 repair 01, finding e): the used
        # materials in input order, each selected for the basis's cases, as Rust's `g8` (its material
        # bases loop) and TS's `invocationBinding` (`b.material_bases.forEach`) check them. A basis used
        # only by cases without a source (a not_required case) was never checked before.
        need([m["input_index"] for m in mb["materials"]] == [j for j, m in enumerate(materials) if m["id"] in used])
        for m in mb["materials"]:
            raw = materials[int(m["input_index"])]; pair, selection = selected_material(raw, model["load_cases"][mb["case_indices"][0]])
            need(m["id"] == raw["id"] and m["selection"] == selection and m["shear_origin"] == shear_origin(raw) and [m["elastic_modulus"],m["shear_modulus"]] == [bits(v) for v in pair])
    if exact: _g8_exact_materials(body, source, invocation, case_bases)
    for si, s in enumerate(body["sources"]):
        if s["owner"]["kind"] == "combination":
            continue  # its K4CMB and operand equality follow (_g8_combinations)
        # The source's index and owner are G3's (the alignment set, item 1); here, the invocation's facts.
        for include_loads, field in ((True, "kernel_source_sha256"), (False, "stiffness_sha256")):
            need(hashlib.sha256(_native_source_encoding(s, include_loads)).hexdigest() == s[field])
        ci = int(s["owner"]["case_index"]); case = model["load_cases"][ci]
        if exact:
            # D1.5-exact (B3-D §4.2; REVISION_01 §4.2 step 8): pressure_regions present and [], no equivalent_static,
            # no analysis_state.
            need(type(case.get("pressure_regions")) is list and case["pressure_regions"] == [] and case.get("equivalent_static") is None and "analysis_state" not in case)
        else:
            # The sourced case on the preview route (DOMAIN D1.5; C1's G8 row, "no 0.4 extension"), aligned in the three
            # readers (I100 B3 addendum 01): pressure_regions absent, null or [] (B3D-11's leniency), type-strict;
            # equivalent_static absent or null; no analysis_state member, null included (the 0.4.0 load-reference
            # state, which D1.5 requires Absent). A key PP's typed case does not have (a case-level `pressure`) is not read.
            regions = case.get("pressure_regions")
            need((regions is None or (type(regions) is list and regions == [])) and case.get("equivalent_static") is None
                 and "analysis_state" not in case)
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
            need(m["id"] == raw["id"] and m["selection"] == selection and m["shear_origin"] == shear_origin(raw) and [m["elastic_modulus"],m["shear_modulus"]] == [bits(v) for v in pair])
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
            need(geo["route"] == ("exact" if exact else "preview") and geo["normalized_od"] == bits(d) and geo["effective_wall"] == bits(wall-tolerance) and 0 < wall-tolerance < d*.5)
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
        if s["preparation"] is not None and "operand_preparation_ref" in s["preparation"]:
            # C3a-7 G8: an operand-prepared source binds its OperandPreparation as a case source binds its attempt.
            a=_records(body)[int(s["preparation"]["operand_preparation_ref"])];need(a["source_ref"]==si and s["preparation"]["sha256"]==_hash("retained_precision_operand_preparation_v1",_operand_preparation_payload(a,route.definition_hash)))
        elif s["preparation"] is not None:
            a=body["product_attempts"][int(s["preparation"]["attempt_ref"])];need(a["source_ref"]==si and s["preparation"]["sha256"]==_hash("retained_precision_preparation_v1",_preparation_payload(a,route.definition_hash)))
        if s["preparation"] is not None:
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
    for a in body["product_attempts"] + _records(body):
        if a["owner_ref"]["kind"] != "case":
            continue  # a CombinationAttempt has no preparation (CONTRACT §4); operand preparations bind as cases' (C3a-7)
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
    _g8_combinations(body, need)


# ---------------------------------------------------------------------------------------------------------------
# B2 (B2-C, final for J1: CONTRACT with REVISION_01 and REVISION_02): model combinations, operand preparations and
# their gates. At z = 0 (no combination entry, no operand preparation, no CombinationSource) each check below is
# vacuous, so every existing predicate is today's (REVISION_01 §4.1).

# REVISION_02 §2.2: a combination's displacement magnitudes are formed in the projection's second pass, not hull-projected.
COMBINATION_HULL_EXCLUDED = HULL_EXCLUDED | {"displacement_magnitude"}
# CONTRACT §4: a CombinationAttempt has no preparation stage; its stage records are a case attempt's without it.
COMBINATION_STAGE_ORDER = STAGE_ORDER[1:]
COMBINATION_ERROR_STAGE_RECORDS = {kind: frozenset(r[1:] for r in records if r[0] == "completed")
                                   for kind, records in ERROR_STAGE_RECORDS.items() if kind != "preparation"}


def _resolve(items, ref):
    """A strict index that resolves, else None (the caller's own check reports it)."""
    i = _integral(ref)
    return items[i] if i is not None and 0 <= i < len(items) else None


def _records(body):
    """`operand_preparations[]`, absent when empty (C-6)."""
    records = body.get("operand_preparations")
    return records if type(records) is list else []


def _referenced_ids(expression):
    """The case ids a combination's expression names (CONTRACT §2.7's three expression kinds)."""
    if expression["kind"] == "mechanics":
        return [t["case_id"] for t in expression["terms"]]
    if expression["kind"] == "result_state_subtraction":
        return [expression["minuend_id"], expression["subtrahend_id"]]
    return list(expression["operand_ids"])


def _cause_kind(entry):
    reason = entry.get("reason")
    return (reason.get("cause") or {}).get("kind") if type(reason) is dict else None


def _needs_operands(entry):
    """C3a-1: a retained mechanics combination not decided at T-10b(i) (`operand_source_unavailable`)."""
    return (entry["disposition"] in RETAINED_DISPOSITIONS and entry["expression"]["kind"] == "mechanics"
            and _cause_kind(entry) != "operand_source_unavailable")


def _source_unusable(case):
    """CONTRACT §2.4 (i), REVISION_01 §3.3 N-1: an `unavailable` operand case with no CaseSource, or whose Run refused
    `ledger_unavailable`, has no usable operand source."""
    run = case.get("run")
    ledger = run is not None and run["kernel_terminal"]["kind"] == "refused" and (run["kernel_terminal"]["reason"] or {}).get("tag") == "ledger_unavailable"
    return case["status"] == "unavailable" and (case.get("source_ref") is None or ledger)


def _operand_source_ref(body, ci):
    """CONTRACT §2.5: a term's operand source, a `selected` or `unavailable` case's own CaseSource, or a `not_required`
    case's prepared operand-preparation source; None when there is none."""
    case = body["cases"][ci]
    if case["status"] in ("selected", "unavailable"):
        return case.get("source_ref")
    record = next((r for r in _records(body) if r["owner_ref"]["index"] == ci and r["result"]["kind"] == "prepared"), None)
    return None if record is None else record["source_ref"]


def _operand_preparation_payload(record, definition_hash):
    """C3a-5: H(`retained_precision_operand_preparation_v1`, {definition_id, definition_sha256 (DEF-O's table-bound H),
    owner_ref, ordinary_attempt_ref, material_basis_ref, purpose, members})."""
    return dict(_preparation_payload(record, definition_hash), purpose=record["purpose"])


def _g1_combinations(body, route):
    """CONTRACT §10.1 G1 (RECEIPT_MISMATCH), on already-addressable records of the named kind only (S-6 steps 2, 4 and
    5; §2.7's hashes): a `retained_selected` combination's identity over its CombinationSource, each CombinationSource
    operand's identity over the CaseSource at its `source_ref`, and each operand-prepared CaseSource's preparation hash
    when its record and every member are prepared. A reference to a source of the other kind is G3's or G5's."""
    need = lambda ok: _need(ok, "G1", "RECEIPT_MISMATCH")
    sources = body["sources"]
    for entry in body["combinations"]:
        if entry["disposition"] == "retained_selected":
            source = _resolve(sources, entry["source_ref"])
            if source is not None and source["owner"]["kind"] == "combination": need(entry["source_identity_sha256"] == _source_hash(source))
    for source in sources:
        if source["owner"]["kind"] == "combination":
            for operand in source["operands"]:
                case_source = _resolve(sources, operand["source_ref"])
                if case_source is not None and case_source["owner"]["kind"] == "case": need(operand["source_identity_sha256"] == _source_hash(case_source))
        elif source["preparation"] is not None and "operand_preparation_ref" in source["preparation"]:
            record = _resolve(_records(body), source["preparation"]["operand_preparation_ref"])
            if record is not None and record["result"]["kind"] == "prepared" and all(m["result"]["kind"] == "prepared" for m in record["preparation"]["members"]):
                need(source["preparation"]["sha256"] == _hash("retained_precision_operand_preparation_v1", _operand_preparation_payload(record, route.definition_hash)))


def _g3_combination_attempt(body, a, ai, crows):
    """REVISION_01 §4.1 #3: a CombinationAttempt's owner is its combination entry, its projection outcomes index that
    entry's own rows, and a complete summary roster lists operand 0's CaseSource bodies (a CombinationSource has none).
    The member, captured-prefix and inventory checks are a case attempt's only."""
    need = lambda ok: _need(ok, "G3", "COVERAGE_MISMATCH")
    combos = body["combinations"]
    k = _integral(a["owner_ref"]["index"])
    need(a["id"] == ai and k is not None and k < len(combos) and combos[k].get("product_attempt_ref") == ai)
    if a["proof"] is not None:
        rows = crows[combos[k]["basis_ref"]["ref_id"]]
        indices = [x["row_index"] for x in a["proof"]["projection_outcomes"]]
        need(indices == sorted(set(indices)) and all(x < len(rows) and rows[int(x)]["kind"] not in COMBINATION_HULL_EXCLUDED for x in indices))
        coverage, source = a["proof"]["summary_coverage"], _resolve(body["sources"], a["source_ref"])
        operands = source["operands"] if source is not None and source["owner"]["kind"] == "combination" else []
        representative = _resolve(body["sources"], operands[0]["source_ref"]) if operands else None
        if coverage is not None and representative is not None and representative["owner"]["kind"] == "case":
            inventory = [b["body"] for b in representative["body_membership"]]
            need(bool(inventory) and [x["body"] for x in coverage] == inventory == list(range(len(inventory))))


def _g3_combinations(snapshot, body, ids, crows):
    """CONTRACT §10.1 G3 (a) to (e) and (g), REVISION_01 §4.2 (h) and (i), C3a-7's coverage with REVISION_01 §4.3
    (a record's `source_ref` is read only when it is prepared), after the existing checks. COVERAGE_MISMATCH."""
    need = lambda ok: _need(ok, "G3", "COVERAGE_MISMATCH")
    cases, combos, sources, attempts, records = body["cases"], body["combinations"], body["sources"], body["product_attempts"], _records(body)
    cids = [c["basis_ref"]["ref_id"] for c in combos]
    case_at = {cid: i for i, cid in enumerate(ids)}
    # (a) One entry per gate-evidence entry, in order and by id. It binds once the receipt has an entry or an
    # `operand_preparations` member (PR-B2 ruling 1; REVISION_01 §4.1: today's predicate at z = 0): with neither, the
    # receipt is today's (07n's `t_gate_*` entries carry gate evidence for a combination outside the model).
    b2 = bool(combos) or "operand_preparations" in body
    if b2:
        gates = (snapshot.get("contract_evidence") or {}).get("combination_gates")
        need(type(gates) is list and len(gates) == len(combos) and all(type(g) is dict and g.get("combination_id") == cid for g, cid in zip(gates, cids)))
    # (b) Case ids and combination ids are one id set.
    need(len(set(ids) | set(cids)) == len(ids) + len(cids))
    # (c) A combination's result_ids are exactly its rows, in publication order (every row's basis resolved above).
    for entry, cid in zip(combos, cids):
        need(entry["result_ids"] == [row["id"] for row in crows[cid]])
    # (d) C3a's coverage: id = position; the owner not_required, at most one record per owner; requested_by ascending
    # and unique, each naming a retained combination that needs operands and names the owner; conversely every
    # not_required term case of such a combination has exactly one record listing it.
    for ri, record in enumerate(records):
        owner = _integral(record["owner_ref"]["index"])
        need(record["id"] == ri and owner is not None and owner < len(cases) and cases[owner]["status"] == "not_required")
        requested = record["requested_by"]
        need(requested == sorted(set(requested)))
        for k in requested:
            entry = _resolve(combos, k)
            need(entry is not None and _needs_operands(entry) and ids[owner] in _referenced_ids(entry["expression"]))
    owners = [record["owner_ref"]["index"] for record in records]
    need(len(set(owners)) == len(owners))
    for k, entry in enumerate(combos):
        if _needs_operands(entry):
            for cid in dict.fromkeys(_referenced_ids(entry["expression"])):
                ci = case_at.get(cid)
                if ci is not None and cases[ci]["status"] == "not_required":
                    need(sum(1 for record in records if record["owner_ref"]["index"] == ci and k in record["requested_by"]) == 1)
    # A prepared record's source is a CaseSource of its owner naming this record, and every operand-prepared
    # CaseSource is named by exactly one prepared record.
    prepared = []
    for ri, record in enumerate(records):
        if record["result"]["kind"] == "prepared" and record["source_ref"] is not None:
            source = _resolve(sources, record["source_ref"])
            need(source is not None and source["owner"]["kind"] == "case" and source["owner"]["case_index"] == record["owner_ref"]["index"]
                 and type(source["preparation"]) is dict and source["preparation"].get("operand_preparation_ref") == ri)
            prepared.append(int(record["source_ref"]))
    operand_prepared = lambda s: s["owner"]["kind"] == "case" and type(s["preparation"]) is dict and "operand_preparation_ref" in s["preparation"]
    for si, source in enumerate(sources):
        if operand_prepared(source): need(prepared.count(si) == 1)
    # (e) Case attempts first, then combination attempts in authored order, one for each entry with a Run.
    kinds = [a["owner_ref"]["kind"] for a in attempts]
    need(kinds == sorted(kinds, key=lambda kind: kind == "combination"))
    need([a["owner_ref"]["index"] for a in attempts if a["owner_ref"]["kind"] == "combination"] == [k for k, e in enumerate(combos) if e.get("run") is not None])
    need(all((e.get("product_attempt_ref") is not None) == (e.get("run") is not None) for e in combos))
    # (g) Each CombinationSource is named by exactly one combination entry, and each operand-prepared CaseSource by
    # exactly one prepared operand preparation and no case, always. A batch CaseSource is named by exactly one case or
    # one prepared operand preparation once, like (a), the receipt has an entry or an `operand_preparations` member
    # (PR-B2 ruling 1): on a receipt with neither, today's checks place a source no case names (07n's
    # `d38_m8_case_source_other` at G5, `orphan_source_beside_t7` at G8), and no 07n first failure moves (R5).
    for si, source in enumerate(sources):
        by_cases = sum(1 for c in cases if c.get("source_ref") == si)
        if source["owner"]["kind"] == "combination":
            need(sum(1 for e in combos if e.get("source_ref") == si) == 1)
        elif operand_prepared(source):
            need(by_cases == 0 and prepared.count(si) == 1)
        elif b2:
            need(by_cases + prepared.count(si) == 1)
    # (h) Operand preparations in first-need order (C3a-1): combinations in authored order, terms in authored order.
    need_order = []
    for entry in combos:
        if _needs_operands(entry):
            for cid in _referenced_ids(entry["expression"]):
                ci = case_at.get(cid)
                if ci is not None and cases[ci]["status"] == "not_required" and ci not in need_order: need_order.append(ci)
    need(owners == need_order)
    # (i) The batch's CaseSources, then the operand-prepared CaseSources in record order, then the CombinationSources.
    role = [2 if s["owner"]["kind"] == "combination" else 1 if operand_prepared(s) else 0 for s in sources]
    refs = [s["preparation"]["operand_preparation_ref"] for s in sources if operand_prepared(s)]
    need(role == sorted(role) and refs == sorted(refs))


def _g4_combinations(combos, diags):
    """CONTRACT §10.1 G4 (DIAGNOSTIC_MISMATCH): one RETAINED_PRECISION_SELECTED per `retained_selected` and one
    RETAINED_PRECISION_UNAVAILABLE per `retained_unavailable` combination, each naming exactly that combination;
    none for an `ordinary` or `base_withheld` one; an unavailable entry's `diagnostic_ref` is its diagnostic's id."""
    need = lambda ok: _need(ok, "G4", "DIAGNOSTIC_MISMATCH")
    for entry in combos:
        cid, disposition = entry["basis_ref"]["ref_id"], entry["disposition"]
        named = lambda code: [d for d in diags if d["code"] == code and type(d.get("affected_refs")) is list and cid in d["affected_refs"]]
        selected, unavailable = named("RETAINED_PRECISION_SELECTED"), named("RETAINED_PRECISION_UNAVAILABLE")
        need(len(selected) == int(disposition == "retained_selected") and len(unavailable) == int(disposition == "retained_unavailable")
             and all(d["affected_refs"] == [cid] for d in selected + unavailable))
        if disposition == "retained_unavailable": need(unavailable[0]["id"] == entry["diagnostic_ref"])


def _g5_dispositions(body, cases, diags, gates, fail):
    """CONTRACT §10.1 G5 ordinary class (ATTEMPT_MISMATCH), the disposition rule: `base_withheld` iff the aligned gate
    entry is withheld, with its reason, on a mechanics expression; retained iff mechanics with distinct term cases, at
    least one `selected`, and not withheld; otherwise `ordinary`. Then D6a's rule for each entry's diagnostic_refs.
    D6b stays a case rule (REVISION_01 §4.5)."""
    status = {c["basis_ref"]["ref_id"]: c["status"] for c in cases}
    for entry, gate in zip(body["combinations"], gates):
        expression, disposition = entry["expression"], entry["disposition"]
        withheld = gate["withheld"] is True
        if withheld:
            fail(disposition == "base_withheld" and entry["reason"] == gate["reason"] and expression["kind"] == "mechanics")
        else:
            terms = [t["case_id"] for t in expression["terms"]] if expression["kind"] == "mechanics" else None
            trigger = (terms is not None and len(set(terms)) == len(terms) and all(t in status for t in terms)
                       and any(status[t] == "selected" for t in terms))
            fail(disposition in RETAINED_DISPOSITIONS if trigger else disposition == "ordinary")
        cid = entry["basis_ref"]["ref_id"]
        fail(entry["diagnostic_refs"] == [d["id"] for d in diags if isinstance(d.get("affected_refs"), list) and cid in d["affected_refs"]
                                          and not str(d.get("code")).startswith("RETAINED_PRECISION_")])


def _g5_combination_native(body, runs, fail):
    """CONTRACT §10.1 G5 native class (ATTEMPT_MISMATCH): `calls[0]` the case batch, then one mechanics Call per entry
    with a non-null `call_ref`, in authored order, naming it; its requested operands are the expression's terms with
    their §2.5 sources; `runs` with one source and one Run, `pre_source_refusal` with none; the CombinationSource's
    structure; a combination Call's group is a CombinationGroup (and a batch's a Group); its imports come only from
    `selected` operands, the first occupied slot in authored order, each build existing and earlier; `cache_before`
    is the imports, and a record reuses no other group's build. The meter chain and the Run's charges are the
    existing WORK checks."""
    cases, combos, sources, calls, groups = body["cases"], body.get("combinations") or [], body["sources"], body["calls"], body["groups"]
    case_at = {c["basis_ref"]["ref_id"]: i for i, c in enumerate(cases)}
    called = [k for k, e in enumerate(combos) if e.get("call_ref") is not None]
    fail(bool(calls) and calls[0]["kind"] == "case_batch" and len(calls) == 1 + len(called))
    for call, k in zip(calls[1:], called):
        entry = combos[k]
        fail(call["kind"] == "mechanics_combination" and call["owner_refs"] == [{"kind": "combination", "index": k}] and entry["call_ref"] == call["id"])
        fail(entry["expression"]["kind"] == "mechanics")
        terms, requested, result = entry["expression"]["terms"], call["requested_operands"], call["result"]
        refused = result["kind"] == "pre_source_refusal"
        # CONTRACT §2.5: `requested_operands` keeps the authored terms whatever the Call's result, a `no_operands`
        # pre-source refusal included (PR-B2 ruling 2).
        fail(len(requested) == len(terms))
        for term, operand in zip(terms, requested):
            ci = case_at.get(term["case_id"])
            fail(ci is not None and operand["factor"] == term["factor"] and _operand_source_ref(body, ci) is not None and operand["source_ref"] == _operand_source_ref(body, ci))
        if refused:
            # REVISION_01 §4.1 row 8: a refused Call has no source and no Run; the entry's null members are G5 products'
            # (CONTRACT §10.1: a CombinationReason cause with `call_ref` non-null and the rest null).
            fail(call["source_refs"] == [] and call["run_refs"] == [])
            continue
        run = entry.get("run")
        fail(run is not None and call["run_refs"] == [run["id"]] and call["source_refs"] == [entry["source_ref"]])
        combined = _at(sources, entry["source_ref"], code="ATTEMPT_MISMATCH")
        operands = combined["operands"]
        fail(combined["owner"]["kind"] == "combination" and len(operands) == len(terms) and combined["representative_source_ref"] == operands[0]["source_ref"])
        for term, operand, ask in zip(terms, operands, requested):
            fail(operand["case_index"] == case_at.get(term["case_id"]) and operand["factor"] == term["factor"] and operand["source_ref"] == ask["source_ref"])
            case_source = _at(sources, operand["source_ref"], code="ATTEMPT_MISMATCH")
            fail(case_source["owner"]["kind"] == "case" and case_source["owner"]["case_index"] == operand["case_index"]
                 and case_source["stiffness_sha256"] == combined["stiffness_sha256"])
        if entry["disposition"] == "retained_selected":
            fail(combined["ledger_sha256"] == entry["selection"]["ledger_sha256"])
        group = _at(groups, run["origin"]["group"], code="ATTEMPT_MISMATCH")
        imports = []
        for slot in SLOT_ORDER:
            for i, term in enumerate(terms):
                case = cases[case_at[term["case_id"]]]
                held = [x for x in case["run"]["cache_after"] if x["slot"] == slot] if case["status"] == "selected" and case.get("run") is not None else []
                if held:
                    imports.append({"operand_index": i, "selected_run": case["run"]["id"], "slot": slot, "build": held[0]["build"]})
                    break
        fail(group.get("imports") == imports and run["cache_before"] == [{"slot": x["slot"], "build": x["build"]} for x in imports])
        for x in imports:
            build = _ref(body["builds"], x["build"])
            fail(build is not None and build["slot"] == x["slot"] and build["origin"]["run"] < run["id"])
        imported = {(x["slot"], x["build"]) for x in imports}
        for record in run["records"]:
            for field in ("shared_build_ref", "verification_shared_build_ref"):
                build = _ref(body["builds"], record[field]) if record[field] is not None else None
                if build is not None:
                    fail(build["group"] == run["origin"]["group"] or (build["slot"], int(record[field])) in imported)
    for group in groups:
        call = _ref(calls, group["call"])
        fail(call is not None and ("imports" in group) == (call["kind"] == "mechanics_combination"))


def _g5_combination_stages(a, entry, fail):
    """C3's stage rules without a preparation stage (CONTRACT §4): a Run always exists, native completed iff it is
    selected, then the proof stages as a case attempt's."""
    st = a["stages"]
    seen_end = False
    for state in [st[k] for k in COMBINATION_STAGE_ORDER[:7]]:
        if seen_end:
            fail(state == "not_entered")
        elif state != "completed":
            seen_end = True
    fail((st["observables"] == "not_entered") == (st["g5a"] == "not_entered"))
    if st["observables"] != "not_entered":
        fail(st["certificate"] in ("completed", "failed"))
    run = entry.get("run")
    fail(st["native"] != "not_entered" and run is not None and (st["native"] == "completed") == (run["kernel_terminal"]["kind"] == "selected"))
    _g5_proof_stages(st, a["proof"], fail)


def _g5_combination_attempt(body, a, ai, rows_by_case, fail, wf):
    """CONTRACT §4 and §10.1 G5 products for a CombinationAttempt: its entry, Run and CombinationSource; Ready iff
    `retained_selected`; the proof's lanes and projection; the stages; C3's accounting; §4's reason table; and N-5
    (REVISION_01 §3.3): no CaptureError `origin` in its result error's capture, observable or abandoned cause."""
    entry = _at(body["combinations"], a["owner_ref"]["index"])
    fail(a["id"] == ai and entry.get("product_attempt_ref") == ai and entry["disposition"] in RETAINED_DISPOSITIONS)
    run = entry.get("run")
    fail(run is not None and a["run_ref"] == run["id"] and a["source_ref"] == entry.get("source_ref"))
    source = _at(body["sources"], a["source_ref"])
    fail(source["owner"]["kind"] == "combination" and source["owner"]["combination_index"] == a["owner_ref"]["index"])
    ready = a["result"]["kind"] == "ready"
    fail(ready == (entry["disposition"] == "retained_selected"))
    if not ready:
        fail(entry["reason"]["cause"] == {"kind": "prepared_product_failure", "product_attempt_ref": ai})
    rows = rows_by_case[entry["basis_ref"]["ref_id"]]
    _g5_proof_rows(a, rows, fail, wf)
    _g5_coverage(a, entry, source, fail)
    _g5_combination_stages(a, entry, fail)
    for ok in _accounting_rules(dict(a, preparation={"members": []}, operational={"old_coverage": "complete", "old": [], "new": []})): wf(ok)
    selected = run["kernel_terminal"]["kind"] == "selected"
    proof = a["proof"]
    if ready:
        fail(selected and proof is not None and all(v == "completed" for v in a["stages"].values()) and all(v["kind"] == "passed" for v in proof["checks"].values()))
        wf(_exact_work(proof) and a["adapter"]["fault"] is None and not a["g5a_work"]["lost"] and not a["overlay_work"]["lost"])
        expected = [i for i, r in enumerate(rows) if r["kind"] not in NONQUANTITY | COMBINATION_HULL_EXCLUDED]
        fail([x["row_index"] for x in proof["projection_outcomes"]] == expected)
        return
    error = a["result"]["error"]
    # §4's reason table, by the attempt's own error; a preparation error is refused.
    fail(error["kind"] != "preparation")
    if error["kind"] == "native":
        fail(not selected and error["run_ref"] == run["id"])
    kernel = error["kind"] == "native" or (error["kind"] == "capture" and not selected)
    fail(kernel or selected)
    fail((entry["reason"]["code"], entry["reason"]["phase"]) == (("combination_unresolved", "kernel") if kernel else ("facade_certificate", "facade")))
    fail(not (error["kind"] in ("capture", "observable", "abandoned") and error["cause"].get("kind") == "origin"))


def _g5_combination_entries(body, fail):
    """CONTRACT §10.1 G5 products (PRODUCT_ATTEMPT_MISMATCH), the `retained_unavailable` reason table: a no-Call cause
    iff `call_ref`, `run`, `source_ref` and `product_attempt_ref` are all null (`combination_unresolved`,
    `preparation`); `operand_source_unavailable` names the first term with no usable source; `operand_preparation_failure`
    names the refused record of the first `not_required` term with one, and no term lacks a source; a
    CombinationReason cause iff it is the entry's Call's pre-source refusal reason, with the rest null; with a Run,
    `prepared_product_failure` naming its attempt (§4's table is the attempt's)."""
    cases, records, calls = body["cases"], _records(body), body["calls"]
    case_at = {c["basis_ref"]["ref_id"]: i for i, c in enumerate(cases)}
    for entry in body["combinations"]:
        if entry["disposition"] != "retained_unavailable":
            continue
        cause, reason = entry["reason"]["cause"], (entry["reason"]["code"], entry["reason"]["phase"])
        nulls = all(entry.get(key) is None for key in ("call_ref", "run", "source_ref", "product_attempt_ref"))
        terms = [case_at.get(t["case_id"]) for t in entry["expression"]["terms"]] if entry["expression"]["kind"] == "mechanics" else []
        if cause.get("kind") in ("operand_source_unavailable", "operand_preparation_failure"):
            fail(nulls and reason == ("combination_unresolved", "preparation") and bool(terms) and None not in terms)
            unusable = next((i for i, ci in enumerate(terms) if _source_unusable(cases[ci])), None)
            if cause["kind"] == "operand_source_unavailable":
                fail(unusable is not None and cause["operand_index"] == unusable)
            else:
                refused = lambda ci: any(r["owner_ref"]["index"] == ci and r["result"]["kind"] == "refused" for r in records)
                first = next((ci for ci in terms if cases[ci]["status"] == "not_required" and refused(ci)), None)
                record = _at(records, cause["operand_preparation_ref"])
                fail(unusable is None and first is not None and record["result"]["kind"] == "refused" and record["owner_ref"]["index"] == first)
        elif cause.get("space") == "combination":
            call = _at(calls, entry["call_ref"]) if entry.get("call_ref") is not None else None
            fail(call is not None and all(entry.get(key) is None for key in ("run", "source_ref", "product_attempt_ref"))
                 and call["kind"] == "mechanics_combination" and call["result"]["kind"] == "pre_source_refusal"
                 and call["result"]["reason"] == cause and reason == ("combination_unresolved", "preparation"))
        elif cause.get("kind") == "prepared_product_failure":
            fail(not nulls and all(entry.get(key) is not None for key in ("call_ref", "run", "source_ref")) and entry.get("product_attempt_ref") == cause["product_attempt_ref"])
        else:
            fail(False)


def _g5_operand_preparations(body, fail, wf):
    """C3a-7's G5 rows: its ordinary attempt and material basis are its owner's; `stage == completed` iff `prepared` iff
    a `source_ref`; a refused record's error is the preparation branch, with no CaptureError `origin` (N-5); C3's
    member-prefix rules (complete members, or a successful prefix then at most one refused member), PRODUCT_ATTEMPT;
    C3's preparation-work status, conversion-prefix and count equations, WORK."""
    for record in _records(body):
        owner = _at(body["cases"], record["owner_ref"]["index"])
        fail(record["ordinary_attempt_ref"] == owner["ordinary"]["attempt_ref"])
        ordinary = _at(body["ordinary_attempts"], record["ordinary_attempt_ref"])
        fail(record["material_basis_ref"] == ordinary["material_basis_ref"])
        prepared = record["result"]["kind"] == "prepared"
        fail((record["stage"] == "completed") == prepared and prepared == (record["source_ref"] is not None))
        if not prepared:
            fail(record["result"]["error"]["kind"] == "preparation" and record["result"]["error"]["capture"].get("kind") != "origin")
        pm, new = record["preparation"]["members"], record["operational"]["new"]
        _g5_preparation_members(record, record["operational"]["old_coverage"] != "captured_prefix" or (not pm and not new and not prepared), fail, wf)
        if prepared:
            fail(len(pm) == len(record["operational"]["old"]) == len(new) and all(m["result"]["kind"] == "prepared" for m in pm)
                 and record["operational"]["old_coverage"] == "complete" and all(m["result"]["kind"] == "ready" for m in new))
            fail(_at(body["sources"], record["source_ref"])["material_basis_ref"] == record["material_basis_ref"])
            wf(all(_exact_work(m["work"]) for m in pm) and record["adapter"]["fault"] is None)
        result = {"kind": "ready"} if prepared else {"kind": "unavailable", "error": record["result"]["error"]}
        for ok in _accounting_rules({"proof": None, "result": result, "preparation": record["preparation"], "operational": record["operational"], "adapter": record["adapter"]}): wf(ok)


def _numeric_source(body, kind, ref):
    """G5a-G5c's source for a numeric owner (REVISION_01 §3.3 N-9): a case's own CaseSource; for a combination, its
    representative (operand 0's CaseSource), with the operands whose factor is nonzero as the data-fact sources (DEF-C
    `lanes.loads`: data flags come from each individual product c_i*v_ij, never from a net)."""
    source = body["sources"][int(ref)]
    if kind == "case":
        return source, None
    operands = source["operands"]
    return body["sources"][int(operands[0]["source_ref"])], [body["sources"][int(o["source_ref"])] for o in operands if from_bits(o["factor"]) != 0]


def _r_comb_1(body, rows_by_case):
    """CONTRACT §5 (R) with REVISION_01 §2 and REVISION_02 §4.1: every row of a `retained_unavailable` combination, and
    of an `ordinary` one whose expression names a case that is not `not_required`, is withheld from binding with its
    value unchanged: `not_covered` for a quantity row, `non_quantity` for a record row; `normalized_bits` from the
    row's value, `scale_bits` null. Combinations in authored order, rows in publication order. A derivation, not a check."""
    status = {c["basis_ref"]["ref_id"]: c["status"] for c in body["cases"]}
    classes = []
    for entry in body.get("combinations") or []:
        if entry["disposition"] == "retained_unavailable" or (entry["disposition"] == "ordinary" and any(status.get(x) != "not_required" for x in _referenced_ids(entry["expression"]))):
            for row in rows_by_case[entry["basis_ref"]["ref_id"]]:
                classes.append({"result_id": row["id"], "basis_ref": row["basis_ref"], "normalized_bits": bits(_normalized(row)), "scale_bits": None,
                                "class": "non_quantity" if _row_kind(row) == "non_quantity" else "not_covered", "bound_bits": None})
    return classes


def _model_combinations_match(body, model):
    """CONTRACT §10.1 G8 invocation: the entries are the invocation's `model.combinations` (absent is []), in order and
    by id, and each expression is its model combination's: basis to kind; mechanics terms' `load_case` and factor bits
    (the JSON number as binary64), in authored order with repeats; subtraction's minuend then subtrahend; range ids
    sorted in UTF-8 byte order, and the mode."""
    raw = model.get("combinations", [])
    if type(raw) is not list or len(raw) != len(body["combinations"]):
        return False
    for entry, combination in zip(body["combinations"], raw):
        if type(combination) is not dict or entry["basis_ref"]["ref_id"] != combination.get("id"):
            return False
        expression, basis = entry["expression"], combination.get("basis")
        if expression["kind"] == "mechanics":
            terms = combination.get("terms")
            if basis != "mechanics" or type(terms) is not list or len(terms) != len(expression["terms"]):
                return False
            for term, raw_term in zip(expression["terms"], terms):
                factor = raw_term.get("factor") if type(raw_term) is dict else None
                if type(factor) not in (int, float) or not math.isfinite(factor) or term["case_id"] != raw_term.get("load_case") or term["factor"] != bits(float(factor)):
                    return False
        elif expression["kind"] == "result_state_subtraction":
            if basis != "result_state_subtraction" or (expression["minuend_id"], expression["subtrahend_id"]) != (combination.get("minuend_id"), combination.get("subtrahend_id")):
                return False
        else:
            ids = combination.get("operand_ids")
            if (basis != "range_envelope" or type(ids) is not list or not all(type(x) is str for x in ids)
                    or expression["operand_ids"] != sorted(ids, key=lambda x: x.encode("utf-8")) or expression["mode"] != combination.get("mode")):
                return False
    return True


def _g8_combinations(body, need):
    """CONTRACT §10.1 G8 preparation (PREPARATION_MISMATCH), after the cases' and operand preparations' bindings: K4CMB
    recomputed from the operands' recomputed K4SRC bytes and factor bits (FK `CasePrep::combination`) equals
    `kernel_source_sha256`; R-8's operand equality (§2.5): every operand CaseSource's material basis and section terms
    are the representative's, and a selected combination's Selection section terms are the representative's,
    projected; the combination attempt's material basis is every operand case's. K4LED stays attested (C-12)."""
    sources = body["sources"]
    for entry in body["combinations"]:
        if entry.get("source_ref") is None:
            continue
        combined = sources[int(entry["source_ref"])]
        operands = combined["operands"]
        encoding = bytearray(b"K4CMB\x01") + struct.pack("<I", len(operands))
        for operand in operands:
            k4src = _native_source_encoding(sources[int(operand["source_ref"])], True)
            encoding += struct.pack("<Q", int(operand["factor"], 16)) + struct.pack("<I", len(k4src)) + k4src
        need(hashlib.sha256(bytes(encoding)).hexdigest() == combined["kernel_source_sha256"])
        representative = sources[int(combined["representative_source_ref"])]
        for operand in operands:
            case_source = sources[int(operand["source_ref"])]
            need(case_source["material_basis_ref"] == representative["material_basis_ref"] and case_source["section_terms"] == representative["section_terms"])
        if entry["disposition"] == "retained_selected":
            members = {m["kernel_member"]: m["id"] for m in representative["id_maps"]["members"]}
            keys = ("area", "section_modulus", "length", "axial_stiffness", "torsional_stiffness")
            need(entry["selection"]["section_terms"] == [dict({"member_id": members.get(t["member"])}, **{k: t[k] for k in keys}) for t in representative["section_terms"]])
        if entry.get("product_attempt_ref") is not None:
            basis = body["product_attempts"][int(entry["product_attempt_ref"])]["material_basis_ref"]
            need(basis == representative["material_basis_ref"] and all(basis == body["ordinary_attempts"][int(o["case_index"])]["material_basis_ref"] for o in operands))

def validate_retained_precision(source: Any, invocation: Any = None) -> dict[str, Any]:
    """The accepted ordered reader (G0-G8). D-U6-1: every gate runs; since U7 a valid invocation-bound
    statement of a solved model whose cases are selected or not_required reads eligible. Hashes bind
    the supplied statements; they do not establish producer origin (D-U7-6)."""
    return _validate_draft(source, invocation)


def validate_retained_precision_transport(source: Any) -> dict[str, Any]:
    """F-U6b-2 (B6): G0-G2, then the base step on the transport projection (no rows read).
    On G0-G2 this is the twin of Rust `validate_transport_metadata` and TS
    `validateRetainedPrecisionTransport`: the same checks, gates and codes. At the base step it runs
    both of theirs, and every reader runs both (RV108 N6(a); the alignment set, item 4): the base header
    check, in Rust's order and with Rust's codes, reported at G2 (RV113 N-1; RR "I4 made at
    `30f3d1b24a`; …", ruling 1: no carrier branch, and `source_block_recovery` before `contract_evidence`);
    then the preview-physics transport metadata check, reported at G7
    (SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID), which refuses a `carrier_evidence` member and compares
    the cases' withheld records as multisets (ruling 2). Omitted raw publication bytes are never
    reconstructed or verified, so a transported statement is never eligible."""
    return _validate_draft(source, None, raw=False)


def _transport_base(snapshot: dict[str, Any], route: Route = PREVIEW_ROUTE) -> None:
    """The base step on the reader's transport projection (no rows are read): `_source_contract` runs the base
    header check in Rust's order (`rust_header_order`, ruling 1), then the preview-physics transport metadata
    check. A failure keeps the base validator's leading code, with its full text as detail, as at the raw G7.
    Its gate (the alignment set, item 4): the metadata check raises only SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID,
    and the header check never does (it runs first, and every header code differs), so that code is G7's and
    every other is the header's, G2. Raw reads keep Python's own header order at G7 (the declared raw codes).
    B3b (B3-D §6.2, transport): on the exact branch the projection is physics-1's and its metadata check is physics-1's
    transport check, whose code (SOURCE_PHYSICS_EVIDENCE_INVALID) is G7's in the same way."""
    projected=_project(snapshot,route)
    from .compatibility import _source_contract
    try:_source_contract(projected,check_receipt=False,rust_header_order=True)
    except ValueError as exc:
        text=str(exc);match=re.match(r"[A-Z][A-Z0-9_]*",text)
        code=match.group(0) if match else route.base_invalid
        error=RetainedPrecisionError("G7" if code in route.metadata_codes else "G2",code);error.detail=text
        raise error from exc


def _g0_exact(receipt):
    """G0 on the exact branch, B3-D §2.4 steps 3 to 9 (step 1 is the dispatch, step 2 the envelope): the packaged
    physics-retained-1 table and DEF-E, the table/constant cross-check, then the receipt's bound values and
    definition ids read from the table (decision 31, N-12). Every failure is SOURCE_PRODUCER_CONTRACT_UNSUPPORTED
    except DEF-E's binding (step 4), RETAINED_PRECISION_FORMATION_MISMATCH."""
    need = lambda ok, code="SOURCE_PRODUCER_CONTRACT_UNSUPPORTED": _need(ok, "G0", code)
    results = ROOT / "fixtures/results"
    table_bytes = (results / "semantic_contract_v0_3_physics_retained_1.json").read_bytes()
    table = json.loads(table_bytes)
    # 3. The table's identity and profile.
    need(type(table) is dict and table.get("semantic_contract_id") == EXACT_CONTRACT_ID and table.get("formulation_profile_id") == EXACT_PROFILE)
    # 4. H(packaged DEF-E) is the reader's constant, and the table binds exactly that definition.
    definition = json.loads((results / "retained_precision_prepared_exact_v1.json").read_text())
    need(_hash("retained_precision_formation_v1", definition) == EXACT_DEFINITION_HASH
         and _same(table.get("product_formation_definitions"), [{"id": EXACT_DEFINITION_ID, "sha256": EXACT_DEFINITION_HASH}]), "FORMATION_MISMATCH")
    # 5. The table's bytes, and its inherited hash is physics-1's packaged table.
    inherited = (results / "semantic_contract_v0_3_physics_1.json").read_bytes()
    need(hashlib.sha256(table_bytes).hexdigest() == EXACT_TABLE_HASH and hashlib.sha256(inherited).hexdigest() == table.get("inherited_semantic_contract_sha256"))
    # 6. The cross-check: each bound table value equals the reader's constant.
    policy = table.get("accuracy_classification")
    if not (_same(table.get("receipt_bindings"), RECEIPT_BINDINGS) and table.get("receipt_policy") == RECEIPT_POLICY
            and type(policy) is dict and policy.get("policy") == FACADE_POLICY):
        raise RetainedPrecisionError("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "table/constant cross-check")
    bound = table["receipt_bindings"]
    # 7. A receipt with a body, receipt_version 1.
    need(type(receipt) is dict and type(receipt.get("body")) is dict)
    b = receipt["body"]
    need(_integral(b.get("receipt_version")) == 1)
    # 8. The receipt's values are the table's.
    for key, value in {"policy": table["receipt_policy"], "facade_policy": policy["policy"], "projection_policy": bound["projection_policy"],
                       "work_policy": bound["work_policy"], "canonicalization": bound["canonicalization"]}.items():
        need(type(b.get(key)) is str and b[key] == value)
    w = b.get("work")
    need(type(w) is dict and _integral(w.get("case_limit")) == bound["work"]["case_limit"] and _integral(w.get("invocation_limit")) == bound["work"]["invocation_limit"])
    # 9. Every attempt's definition is the table's (and every operand preparation's: B2-C §8 row 9; the exact table
    # binds DEF-E only, so an operand preparation, which carries DEF-O's id, is refused on this route).
    for attempt in _definition_bearers(b):
        need(attempt.get("definition_id") == table["product_formation_definitions"][0]["id"])


def _definition_bearers(body):
    """The receipt's `product_attempts[]` and `operand_preparations[]` objects (G0 row 9), tolerating any shape."""
    for key in ("product_attempts", "operand_preparations"):
        for item in body.get(key, []) if type(body.get(key)) is list else []:
            if type(item) is dict: yield item


def _preview_statics():
    """The preview route's packaged statics (B2-C §8): PTABLE's bytes, its pinned hash, the inherited table's bytes, and
    each packaged formation definition (DEF-O, DEF-C) by id. G0's table-dependent checks take these as a parameter."""
    results = ROOT / "fixtures/results"
    return {"table": (results / "semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes(), "table_hash": TABLE_HASH,
            "inherited": (results / "semantic_contract_v0_3_preview_physics_1.json").read_bytes(),
            "definitions": {DEFINITION_ID: json.loads((results / "retained_precision_prepared_ordinary_v1.json").read_text()),
                            COMBINATION_DEFINITION_ID: json.loads((results / "retained_precision_prepared_combination_v1.json").read_text())}}


def _g0_preview(receipt, statics=None):
    """G0 on the preview route after the dispatch (B2-C §8, N-12, decision 31), rows 4 to 9: the formation definitions
    (FORMATION_MISMATCH), the table's hashes, the table/constant cross-check, the body, the receipt's bound values read
    from the table, and every attempt's and operand preparation's definition id among the table's. Every other failure
    is SOURCE_PRODUCER_CONTRACT_UNSUPPORTED. `statics` defaults to the packaged copy (tests pass a test-only copy)."""
    need = lambda ok, code="SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", detail=None: ok or _raise("G0", code, detail)
    statics = _preview_statics() if statics is None else statics
    table = json.loads(statics["table"])
    # 4. The table binds exactly [DEF-O, DEF-C] in that order, and each packaged definition's H is its constant.
    need(type(table) is dict and _same(table.get("product_formation_definitions"), PREVIEW_DEFINITIONS)
         and all(_hash("retained_precision_formation_v1", statics["definitions"].get(d["id"])) == d["sha256"] for d in PREVIEW_DEFINITIONS),
         "RETAINED_PRECISION_FORMATION_MISMATCH", "formation definitions")
    # 5. The table's bytes and its inherited hash (today's check).
    need(hashlib.sha256(statics["table"]).hexdigest() == statics["table_hash"]
         and hashlib.sha256(statics["inherited"]).hexdigest() == table.get("inherited_semantic_contract_sha256"), detail="table hash")
    # 6. The cross-check: each bound table value equals the reader's constant.
    policy = table.get("accuracy_classification")
    need(_same(table.get("receipt_bindings"), RECEIPT_BINDINGS) and table.get("receipt_policy") == RECEIPT_POLICY
         and type(policy) is dict and policy.get("policy") == FACADE_POLICY, detail="table/constant cross-check")
    bound = table["receipt_bindings"]
    # 7. D2 + settled readings 1-2: an absent retained_precision or body is an absent G0 field.
    need(type(receipt) is dict and type(receipt.get("body")) is dict)
    b = receipt["body"]
    # 8. The receipt's bound members are the table's (receipt_version exactly 1, C1 s4).
    for key, value in {"receipt_version": 1, "policy": table["receipt_policy"], "projection_policy": bound["projection_policy"],
                       "work_policy": bound["work_policy"], "facade_policy": policy["policy"], "canonicalization": bound["canonicalization"]}.items():
        need((_integral(b.get(key)) == value) if type(value) is int else (type(b.get(key)) is str and b.get(key) == value))
    w = b.get("work")
    need(type(w) is dict and _integral(w.get("case_limit")) == bound["work"]["case_limit"] and _integral(w.get("invocation_limit")) == bound["work"]["invocation_limit"])
    # 9. Every attempt's and operand preparation's definition id is one of the table's (which owner carries which is G1's).
    ids = [d["id"] for d in table["product_formation_definitions"]]
    for attempt in _definition_bearers(b):
        need(attempt.get("definition_id") in ids)


def _raise(gate, code, detail=None):
    raise RetainedPrecisionError(gate, code, detail)


def _project(snapshot: dict[str, Any], route: Route) -> dict[str, Any]:
    """G7's projection to the route's base (D2 §4.9.3; B3-D §6.2): no receipt, the base identity and profile, and
    no row token. The exact branch keeps physics-1's contract_evidence for physics-1's unchanged base validator."""
    projected=deepcopy(snapshot);del projected["retained_precision"]
    projected["producer"]["semantic_contract_id"]=route.base_id;projected["formulation_basis"]["profile_id"]=route.base_profile
    for row in projected["results"] if type(projected.get("results")) is list else []:
        if type(row) is dict:row.pop("recovery_method",None)
    return projected


def _validate_draft(source: Any, invocation: Any = None, *, raw: bool = True) -> dict[str, Any]:
    """The ordered checks behind the public entry (D-U6-1). With raw=False, the transport checks
    (F-U6b-2): G1 skips the raw rows and the publication digest, as Rust's g1(source, false).
    B3b: one dispatch on the identity at G0 selects the route (B3-D §6.1); the other gates are shared."""
    gate="G0";route=None
    try:
        producer=source.get("producer") if type(source) is dict else None;basis=source.get("formulation_basis") if type(source) is dict else None
        contract=producer.get("semantic_contract_id") if type(producer) is dict else None
        route=ROUTES.get(contract) if type(contract) is str else None
        _need(type(producer) is dict and type(basis) is dict and route is not None and basis.get("profile_id")==route.profile,"G0","SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        snapshot=deepcopy(source);invocation=deepcopy(invocation);receipt=snapshot.get("retained_precision");schema=_schema()
        _need(snapshot.get("schema_version")=="0.2.0" and snapshot["producer"].get("component_name")=="open_pipe_stress_product_physics" and snapshot["producer"].get("component_version")=="0.2.0",gate,"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
        if route.exact:_g0_exact(receipt)
        else:_g0_preview(receipt)
        gate="G1";_need(_shape(receipt,schema) and (not raw or (type(snapshot.get("results")) is list and all(_shape(r,schema["$defs"]["RawRow"]) for r in snapshot["results"]))),gate,"RECEIPT_MISMATCH")
        body=receipt["body"]
        _need(_hash("retained_precision_receipt_mp_v2",body)==receipt["receipt_sha256"],gate,"RECEIPT_MISMATCH")
        if raw:_need(_hash("retained_precision_publication_mp_v2",{k:v for k,v in snapshot.items() if k!="retained_precision"})==body["publication_sha256"],gate,"RECEIPT_MISMATCH")
        # Integrity is G1. Invalid reference representation/coverage is left for
        # G2/G3; only already-addressable records have an integrity comparison.
        for c in body["cases"]:
            si=_integral(c.get("source_ref"))
            if c["status"]=="selected" and si is not None and 0<=si<len(body["sources"]):
                _need(c["source_identity_sha256"]==_source_hash(body["sources"][si]),gate,"RECEIPT_MISMATCH")
        for s in body["sources"]:
            prep=s.get("preparation")
            if prep is not None and "attempt_ref" in prep:
                ai=_integral(prep["attempt_ref"])
                if ai is not None and 0<=ai<len(body["product_attempts"]):
                    a=body["product_attempts"][ai]
                    if all(m["result"]["kind"]=="prepared" for m in a["preparation"]["members"]):
                        # S-1 (B3-D REVISION_01 §2): the payload carries the route's definition hash, not DEF-O's.
                        _need(prep["sha256"]==_hash("retained_precision_preparation_v1",_preparation_payload(a,route.definition_hash)),gate,"RECEIPT_MISMATCH")
        _g1_combinations(body,route)
        gate="G2";_encoding(receipt,schema);_need(not _negative_zero(receipt),gate,"ENCODING_MISMATCH");_normalize_integrals(receipt)  # D34, then D32
        if not raw:
            gate="G7";_transport_base(snapshot,route)  # a header failure is raised at G2 (item 4); an escape still falls back at G7
            return {"invocation_bound":False,"numerical_eligible":False,"standing":"needs_recompute","publication_sha256":body["publication_sha256"],"classifications":[]}
        gate="G3";cases=body["cases"];quality=snapshot["numerical_quality"]["cases"]
        ids=[c["basis_ref"]["ref_id"] for c in cases]
        _need(len(set(ids))==len(ids) and [c["basis_ref"] for c in cases]==[q["basis_ref"] for q in quality] and any(c["status"]=="selected" for c in cases),gate,"COVERAGE_MISMATCH")
        if invocation is not None:_need(ids==[c["id"] for c in invocation["request"]["model"]["load_cases"]],gate,"COVERAGE_MISMATCH")
        # B2-C §10.1 G3 (c), REVISION_01 §4.1 #5: a row's basis names a case or a combination entry (at z = 0, a case).
        combos=body["combinations"];cids=[c["basis_ref"]["ref_id"] for c in combos]
        rows={cid:[] for cid in ids};crows={cid:[] for cid in cids};seen=set()
        for row in snapshot["results"]:
            basis=row.get("basis_ref",{});owned=rows if basis.get("ref_type")=="load_case" else crows if basis.get("ref_type")=="combination" else {}
            _need(row["id"] not in seen and basis.get("ref_id") in owned,gate,"COVERAGE_MISMATCH")
            seen.add(row["id"]);owned[basis["ref_id"]].append(row)
        _need(len(body["ordinary_attempts"])==len(cases),gate,"COVERAGE_MISMATCH")
        for s in body["sources"]:_need(s["owner"]["kind"]=="combination" or len(s["body_membership"])>0,gate,"COVERAGE_MISMATCH")  # D29 (a CombinationSource has none)
        # The receipt's own references, bound and unbound (RR "RV113's three returns verified; ...; the three-reader
        # alignment set ruled", item 1; TS `coverage`): each source and each material basis sits at its index, a
        # source's owner is its own case, and a basis's case list is unique and in range.
        for si,s in enumerate(body["sources"]):
            if s["owner"]["kind"]=="combination":
                # REVISION_01 §4.1 #4: a CombinationSource names its own combination entry.
                ki=_integral(s["owner"]["combination_index"])
                _need(s["index"]==si and ki is not None and 0<=ki<len(combos) and cids[ki]==s["owner"]["combination_id"],gate,"COVERAGE_MISMATCH")
                continue
            ci=_integral(s["owner"]["case_index"])
            _need(s["index"]==si and s["owner"]["kind"]=="case" and ci is not None and 0<=ci<len(cases) and ids[ci]==s["owner"]["case_id"],gate,"COVERAGE_MISMATCH")
        for mi,mb in enumerate(body["material_bases"]):
            _need(mb["index"]==mi and len(set(mb["case_indices"]))==len(mb["case_indices"]) and all(x<len(cases) for x in mb["case_indices"]),gate,"COVERAGE_MISMATCH")
        # REVISION_01 §4.1 #1: the attempt bijection covers the cases' and the combination entries' references.
        refs=[c["product_attempt_ref"] for c in cases if c["product_attempt_ref"] is not None]+[c["product_attempt_ref"] for c in combos if c.get("product_attempt_ref") is not None]
        _need(sorted(refs)==list(range(len(body["product_attempts"]))),gate,"COVERAGE_MISMATCH")
        for ai,a in enumerate(body["product_attempts"]):
            if a["owner_ref"]["kind"]=="combination":
                _g3_combination_attempt(body,a,ai,crows);continue  # REVISION_01 §4.1 #3
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
        # B2-C G3 (f), REVISION_01 §4.1 #2: the case Runs, then the combination Runs in Call order (at z = 0, the cases' only).
        order=body["work"]["execution_order"];with_run=[i for i,c in enumerate(cases) if c.get("run") is not None]
        n=next((k for k,e in enumerate(order) if e["kind"]!="case"),len(order));head,tail=order[:n],order[n:]
        _need(all(e["kind"]=="case" for e in head) and sorted(e["index"] for e in head)==with_run and all(cases[e["index"]]["run"]["id"]==k for k,e in enumerate(head)),gate,"COVERAGE_MISMATCH")
        _need(all(e["kind"]=="combination" for e in tail) and [e["index"] for e in tail]==[k for k,c in enumerate(combos) if c.get("run") is not None]
              and all(combos[e["index"]]["run"]["id"]==n+k for k,e in enumerate(tail)),gate,"COVERAGE_MISMATCH")
        for i,c in enumerate(cases):
            _need(c["ordinary"]["attempt_ref"]==i and c["ordinary"]["quality_binding"]=={"kind":"present","index":i} and body["ordinary_attempts"][i]["case_index"]==i and body["ordinary_attempts"][i]["case_id"]==ids[i],gate,"COVERAGE_MISMATCH")
        _g3_combinations(snapshot,body,ids,crows)
        rows.update(crows)  # one id set (G3 (b)): the cases' and the combination entries' rows
        gate="G4";diags=snapshot["diagnostics"]
        _need(len({d["id"] for d in diags})==len(diags) and not any(d["code"]=="SOURCE_BLOCK_RECOVERY_SELECTED" for d in diags),gate,"DIAGNOSTIC_MISMATCH")
        for i,c in enumerate(cases):
            cid=ids[i];selected=[d for d in diags if d["code"]=="RETAINED_PRECISION_SELECTED" and cid in d.get("affected_refs",[])];unavailable=[d for d in diags if d["code"]=="RETAINED_PRECISION_UNAVAILABLE" and cid in d.get("affected_refs",[])]
            _need(len(selected)==int(c["status"]=="selected") and len(unavailable)==int(c["status"]=="unavailable") and all(d["affected_refs"]==[cid] for d in selected+unavailable),gate,"DIAGNOSTIC_MISMATCH")
            if c["status"]=="unavailable":_need(unavailable[0]["id"]==c["diagnostic_ref"],gate,"DIAGNOSTIC_MISMATCH")
            if c["status"]=="selected":_need(not any(d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE" and cid in d.get("affected_refs",[]) for d in diags),gate,"DIAGNOSTIC_MISMATCH")
        for d in diags:
            # D7 (C1 G4 row; C1:147): every retained diagnostic names exactly one requested case.
            # REVISION_01 §4.1 #6: one case or one combination.
            if d["code"] in ("RETAINED_PRECISION_SELECTED","RETAINED_PRECISION_UNAVAILABLE"):_need(type(d.get("affected_refs")) is list and len(d["affected_refs"])==1 and (d["affected_refs"][0] in ids or d["affected_refs"][0] in cids),gate,"DIAGNOSTIC_MISMATCH")
        _g4_combinations(combos,diags)
        gate="G5";_g5_native(body);_g5_ordinary(body,cases,diags,quality,(snapshot.get("contract_evidence") or {}).get("combination_gates"));_g5_products(body,rows)
        # B3-D §6.2: on the exact branch G5b also binds each selected case's own physics-1 section evidence.
        gate="G5a";phase=["G5a"];classes=_g5_numeric(body,rows,phase,(snapshot.get("contract_evidence"),) if route.exact else None)
        gate="G6"
        for c in cases:
            for row in rows[c["basis_ref"]["ref_id"]]:_need(row.get("recovery_method")==METHOD if c["status"]=="selected" else "recovery_method" not in row,gate,"ROW_METHOD_MISMATCH")
        for c in body["combinations"]:
            # CONTRACT §10.1 G6: recovery_method exactly on a retained_selected combination's rows.
            for row in rows[c["basis_ref"]["ref_id"]]:_need(row.get("recovery_method")==METHOD if c["disposition"]=="retained_selected" else "recovery_method" not in row,gate,"ROW_METHOD_MISMATCH")
        gate="G7";projected=_project(snapshot,route)
        from .compatibility import _source_contract
        try:_source_contract(projected)
        except ValueError as exc:
            text=str(exc);match=re.match(r"[A-Z][A-Z0-9_]*",text)
            error=RetainedPrecisionError("G7",match.group(0) if match else route.base_invalid);error.detail=text
            raise error from exc
        gate="G8"
        if invocation is not None:_g8(body,snapshot,invocation,route)
        eligible=_IMPLEMENTATION_COMPLETE and invocation is not None and snapshot["status"]["mechanics"]=="MECHANICS_SOLVED" and all(c["status"] in ("selected","not_required") for c in cases)
        return {"invocation_bound":invocation is not None,"numerical_eligible":eligible,"standing":"eligible" if eligible else "needs_recompute","publication_sha256":body["publication_sha256"],"classifications":classes}
    except RetainedPrecisionError:raise
    except (KeyError,IndexError,TypeError,ValueError,OverflowError,ZeroDivisionError,StopIteration,AttributeError) as exc:
        # Fail-closed fallback only (D16): checks report their own codes; G5a/G5b/G5c follow the phase (D10).
        if gate=="G5a":gate=phase[0]
        code={"G0":"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED","G1":"RETAINED_PRECISION_RECEIPT_MISMATCH","G2":"RETAINED_PRECISION_ENCODING_MISMATCH","G3":"RETAINED_PRECISION_COVERAGE_MISMATCH","G4":"RETAINED_PRECISION_DIAGNOSTIC_MISMATCH","G5":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH","G5a":"RETAINED_PRECISION_SCALE_MISMATCH","G5b":"RETAINED_PRECISION_SCALE_MISMATCH","G5c":"RETAINED_PRECISION_CLASSIFICATION_MISMATCH","G6":"RETAINED_PRECISION_ROW_METHOD_MISMATCH","G8":"RETAINED_PRECISION_PREPARATION_MISMATCH"}.get(gate,(route or PREVIEW_ROUTE).base_invalid)
        raise RetainedPrecisionError(gate,code) from exc
