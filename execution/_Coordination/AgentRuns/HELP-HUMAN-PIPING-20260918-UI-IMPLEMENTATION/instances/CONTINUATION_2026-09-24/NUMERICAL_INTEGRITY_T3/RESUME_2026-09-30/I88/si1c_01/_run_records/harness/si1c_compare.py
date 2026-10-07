#!/usr/bin/env python3
"""I88 T3-SI1c differential analysis (scratch tool, not repository content).

Reads the dumps of base (main 025c1cf326), the instrumented base (`ibase`, the
same tree plus a side channel) and the candidate, and checks I87 PLAN §6.4's
six pass conditions:

1. every line with no flag and no N-4 value is byte-identical (all modes);
2. every flagged line (check) is blocked on the candidate, with NonFiniteInput
   at the first flagged producer as its last finding (no flagged line passes
   or fails);
3. the interval evaluator dumps are byte-identical on every line;
4. runner lines in bounded modes are byte-identical except N-4 lines and
   point-path checks flagged inside a bounded run, each checked as in 2 or 5.1;
5. plain equals b = 0 on every candidate line;
6. every candidate runner line validates against the committed schema; on the
   base, the failures are exactly the two `null` classes.

Also: the instrumented base's dump equals the plain base's byte for byte, and
the counts by family, site and consumer.

Usage: si1c_compare.py <scratch dir S> <schema path> <report json>
"""
from __future__ import annotations

import collections
import hashlib
import json
import re
import sys
from pathlib import Path

S = Path(sys.argv[1])
SCHEMA = Path(sys.argv[2])
REPORT = Path(sys.argv[3])
D = S / "dumps"

D_MESSAGES = {
    "add_subtract": "sum or difference must be finite (it overflowed)",
    "multiply": "product must be finite (it overflowed)",
    "divide": "quotient must be finite (it overflowed)",
}
INTERP_MESSAGE = "interpolated table value must be finite (a step of the interpolation overflowed)"
INPUT_MESSAGE = "supplied value must be finite (NaN or ±inf after unit normalization)"
LIMIT_MESSAGE = "value-slot limit must be finite (NaN or ±inf after unit normalization)"
NOTE = "non-finite value (NaN or ±inf, after unit normalization): not bound"


def d_message(subject: str) -> str:
    return D_MESSAGES.get(subject, INTERP_MESSAGE)


FINDING = re.compile(r'EvaluationFinding \{ code: (\w+), subject_id: "((?:[^"\\]|\\.)*)", message: "((?:[^"\\]|\\.)*)" \}')
SOURCES = re.compile(r'source_variable_ids: \[(.*?)\], findings')
STATUSES = re.compile(r'statuses: \[(.*?)\], source_variable_ids')

report: dict = {"violations": collections.Counter(), "examples": collections.defaultdict(list)}


def violation(kind: str, detail: str):
    report["violations"][kind] += 1
    if len(report["examples"][kind]) < 8:
        report["examples"][kind].append(detail[:600])


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


# -------------------------------------------------------------------- flags

def read_flags(path: Path) -> dict[str, list[list[str]]]:
    by_thread: dict[str, list[list[str]]] = collections.defaultdict(list)
    if not path.exists():
        return by_thread
    for line in path.read_text().splitlines():
        parts = line.split("\t")
        by_thread[parts[0]].append(parts[1:])
    return by_thread


def parse_events(record: list[str]) -> dict[int, dict]:
    """R record -> per check index: formula (first, consumer, fires), n4in list, n4lim list."""
    checks: dict[int, dict] = collections.defaultdict(lambda: {"formula": None, "compare": None, "n4in": [], "n4lim": []})
    if len(record) < 2 or not record[1]:
        return checks
    for event in record[1].split(";"):
        index, kind, detail = event.split("|", 2)
        c = checks[int(index)]
        if kind in ("formula", "compare"):
            first, consumer, fires = detail.split(",")
            c[kind] = (first, consumer, int(fires))
        else:
            c[kind].append(detail)
    return checks


# --------------------------------------------------------------- evaluator

def findings_of(debug: str):
    return FINDING.findall(debug)


def sources_of(debug: str):
    m = SOURCES.search(debug)
    return [x.strip('"') for x in m.group(1).split(", ")] if m and m.group(1) else []


def outcome_class(bits: str) -> str:
    if bits == "none":
        return "blocked"
    if bits in ("true", "false"):
        return f"decided_{bits}"
    if bits.startswith("0x"):
        value = int(bits, 16)
        exponent = (value >> 52) & 0x7FF
        return "nonfinite_quantity" if exponent == 0x7FF else "finite_quantity"
    return bits


def ee_family(dump: str, label: str) -> str:
    if dump.startswith("i79"):
        return dump
    return f"{dump}:{label.split('_', 1)[0]}"


def compare_ee(dump: str, base_lines, cand_lines, flags, point_format: str):
    stats = collections.Counter()
    consumers = collections.Counter()
    sites = collections.Counter()
    families = collections.defaultdict(collections.Counter)
    point_index = 0
    assert len(base_lines) == len(cand_lines), (dump, len(base_lines), len(cand_lines))
    for b, c in zip(base_lines, cand_lines):
        if point_format == "rv104":
            label, mode, rest_b = b.split("\t", 2)
            label_c, mode_c, rest_c = c.split("\t", 2)
            assert (label, mode) == (label_c, mode_c)
            if mode == "I":
                stats["interval_lines"] += 1
                if b != c:
                    violation("3_interval_differs", f"{dump} {label}")
                continue
            bits_b, _, debug_b = rest_b.partition("|")
            bits_c, _, debug_c = rest_c.partition("|")
        else:  # i79: label \t bits \t debug  |  label \t PANIC
            fb, fc = b.split("\t", 2), c.split("\t", 2)
            label = fb[0]
            assert label == fc[0]
            bits_b, debug_b = (fb[1], fb[2] if len(fb) > 2 else "")
            bits_c, debug_c = (fc[1], fc[2] if len(fc) > 2 else "")
        record = flags[point_index]
        point_index += 1
        assert record[0] == "E", record
        first, consumer, fires = record[1], record[2], int(record[3])
        family = ee_family(dump, label)
        families[family]["point_lines"] += 1
        stats["point_lines"] += 1
        if bits_b == "PANIC" or bits_c == "PANIC":
            violation("panic", f"{dump} {label} {bits_b} {bits_c}")
        if first == "-":
            if b != c:
                violation("1_unflagged_differs", f"{dump} {label}\n{b}\n{c}")
            else:
                stats["unflagged_identical"] += 1
                families[family]["unflagged_identical"] += 1
            continue
        # flagged
        stats["flagged"] += 1
        families[family]["flagged"] += 1
        base_class = outcome_class(bits_b)
        consumers[(consumer, base_class)] += 1
        sites[first if first in D_MESSAGES else "interpolation"] += 1
        families[family][f"flagged_base_{base_class}"] += 1
        if b == c:
            stats["flagged_identical"] += 1  # already blocked identically? (not expected)
        cand_findings = findings_of(debug_c)
        base_findings = findings_of(debug_b)
        expected_last = ("NonFiniteInput", first, d_message(first))
        if bits_c != "none" or not cand_findings or cand_findings[-1] != expected_last:
            violation("2_flagged_not_blocked_at_producer", f"{dump} {label} first={first}\n{b}\n{c}")
            continue
        if cand_findings[:-1] != base_findings[: len(cand_findings) - 1]:
            violation("2_flagged_prefix", f"{dump} {label}\n{b}\n{c}")
        if not set(sources_of(debug_c)) <= set(sources_of(debug_b)):
            violation("2_flagged_sources", f"{dump} {label}")
        if STATUSES.search(debug_c).group(1) != STATUSES.search(debug_b).group(1):
            violation("2_flagged_statuses", f"{dump} {label}")
        stats["flagged_verified"] += 1
    assert point_index == len(flags), (dump, point_index, len(flags))
    return {"stats": dict(stats), "consumers": {f"{k[0]}|{k[1]}": v for k, v in sorted(consumers.items())},
            "sites": dict(sites), "families": {k: dict(v) for k, v in sorted(families.items())}}


def compare_lines_identical(name: str, base: Path, cand: Path):
    b, c = base.read_text().splitlines(), cand.read_text().splitlines()
    differ = sum(1 for x, y in zip(b, c) if x != y) + abs(len(b) - len(c))
    if differ:
        violation("3_interval_differs", f"{name}: {differ} lines")
    return {"lines": len(b), "differ": differ}


# ------------------------------------------------------------------ runner

def split_run_line(dump: str, line: str):
    """(key, mode, json text or None, hash or None)."""
    parts = line.split("\t")
    if dump == "rv104_run":
        label, mode, rest = parts[0], parts[1], parts[2:]
        if rest[0].startswith("PANIC"):
            return label, mode, None, rest[0]
        if len(rest) == 2:
            return label, mode, rest[1], rest[0]
        return label, mode, None, rest[0]
    if dump == "i79_run_plain":
        return "\t".join(parts[:3]), parts[3], parts[4], None
    if dump == "i79_run_extreme":
        return "\t".join(parts[:4]), parts[4], "\t".join(parts[5:]), None
    # si1c_run
    return parts[0], parts[1], parts[2], None


def runner_family(dump: str, key: str) -> str:
    if dump == "rv104_run":
        if key.startswith("m_tab"):
            return "rv104_run:m_tab"
        return f"rv104_run:{key.split('_', 1)[0]}"
    if dump == "si1c_run":
        return "si1c_run"
    return f"{dump}:{key.split(chr(9))[0]}"


VALIDATOR = None


def schema_errors(doc) -> list:
    return list(VALIDATOR.iter_errors(doc))


def classify_base_schema(doc) -> collections.Counter:
    out = collections.Counter()
    for error in schema_errors(doc):
        path = list(error.absolute_path)
        if error.instance is None and path[-1:] == ["value"] and "bound_inputs" in path:
            out["bound_inputs_value_null"] += 1
        elif error.instance is None and path[-2:] == ["computed_value", "value"]:
            out["computed_value_null"] += 1
        else:
            out[f"other:{path}:{error.message[:80]}"] += 1
    return out


def finding_tuple(f: dict):
    return (f["code"], f["subject_id"], f["message"])


def strip_undeclared(findings: list) -> list:
    return [f for f in findings if f["code"] != "STATUS_NOT_DECLARED"]


def verify_check(cb: dict, cc: dict, e: dict) -> tuple[str, str | None]:
    """The class of a check with a flag or an N-4 value, and the first problem (or None).

    The expected candidate check is built from the base check and the oracle's
    events: N-4 inputs lose their value and gain the note (a raw value in
    another unit also has its unit-mismatch finding replaced by the N-4
    finding); then the check blocks where the candidate blocks first:
    completeness (as on the base), an N-4 formula input, the first flagged
    producer, or an N-4 limit."""
    import copy
    exp = copy.deepcopy(cb)
    inputs = {b["input_id"]: b for b in exp["bound_inputs"]}
    converted = []
    plain_n4 = []
    for event in e["n4in"]:
        input_id, stage, route = event.split(":")
        record = inputs[input_id]
        record.pop("value", None)
        record["note"] = NOTE
        if stage == "raw" and route == "converted":
            converted.append(input_id)
            for i, f in enumerate(exp["evaluator_findings"]):
                if f["code"] == "UnitMismatch" and f["subject_id"] == input_id and f["message"].startswith("unit '"):
                    exp["evaluator_findings"][i] = {"code": "NonFiniteInput", "severity": "blocking",
                                                    "subject_id": input_id, "message": INPUT_MESSAGE}
                    break
            else:
                return "n4_completeness", "converted_without_unit_mismatch"
        else:
            plain_n4.append(input_id)
    base_completeness_blocked = any(f["severity"] == "blocking" for f in cb["completeness_findings"])
    if base_completeness_blocked:
        return ("n4_completeness" if e["n4in"] else "completeness"), (None if cc == exp else "differs_from_expected")
    base_missing = [f["subject_id"] for f in cb["evaluator_findings"]
                    if f["code"] == "MissingRequiredValue" and f["message"] == "required variable has no supplied value"]
    formula_n4 = [i for i in dict.fromkeys(base_missing) if i in plain_n4]
    if formula_n4:
        resolution = [f for f in exp["evaluator_findings"] if f["message"].startswith("unit '") or
                      (f["code"] == "NonFiniteInput" and f["subject_id"] in converted)]
        expected = resolution + [{"code": "NonFiniteInput", "severity": "blocking", "subject_id": i, "message": INPUT_MESSAGE}
                                 for i in formula_n4]
        if strip_undeclared(cc["evaluator_findings"]) != expected:
            return "n4_formula", "findings"
        if cc["status"] != cb["status"]:
            return "n4_formula", "status_changed"
        if cc["diagnostic_codes"] != cb["diagnostic_codes"]:
            return "n4_formula", "diagnostic_changed"
        if cc["acceptability_relation"] != "none" or "computed_value" in cc or "limit_value" in cc:
            return "n4_formula", "outcome_fields"
        if cc["bound_inputs"] != exp["bound_inputs"] or cc["completeness_findings"] != exp["completeness_findings"]:
            return "n4_formula", "records"
        return "n4_formula", None
    if e["formula"] and e["formula"][0] != "-":
        first = e["formula"][0]
        findings = strip_undeclared(cc["evaluator_findings"])
        base_findings = [finding_tuple(f) for f in cb["evaluator_findings"]]
        if cc["status"] != "RULE_INPUTS_INCOMPLETE":
            return "flagged", "status"
        if not findings or finding_tuple(findings[-1]) != ("NonFiniteInput", first, d_message(first)):
            return "flagged", "last_finding"
        if [finding_tuple(f) for f in findings[:-1]] != base_findings[: len(findings) - 1]:
            return "flagged", "prefix"
        if "computed_value" in cc or "limit_value" in cc or cc["acceptability_relation"] != "none":
            return "flagged", "outcome_fields"
        if cc["bound_inputs"] != exp["bound_inputs"] or cc["completeness_findings"] != exp["completeness_findings"]:
            return "flagged", "records"
        return "flagged", None
    if e["n4lim"]:
        slot = e["n4lim"][0].split(":")[0]
        if not exp["evaluator_findings"]:
            return "n4_limit", "no_base_finding"
        exp["evaluator_findings"][-1] = {"code": "NonFiniteInput", "severity": "blocking", "subject_id": slot,
                                         "message": LIMIT_MESSAGE}
        return "n4_limit", (None if cc == exp else "differs_from_expected")
    return "n4_record_only", (None if cc == exp else "differs_from_expected")


def compare_runner(dump: str, base_lines, cand_lines, records):
    stats = collections.Counter()
    families = collections.defaultdict(collections.Counter)
    base_by_status = collections.Counter()
    schema_base = collections.Counter()
    plain_by_key: dict[str, str] = {}
    record_index = 0
    assert len(base_lines) == len(cand_lines), (dump, len(base_lines), len(cand_lines))
    for b, c in zip(base_lines, cand_lines):
        if b.startswith("#"):
            if b != c:
                violation("1_header", dump)
            continue
        key, mode, json_b, hash_b = split_run_line(dump, b)
        key_c, mode_c, json_c, hash_c = split_run_line(dump, c)
        assert (key, mode) == (key_c, mode_c), (dump, key, mode)
        record = records[record_index]
        record_index += 1
        assert record[0] == "R", record
        events = parse_events(record)
        family = runner_family(dump, key)
        families[family]["lines"] += 1
        stats["lines"] += 1
        flagged = {i for i, e in events.items() if e["formula"] and e["formula"][0] != "-"}
        n4 = {i for i, e in events.items() if e["n4in"] or e["n4lim"]}
        for i, e in events.items():
            if e["compare"] and e["compare"][0] != "-":
                violation("compare_flagged", f"{dump} {key} {mode}")
        if (json_c or "").startswith("PANIC") or (hash_c or "").startswith("PANIC"):
            violation("panic", f"{dump} {key} {mode}")
        # condition 5: plain == b0 on the candidate
        if mode in ("p", "plain"):
            plain_by_key[key] = hash_c if dump == "rv104_run" else json_c
        elif mode == "b0":
            mine = hash_c if dump == "rv104_run" else json_c
            if plain_by_key.get(key) != mine:
                violation("5_plain_ne_b0", f"{dump} {key}")
            else:
                stats["plain_eq_b0"] += 1
        # condition 6: schema
        if json_c is not None and not json_c.startswith("PANIC"):
            doc_c = json.loads(json_c)
            if schema_errors(doc_c):
                violation("6_candidate_schema", f"{dump} {key} {mode}: {schema_errors(doc_c)[0].message[:200]}")
            else:
                stats["schema_valid_candidate"] += 1
        if json_b is not None and not json_b.startswith("PANIC"):
            classes = classify_base_schema(json.loads(json_b))
            schema_base.update(classes)
            if classes:
                schema_base[f"lines_failing_mode_{mode}"] += 1
        if not flagged and not n4:
            if b != c:
                violation("1_unflagged_differs" if mode in ("p", "plain", "b0") else "4_bounded_differs", f"{dump} {key} {mode}\n{b[:400]}\n{c[:400]}")
            else:
                stats["unflagged_identical"] += 1
                families[family]["unflagged_identical"] += 1
            continue
        kind = "flagged" if flagged and not n4 else ("n4" if n4 and not flagged else "flagged_and_n4")
        stats[f"{kind}_lines"] += 1
        families[family][f"{kind}_lines"] += 1
        if json_b is None or json_c is None:
            stats[f"{kind}_hash_only"] += 1
            families[family][f"{kind}_hash_only"] += 1
            if b == c:
                stats[f"{kind}_hash_only_identical"] += 1
            continue
        doc_b, doc_c = json.loads(json_b), json.loads(json_c)
        if len(doc_b["checks"]) != len(doc_c["checks"]):
            violation("check_count", f"{dump} {key} {mode}")
            continue
        for i, (cb, cc) in enumerate(zip(doc_b["checks"], doc_c["checks"])):
            e = events.get(i)
            if i in n4 or i in flagged:
                klass, problem = verify_check(cb, cc, e)
                stats[f"checks_{klass}"] += 1
                families[family][f"checks_{klass}"] += 1
                if klass == "flagged":
                    base_by_status[(e["formula"][1], cb["status"])] += 1
                if problem:
                    violation(f"check_{klass}_{problem}", f"{dump} {key} {mode} check {i} {e['formula']} {e['n4in']} {e['n4lim']}\n{json.dumps(cb)[:700]}\n{json.dumps(cc)[:700]}")
                else:
                    stats[f"checks_{klass}_verified"] += 1
                    if klass.startswith("n4") and cc["status"] == cb["status"]:
                        stats["n4_status_unchanged"] += 1
                    elif klass.startswith("n4"):
                        violation("n4_status_changed", f"{dump} {key} {mode} check {i}")
            elif cb != cc:
                violation("1_unflagged_check_differs", f"{dump} {key} {mode} check {i}")
            else:
                stats["other_checks_identical"] += 1
        worst = max((ch["status"] for ch in doc_c["checks"]),
                    key=lambda s: {"USER_RULE_CHECKED": 0, "RULE_INPUTS_INCOMPLETE": 1, "USER_RULE_FAILED": 2}[s], default="RULE_INPUTS_INCOMPLETE")
        if doc_c["aggregate_status"] != worst:
            violation("aggregate", f"{dump} {key} {mode}")
    assert record_index == len(records), (dump, record_index, len(records))
    return {"stats": dict(stats), "families": {k: dict(v) for k, v in sorted(families.items())},
            "flagged_checks_by_consumer_and_base_status": {f"{k[0]}|{k[1]}": v for k, v in sorted(base_by_status.items())},
            "base_schema_failures": dict(schema_base)}


def fnv(text: str) -> int:
    h = 0xCBF29CE484222325
    for byte in text.encode():
        h ^= byte
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def reconstruct(full: Path, recorded: Path) -> dict:
    """RV104's recorded-format dump rebuilt from the all-full variant: hash-only
    lines keep only the FNV-1a hash of the JSON text. It must equal the dump of
    RV104's harness byte for byte, so the variant computes the same results."""
    out = []
    bad_hash = 0
    for line in full.read_text().splitlines():
        label, mode, h, text = line.split("\t", 3)
        if int(h, 16) != fnv(text):
            bad_hash += 1
        keep = mode == "p" or (mode in ("b0", "b1") and label.startswith("m_tab"))
        out.append(f"{label}\t{mode}\t{h}\t{text}" if keep else f"{label}\t{mode}\t{h}")
    rebuilt = ("\n".join(out) + "\n").encode()
    same = rebuilt == recorded.read_bytes()
    if not same or bad_hash:
        violation("rv104_full_reconstruction", f"{full}: same={same} bad_hash={bad_hash}")
    return {"lines": len(out), "fnv_mismatches": bad_hash, "rebuilt_sha256": hashlib.sha256(rebuilt).hexdigest(),
            "equals_recorded_format_dump": same}


def main():
    global VALIDATOR
    import jsonschema
    VALIDATOR = jsonschema.Draft202012Validator(json.loads(SCHEMA.read_text()))
    trees = {t: D / t for t in ("base", "ibase", "cand")}
    files = sorted(p.name for p in trees["base"].iterdir() if not p.name.endswith(".flags"))
    report["files"] = {}
    for name in files:
        entry = {t: sha(trees[t] / name) for t in trees}
        entry["lines"] = sum(1 for _ in (trees["base"] / name).open())
        entry["ibase_equals_base"] = entry["ibase"] == entry["base"]
        if not entry["ibase_equals_base"]:
            violation("instrumented_dump_differs", name)
        report["files"][name] = entry
    ee_flags = read_flags(trees["ibase"] / "ee_point.flags")
    rc_flags = read_flags(trees["ibase"] / "runner_crate.flags")
    report["flag_threads"] = {k: len(v) for k, v in list(ee_flags.items()) + list(rc_flags.items())}
    read = lambda t, n: (trees[t] / n).read_text().splitlines()
    report["evaluator"] = {}
    for dump, thread in (("i79_dump.tsv", "i73_point_mode_dump"), ("i79_extreme.tsv", "i79_extreme_dump"),
                         ("i79_table.tsv", "i79_table_dump")):
        report["evaluator"][dump] = compare_ee(dump.split(".")[0], read("base", dump), read("cand", dump), ee_flags[thread], "i79")
        report["evaluator"][dump + ".interval"] = compare_lines_identical(dump, trees["base"] / (dump + ".interval"), trees["cand"] / (dump + ".interval"))
        for side in ("base", "cand"):
            panics = (trees[side] / (dump + ".panics")).read_text()
            if panics:
                violation("panic", f"{side} {dump}.panics")
    for dump, thread in (("rv104_ee.txt", "rv104_evaluator_dump"), ("si1c_ee.txt", "si1c_evaluator_family")):
        report["evaluator"][dump] = compare_ee(dump.split(".")[0], read("base", dump), read("cand", dump), rc_flags[thread], "rv104")
    report["runner"] = {}
    for dump, thread in (("i79_run_plain.tsv", "i73_runner_dump"), ("i79_run_extreme.tsv", "i79_runner_extreme_dump"),
                         ("rv104_run.txt", "rv104_runner_dump"), ("si1c_run.txt", "si1c_runner_family")):
        report["runner"][dump] = compare_runner(dump.split(".")[0], read("base", dump), read("cand", dump), rc_flags[thread])
    full = {t: trees[t] / "rv104_run_full.txt" for t in trees}
    if all(p.exists() for p in full.values()):
        full_flags = read_flags(trees["ibase"] / "runner_full.flags")
        report["runner"]["rv104_run_full.txt"] = compare_runner(
            "rv104_run", read("base", "rv104_run_full.txt"), read("cand", "rv104_run_full.txt"),
            full_flags["rv104_runner_dump"])
        report["rv104_run_full_reconstruction"] = {t: reconstruct(full[t], trees[t] / "rv104_run.txt") for t in ("base", "cand")}
    report["violations"] = dict(report["violations"])
    report["examples"] = dict(report["examples"])
    REPORT.write_text(json.dumps(report, indent=1, sort_keys=True))
    print(json.dumps({"violations": report["violations"]}, indent=1))


main()
