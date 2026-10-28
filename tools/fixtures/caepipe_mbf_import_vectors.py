#!/usr/bin/env python3
"""Parity vectors between the MBF importer and its Rust port (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §6
slice 3).

The Rust engine crate `core/handoff/caepipe_mbf_engine` ports `core/handoff/caepipe_mbf/importer.py` byte for byte.
This writer records what the Python does, and the two checks hold each side to the record:

- `harvest` runs the importer's own test suites (importer, round trip, SI writer, private regression; the writer's
  preflight import included) with every `_Importer` call recorded: its inputs, and the SHA-256 of the model, the import
  report and the id map it returns (or the exception it raises). It adds the committed MBF corpus: the invented import
  fixtures and the authored cases (#1702), each imported as the private regression imports it, and each imported
  model written back by the SI writer and imported again bound; and the correlation cases written by the writer and
  imported bound and unbound. Corpus vectors keep their full output text. It records the writer's regenerated
  sidecar for each bound import (the Rust port takes the writer as an input until slice 2 ports it), the C library
  calls the import made, and numerical boundary vectors (float formatting and parsing at rounding boundaries,
  `math.hypot`, `sum`, the exact norm, the 90-digit arctangent, JSON errors and UTF-8 offsets).
- `--check` replays every recorded input through the Python and fails at the first output that differs: the Python
  side cannot change alone. The Rust test `tests/parity.rs` replays the same file: the Rust side cannot either.

Every input is invented (the test files' own MBFs, the invented fixtures and the authored cases); none is CAEPIPE
output. Usage, from the project root:

    python3 tools/fixtures/caepipe_mbf_import_vectors.py harvest   # rewrite the vectors
    python3 tools/fixtures/caepipe_mbf_import_vectors.py --check   # replay, write nothing
    python3 tools/fixtures/caepipe_mbf_import_vectors.py show <id> # print one vector's Python outputs
"""
from __future__ import annotations

import base64
import contextlib
import copy
import decimal
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import random
import struct
import sys
from typing import Any

sys.dont_write_bytecode = True
PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_mbf import importer  # noqa: E402
from core.handoff.caepipe_mbf import si_writer  # noqa: E402

# An import vector is a row [src, data, sc, mp, pc, sm, rb, lb, pd1, expect]: the index of the test or corpus item
# that made the call in `sources`; the MBF bytes (a `blobs` key); the sidecar (null, ["json", blob key] for JSON text,
# ["parsed", value key] for a parsed one); material_provenance, pressure_closure and shear_model (value keys or null);
# rebind_by_node_number (0/1); source_label (a value key); 1 where the call ran with PD1_VERSIONS = (); and the
# expectation, {"digest": 128 bits of SHA-256 over status and the three output digests} (with "status", "model",
# "report" and "id_map" text for corpus items) or {"raises": [type, message]}. A row's id is its index.
ROW = {name: index for index, name in enumerate(("src", "data", "sc", "mp", "pc", "sm", "rb", "lb", "pd1", "expect"))}
VECTORS = PROJECT / "fixtures/caepipe_mbf/invented/parity/import_vectors.json"
SUITES = ["tests/test_caepipe_mbf_importer.py", "tests/test_caepipe_mbf_round_trip.py",
          "tests/test_caepipe_mbf_si_writer.py", "tests/test_caepipe_private_mbf_import.py"]
CORPUS_MBF = sorted([*(PROJECT / "fixtures/caepipe_mbf/invented/import").glob("*.mbf")])
AUTHORED = PROJECT / "validation/caepipe_correlation/authored"
CASES = PROJECT / "validation/caepipe_correlation/cases"
DOCUMENT_KIND = "openpipestress.caepipe_mbf_engine.import_parity_vectors"
LIBM = ("acos", "asin", "atan2", "sin", "tan", "hypot")
ORIGINAL_CHECKED_JSON = importer._Importer.checked_json_domain


# A non-finite float in a parsed input (JSON has none): {NONFINITE: "nan" | "inf" | "-inf"}.
NONFINITE = "\u0000float"


def restore(value: Any) -> Any:
    """`value` with the tagged non-finite floats put back."""
    if type(value) is dict:
        if list(value) == [NONFINITE]:
            return {"nan": math.nan, "inf": math.inf, "-inf": -math.inf}[value[NONFINITE]]
        return {key: restore(item) for key, item in value.items()}
    if type(value) is list:
        return [restore(item) for item in value]
    return value


class Unrepresentable(Exception):
    """A Python input with no exact JSON form (a NaN, a tuple, a non-string key ...): the vector is not recorded."""


# ---------------------------------------------------------------------------
# lossless JSON forms of inputs
# ---------------------------------------------------------------------------
def lossless(value: Any) -> Any:
    """`value` when JSON carries it unchanged (types included), else Unrepresentable."""
    if value is None or type(value) in (bool, str):
        if type(value) is str:
            try:
                value.encode("utf-8")
            except UnicodeEncodeError:
                raise Unrepresentable("a lone surrogate") from None
        return value
    if type(value) is int:
        if len(str(abs(value))) > 4300:
            raise Unrepresentable("an integer beyond the str() digit limit")
        return value
    if type(value) is float:
        if not math.isfinite(value):
            return {NONFINITE: "nan" if math.isnan(value) else ("inf" if value > 0 else "-inf")}
        return value
    if type(value) is list:
        return [lossless(item) for item in value]
    if type(value) is dict:
        if not all(type(key) is str for key in value):
            raise Unrepresentable("a non-string key")
        return {lossless(key): lossless(item) for key, item in value.items()}
    raise Unrepresentable(f"a {type(value).__name__}")


def dumps(value: Any) -> str:
    return json.dumps(value, ensure_ascii=True, allow_nan=False)


def same(a: Any, b: Any) -> bool:
    """Type-exact equality (1 != 1.0 != True, -0.0 != 0.0, key order kept)."""
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return list(a) == list(b) and all(same(a[key], b[key]) for key in a)
    if type(a) is list:
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    if type(a) is float:
        return struct.pack("<d", a) == struct.pack("<d", b) or (math.isnan(a) and math.isnan(b))
    return a == b


def diff(a: Any, b: Any, path: list, ops: list) -> None:
    """Edit ops turning `a` into `b` with dict order kept: ["set", path, v], ["del", path], ["add", path, v]."""
    if type(a) is dict and type(b) is dict:
        common_a = [key for key in a if key in b]
        common_b = [key for key in b if key in a]
        added = [key for key in b if key not in a]
        if common_a != common_b or list(b) != common_b + added:
            ops.append(["set", path, b])
            return
        for key in a:
            if key not in b:
                ops.append(["del", path + [key]])
        for key in common_a:
            diff(a[key], b[key], path + [key], ops)
        for key in added:
            ops.append(["add", path + [key], b[key]])
        return
    if type(a) is list and type(b) is list and len(a) == len(b):
        for index, (x, y) in enumerate(zip(a, b)):
            diff(x, y, path + [index], ops)
        return
    if not same(a, b):
        ops.append(["set", path, b])


def apply(base: Any, ops: list) -> Any:
    value = copy.deepcopy(base)
    for op in ops:
        kind, path = op[0], op[1]
        if not path:
            value = copy.deepcopy(op[2])
            continue
        parent = value
        for step in path[:-1]:
            parent = parent[step]
        if kind == "del":
            del parent[path[-1]]
        else:
            parent[path[-1]] = copy.deepcopy(op[2])
    return value


# ---------------------------------------------------------------------------
# recording
# ---------------------------------------------------------------------------
class Store:
    """Deduplicated inputs: MBF and JSON-text blobs, parsed values as patches of earlier ones."""

    def __init__(self) -> None:
        self.blobs: dict[str, dict] = {}
        self.values: dict[str, Any] = {}
        self.value_plain: dict[str, Any] = {}
        self.recent: list[str] = []
        self.text_latest: dict[str, str] = {}

    def blob(self, data: bytes | str) -> str:
        raw = data.encode("utf-8") if isinstance(data, str) else bytes(data)
        key = hashlib.sha256(raw).hexdigest()[:16]
        if key in self.blobs:
            return key
        try:
            text = raw.decode("utf-8")
        except UnicodeDecodeError:
            self.blobs[key] = {"b64": base64.b64encode(raw).decode("ascii")}
            return key
        family = "text"
        base_key = self.text_latest.get(family)
        if base_key is not None and "text" in self.blobs[base_key] and len(text) > 512:
            base = self.blob_text(base_key)
            prefix = 0
            limit = min(len(base), len(text))
            while prefix < limit and base[prefix] == text[prefix]:
                prefix += 1
            suffix = 0
            while suffix < limit - prefix and base[-1 - suffix] == text[-1 - suffix]:
                suffix += 1
            middle = text[prefix:len(text) - suffix]
            if prefix + suffix > len(text) // 2:
                self.blobs[key] = {"text": None, "base": base_key, "prefix": prefix, "suffix": suffix, "middle": middle}
                self.text_latest[family] = key
                return key
        self.blobs[key] = {"text": text}
        self.text_latest[family] = key
        return key

    def blob_text(self, key: str) -> str:
        entry = self.blobs[key]
        if entry.get("text") is not None:
            return entry["text"]
        base = self.blob_text(entry["base"])
        return base[:entry["prefix"]] + entry["middle"] + (base[len(base) - entry["suffix"]:] if entry["suffix"] else "")

    def value(self, value: Any) -> str | None:
        """A key for a parsed input in the value table (None for None), stored whole or as edits of a recent one."""
        if value is None:
            return None
        plain = lossless(value)
        text = dumps(plain)
        key = hashlib.sha256(text.encode("utf-8") + repr(type_signature(plain)).encode()).hexdigest()[:16]
        if key in self.values:
            return key
        stored: Any = {"value": plain}
        best = len(text) // 2
        if type(plain) is dict and len(text) > 200:
            for base_key in self.recent[-48:]:
                ops: list = []
                diff(self.value_plain[base_key], plain, [], ops)
                size = len(dumps(ops))
                if size < best:
                    stored, best = {"base": base_key, "ops": ops}, size
            self.recent.append(key)
        self.values[key] = stored
        self.value_plain[key] = plain
        return key


def type_signature(value: Any) -> Any:
    if type(value) is dict:
        return [(key, type_signature(item)) for key, item in value.items()]
    if type(value) is list:
        return [type_signature(item) for item in value]
    return type(value).__name__


def output_digest(status: str, model: bytes | None, report: bytes, id_map: bytes) -> str:
    parts = [status, hashlib.sha256(model).hexdigest() if model is not None else "-",
             hashlib.sha256(report).hexdigest(), hashlib.sha256(id_map).hexdigest()]
    return hashlib.sha256("|".join(parts).encode("ascii")).hexdigest()[:32]


class Recorder:
    """Wraps `_Importer` and the writer's regeneration while the suites run."""

    def __init__(self, store: Store) -> None:
        self.store = store
        self.imports: list[dict] = []
        self.seen: dict[str, int] = {}
        self.orientations: list[dict] = []
        self.oriented: set[str] = set()
        self.regenerations: dict[str, dict] = {}
        self.libm: dict[str, list] = {}
        self.skipped: dict[str, int] = {}
        self.source = "?"
        self.sources: dict[str, int] = {}
        self.full = False

    def skip(self, reason: str) -> None:
        self.skipped[reason] = self.skipped.get(reason, 0) + 1

    @contextlib.contextmanager
    def installed(self):
        recorder = self
        original_init, original_run = importer._Importer.__init__, importer._Importer.run
        original_regenerate = importer.regenerate_caepipe_sidecar
        original_math = importer.math

        def init(self, data, sidecar, provenance, rebind, source_label, closure=None, shear_model=None,
                 derive_only=False):
            original_init(self, data, sidecar, provenance, rebind, source_label, closure, shear_model, derive_only)
            try:
                self._parity_input = recorder.encode_input(data, sidecar, provenance, rebind, source_label, closure,
                                                           shear_model, derive_only)
            except Unrepresentable as error:
                self._parity_input = None
                recorder.skip(f"input not representable in JSON: {error}")

        def run(self):
            entry = getattr(self, "_parity_input", None)
            if entry is not None and type(self).checked_json_domain is not ORIGINAL_CHECKED_JSON:
                recorder.skip("checked_json_domain patched by the test")
                entry = None
            if entry is not None:
                if tuple(importer.PD1_VERSIONS) != (importer.MBF_VERSION,):
                    entry["pd1_disabled"] = True
                if tuple(importer.PD1_VERSIONS) not in ((), (importer.MBF_VERSION,)):
                    recorder.skip("PD1_VERSIONS patched to another value")
                    entry = None
            try:
                result = original_run(self)
            except Exception as error:  # the Python importer raises: the port must raise alike
                if entry is not None:
                    recorder.record(self, entry, None, error)
                raise
            if entry is not None:
                recorder.record(self, entry, result, None)
            return result

        def regenerate(model, *, mbf_sha256, derived_y_reference, **reference):
            sidecar, refusals = original_regenerate(model, mbf_sha256=mbf_sha256,
                                                    derived_y_reference=derived_y_reference, **reference)
            key = regeneration_key(model, mbf_sha256, derived_y_reference, reference)
            outcome = ({"refused": sorted({item["code"] for item in refusals})} if sidecar is None
                       else {"sidecar": lossless(sidecar)})
            recorder.regenerations[key] = outcome
            return sidecar, refusals

        importer._Importer.__init__, importer._Importer.run = init, run
        importer.regenerate_caepipe_sidecar = regenerate
        importer.math = LibmProxy(original_math, self.libm)
        try:
            yield
        finally:
            importer._Importer.__init__, importer._Importer.run = original_init, original_run
            importer.regenerate_caepipe_sidecar = original_regenerate
            importer.math = original_math

    def encode_input(self, data, sidecar, provenance, rebind, source_label, closure, shear_model, derive_only):
        if not isinstance(data, (bytes, bytearray)):
            raise Unrepresentable("data that is not bytes")
        if type(source_label) is not str:
            raise Unrepresentable("a source_label that is not a str")
        if type(rebind) is not bool:
            raise Unrepresentable("a rebind flag that is not a bool")
        entry: dict[str, Any] = {"data": self.store.blob(bytes(data))}
        if derive_only:
            entry["derive_only"] = True
            return entry
        if sidecar is None:
            entry["sc"] = None
        elif isinstance(sidecar, (bytes, bytearray)):
            entry["sc"] = ["json", self.store.blob(bytes(sidecar))]
        elif isinstance(sidecar, str):
            try:
                sidecar.encode("utf-8")
            except UnicodeEncodeError:
                raise Unrepresentable("a str sidecar with a lone surrogate (the Rust API takes bytes)") from None
            entry["sc"] = ["json", self.store.blob(sidecar)]
        else:
            entry["sc"] = ["parsed", self.store.value(sidecar)]
        entry["mp"] = self.store.value(provenance)
        entry["pc"] = self.store.value(closure)
        entry["sm"] = self.store.value(shear_model)
        entry["rb"] = rebind
        entry["lb"] = self.store.value(source_label)
        return entry

    def record(self, instance, entry: dict, result, error) -> None:
        if entry.get("derive_only"):
            key = entry["data"]
            if key in self.oriented:
                return
            self.oriented.add(key)
            if error is not None:
                self.orientations.append({"data": key, "raises": [type(error).__name__, str(error)]})
            else:
                derived = instance.derived
                self.orientations.append({"data": key, "derived": None if derived is None else [
                    [a, b, list(vector)] for (a, b), vector in derived.items()]})
            return
        identity = hashlib.sha256(dumps(entry).encode("utf-8")).hexdigest()
        if identity in self.seen and not self.full:
            return
        if self.source not in self.sources:
            self.sources[self.source] = len(self.sources)
        if error is not None:
            expect = {"raises": [type(error).__name__, str(error)]}
        else:
            expect = expected(result, full=self.full)
        row = [self.sources[self.source], entry["data"], entry["sc"], entry["mp"], entry["pc"], entry["sm"],
               int(entry["rb"]), entry["lb"], int(entry.get("pd1_disabled", False)), expect]
        if identity in self.seen:  # a corpus call a suite made first: the row takes the corpus source and full text
            self.imports[self.seen[identity]] = row
            return
        self.seen[identity] = len(self.imports)
        self.imports.append(row)


def expected(result, *, full: bool) -> dict:
    model, report, id_map = result.model_bytes(), result.report_bytes(), result.id_map_bytes()
    out = {"digest": output_digest(result.status, model, report, id_map)}
    if full:
        out["status"] = result.status
        out.update(model=None if model is None else model.decode("ascii"), report=report.decode("ascii"),
                   id_map=id_map.decode("ascii"))
    return out


def regeneration_key(model, mbf_sha256: str, derived: Any, reference: dict) -> str:
    """The writer call's identity: the model's canonical bytes, the digest, the derived list and the reference."""
    shown = {key: struct.pack("<d", float(value)).hex() for key, value in sorted(reference.items())}
    parts = [hashlib.sha256(importer.canonical_bytes(model)).hexdigest(), mbf_sha256, dumps(list(derived)),
             dumps(shown)]
    return hashlib.sha256("\n".join(parts).encode("utf-8")).hexdigest()


class LibmProxy:
    """The importer's `math` module, recording the C library (and hypot) calls the import makes."""

    def __init__(self, module, calls: dict) -> None:
        self._module, self._calls = module, calls

    def __getattr__(self, name: str):
        target = getattr(self._module, name)
        if name not in LIBM:
            return target
        calls = self._calls

        def wrapped(*args):
            result = target(*args)
            key = name + ":" + ",".join(struct.pack("<d", float(arg)).hex() for arg in args)
            calls[key] = struct.pack("<d", result).hex()
            return result
        return wrapped


# ---------------------------------------------------------------------------
# harvest
# ---------------------------------------------------------------------------
class SourcePlugin:
    """Names each recorded call by the test that made it."""

    def __init__(self, recorder: Recorder) -> None:
        self.recorder = recorder

    def pytest_runtest_setup(self, item):
        self.recorder.source = item.nodeid

    def pytest_runtest_call(self, item):
        self.recorder.source = item.nodeid


def harvest_suites(recorder: Recorder) -> None:
    import pytest
    code = pytest.main(["-q", "-p", "no:cacheprovider", "-p", "no:randomly", "-p", "no:xdist", *SUITES],
                       plugins=[SourcePlugin(recorder)])
    if code not in (0, pytest.ExitCode.OK):
        raise SystemExit(f"the importer suites failed under recording (exit {code}); the vectors are not written")


def harvest_corpus(recorder: Recorder) -> None:
    recorder.full = True
    for path in CORPUS_MBF:
        provenance = json.loads(path.with_name(path.stem + ".provenance.json").read_text(encoding="utf-8"))
        data = path.read_bytes()
        recorder.source = f"corpus:{path.relative_to(PROJECT)}"
        result = importer.import_caepipe_mbf(data, material_provenance=provenance.get("material_provenance"),
                                             source_label=path.name)
        write_back(recorder, result, data, f"corpus:{path.relative_to(PROJECT)} written back")
    manifest = json.loads((AUTHORED / "manifest.json").read_text(encoding="utf-8"))
    for case in manifest["cases"]:
        data = (AUTHORED / case["file"]).read_bytes()
        declaration = {"*": {"origin": "project_invented", "source_name": f"invented authored case {case['case_id']}"}}
        options = {"pressure_closure": case["pressure_closure"]} if case.get("pressure_closure") else {}
        recorder.source = f"corpus:authored/{case['case_id']}"
        result = importer.import_caepipe_mbf(data, material_provenance=declaration,
                                             source_label=f"{case['case_id']}/caepipe_in/model.mbf", **options)
        write_back(recorder, result, data, f"corpus:authored/{case['case_id']} written back", options)
    for path in sorted(CASES.glob("*.json")):
        case = json.loads(path.read_text(encoding="utf-8"))
        recorder.source = f"corpus:cases/{case['case_id']}"
        written = si_writer.render_caepipe_mbf_si(case["model"], title=case.get("title", case["case_id"]),
                                                  reference_temperature_c=case.get("reference_temperature_c", 20.0))
        if written.status != "written":
            continue
        data = written.text.encode("ascii")
        sidecar = written.sidecar(hashlib.sha256(data).hexdigest())
        declaration = {"*": {"origin": "project_invented", "source_name": f"invented correlation case {case['case_id']}"}}
        importer.import_caepipe_mbf(data, sidecar=sidecar, source_label=f"{case['case_id']}/caepipe_in/model.mbf")
        importer.import_caepipe_mbf(data, sidecar=json.dumps(sidecar).encode("utf-8"),
                                    source_label=f"{case['case_id']}/caepipe_in/model.mbf")
        importer.import_caepipe_mbf(data, material_provenance=declaration,
                                    source_label=f"{case['case_id']}/caepipe_in/model.mbf")
        importer.import_caepipe_mbf(data, sidecar=sidecar, material_provenance=declaration, rebind_by_node_number=True,
                                    source_label=f"{case['case_id']}/caepipe_out/exported.mbf")
    recorder.full = False


def coverage_mbf(rows: list[str], *, sections: bool = True) -> bytes:
    lines = ["INVENTED PARITY COVERAGE", "VERSION", "15.10", "OPTIONS", "SI", "OPTEMP", "20"]
    if sections:
        lines += ["MATERIAL", "M1,0,0.3,1,1,CS,invented steel", "M1,20,200000,1.2e-05", "PIPE",
                  "P1,168.3,7.11,0,0,0,0,0,0", "LOADS", "L1,20,10,0", "LAYOUT", *rows]
    return ("\r\n".join(lines) + "\r\n").encode("ascii")


def harvest_coverage(recorder: Recorder) -> None:
    """Invented MBFs for what no suite test reaches: a file without MATERIAL, PIPE and LAYOUT
    (MBF-IMPORT-SECTION-MISSING); a pressurized bend 1.5e-8 rad short of reversing, whose arc the runtime does not form
    on the values in m (MBF-IMPORT-PRESSURE-GEOMETRY-INVALID); and inputs read through Python's own number rules."""
    recorder.full = True
    declaration = {"*": {"origin": "project_invented", "source_name": "invented parity coverage"}}
    closure = {"closure_transfer": "transfers_to_wall", "provenance": "invented parity coverage declaration"}
    recorder.source = "coverage:section-missing"
    importer.import_caepipe_mbf(coverage_mbf([], sections=False), material_provenance=declaration,
                                source_label="coverage")
    theta, radius = math.pi - 1.5e-8, 1
    leg = 3 * radius * math.tan(theta / 2)
    rows = ["F10,KA,MM1,PP1,CL=L1", f"T20,X{leg:.6f},KI,B{radius},CFF=1",
            f"T30,X{-leg * math.cos(math.pi - theta):.6f},Y{leg * math.sin(math.pi - theta):.9g},KA"]
    recorder.source = "coverage:pressure-geometry-invalid"
    importer.import_caepipe_mbf(coverage_mbf(rows), material_provenance=declaration, pressure_closure=closure,
                                source_label="coverage")
    # Inputs the port reads through Python's own number rules: an English length with 450 decimal places (Fraction
    # reads it exactly; the position underflows), and non-ASCII decimal digits, which `\d` and int() accept after the
    # character refusal (a node number, an English length and a zero section).
    english = lambda rows, extra=(): ("\r\n".join(["INVENTED PARITY COVERAGE", "VERSION", "15.10", "OPTIONS", "HGRA",
                                                    "OPTEMP", "70", "MATERIAL", "M1,0,0.3,1,1,CS,invented steel",
                                                    "M1,70,29000000,6.5e-06", "PIPE", "P1,6.625,0.28,0,0,0,0,0,0",
                                                    "LOADS", "L1,70,0,0", *extra, "LAYOUT", *rows]) + "\r\n").encode("utf-8")
    for name, data in (
            ("english-length-underflow", english(["F10,KA,MM1,PP1,CL=L1", "T20,X0." + "0" * 450 + "1"])),
            ("unicode-digit-node", english(["F10,KA,MM1,PP1,CL=L1", "T\u0662\u0660,X10"])),
            ("unicode-digit-length", english(["F10,KA,MM1,PP1,CL=L1", "T20,X\u0663'\u0664"])),
            ("unicode-digit-zero-section", english(["F10,KA,MM1,PP1,CL=L1", "T20,X10"], ("SEISMIC", "\u0660,0")))):
        recorder.source = f"coverage:{name}"
        importer.import_caepipe_mbf(data, material_provenance=declaration, source_label="coverage")
    recorder.full = False


def write_back(recorder: Recorder, result, data: bytes, source: str, options: dict | None = None) -> None:
    """An imported model written by the SI writer and imported again, bound and unbound (the round trip)."""
    if result.status != "imported":
        return
    recorder.source = source
    written = si_writer.render_caepipe_mbf_si(result.model, title="WRITTEN BACK", max_layout_rows=None,
                                              **optemp_reference(data, result))
    if written.status != "written":
        return
    text = written.text.encode("ascii")
    sidecar = written.sidecar(hashlib.sha256(text).hexdigest())
    importer.import_caepipe_mbf(text, sidecar=json.dumps(sidecar).encode("utf-8"), source_label="written back")
    importer.import_caepipe_mbf(text, material_provenance={"*": {"origin": "project_invented",
                                                                 "source_name": "invented written-back case"}},
                                source_label="written back", **(options or {}))


def optemp_reference(data: bytes, result) -> dict:
    """The writer's reference temperature for a written-back model: the file's OPTEMP, in its unit."""
    probe = importer._Importer(data, None, None, False, "probe")
    with contextlib.suppress(importer._Refused):
        probe.decode()
        probe.split_sections()
        probe.read_version()
        probe.read_options()
        probe.read_analysis_options()
    if probe.t_ref is None:
        return {}
    return ({"reference_temperature_c": probe.t_ref.value} if result.source["unit_mode"] == "SI"
            else {"reference_temperature_f": probe.t_ref.value})


# ---------------------------------------------------------------------------
# numerical boundary vectors
# ---------------------------------------------------------------------------
def bits(value: float) -> str:
    return struct.pack("<d", value).hex()


def boundary_floats() -> list[float]:
    """Floats at formatting and rounding boundaries, then a seeded random spread over every binade."""
    values = [0.0, -0.0, 5e-324, -5e-324, 2.2250738585072014e-308, 2.225073858507201e-308, 1.7976931348623157e308,
              0.1, 0.2, 0.3, 0.1 + 0.2, 1.0, -1.0, 1.5, 123.0, 1e-05, 1e-04, 0.0001, 0.00001, 1e15, 1e16, 1e17,
              9999999999999998.0, 1e16 - 2, 1e22, 1e23, 9007199254740993.0, 9007199254740992.0, 2.0 ** 53 + 2,
              999999999999975.2, 279836846559720.12, 0.30000000000000004, 1 / 3, 2 / 3, math.pi, math.e,
              4.4482216152605, 0.45359237, 9.80665, 25.4, 6894.757293168361, 175.12683524647636, 386.4, 0.0254,
              1.0000000000000002, 0.9999999999999999, 5e-324 * 3, 1e-310, 1e300, 1.2e-05, 200000.0, 7833.0]
    for k in range(-1074, 1024):
        values.append(2.0 ** k)
        values.append(math.nextafter(2.0 ** k, math.inf))
        if k > -1074:
            values.append(math.nextafter(2.0 ** k, 0.0))
    for k in range(-325, 309):
        with contextlib.suppress(OverflowError):
            value = float(f"1e{k}")
            values += [value, math.nextafter(value, math.inf), math.nextafter(value, 0.0)]
            values.append(float(f"5e{k}"))
    rng = random.Random(17)
    for _ in range(4000):
        values.append(struct.unpack("<d", struct.pack("<Q", rng.getrandbits(64)))[0])
    for _ in range(1500):
        values.append(round(rng.uniform(-1e6, 1e6), rng.randint(0, 12)))
    return [value for value in values if math.isfinite(value)]


def numeric_vectors() -> dict:
    floats = boundary_floats()
    rng = random.Random(23)
    repr_vectors = [[bits(value), repr(value)] for value in floats]
    g_vectors = [[bits(value), format(value, ".6g")] for value in floats[::5]]
    # Python's float(text) of decimal numerals at rounding boundaries (halfway cases ties to even)
    parse_vectors = []
    for value in floats[::14]:
        exact = Fraction(value)
        up = Fraction(math.nextafter(value, math.inf)) if value < 1.7976931348623157e308 else exact
        middle = (exact + up) / 2
        if 0 < middle < Fraction(10) ** 30:
            text = exact_decimal(middle)  # a dyadic midpoint: exact in decimal, a tie between two binary64 values
            parse_vectors.append([text, bits(float(Fraction(text))), bits(float(text))])
        parse_vectors.append([repr(value), bits(float(Fraction(repr(value)))), bits(float(repr(value)))])
    hypot_vectors = []
    for _ in range(2000):
        args = [rng.uniform(-1, 1) * 2.0 ** rng.randint(-1074, 1023) for _ in range(rng.choice((2, 3)))]
        hypot_vectors.append([[bits(arg) for arg in args], bits(math.hypot(*args))])
    for args in ([3.0, 4.0], [5e-324, 5e-324], [1e308, 1e308], [0.0, -0.0], [1e-310, 3e-310, 2e-320],
                 [1.0, 1e-17, 1e-17], [math.inf, math.nan], [math.nan, 1.0]):
        hypot_vectors.append([[bits(arg) for arg in args], bits(math.hypot(*args))])
    sum_vectors = []
    for _ in range(2000):
        args = [rng.uniform(-1, 1) * 10.0 ** rng.randint(-20, 20) for _ in range(rng.choice((2, 3)))]
        sum_vectors.append([[bits(arg) for arg in args], bits(sum(args))])
    for args in ([-0.0, -0.0], [1e308, 1e308, -1e308], [0.1, 0.2, 0.3], [1.0, 1e100, 1.0, -1e100],
                 [-0.0], [math.inf, -math.inf], [1e16, 1.0, -1e16]):
        sum_vectors.append([[bits(arg) for arg in args], bits(sum(args))])
    norm_vectors = []
    for _ in range(1500):
        args = [rng.uniform(-1, 1) * 2.0 ** rng.randint(-1074, 1023) for _ in range(3)]
        norm_vectors.append([[bits(arg) for arg in args], bits(importer.correct_norm3(*args))])
    for args in ([3.0, 4.0, 0.0], [1e-200, 1e-200, 0.0], [2.0 ** -1074, 0.0, 0.0], [3 * 2.0 ** -1074, 4 * 2.0 ** -1074, 0.0],
                 [1e308, 1e308, 1e308], [0.0, 0.0, 0.0]):
        norm_vectors.append([[bits(arg) for arg in args], bits(importer.correct_norm3(*args))])
    atan_vectors = []
    for _ in range(1500):
        y, x = abs(rng.uniform(0, 1) * 2.0 ** rng.randint(-60, 60)), abs(rng.uniform(0, 1) * 2.0 ** rng.randint(-60, 60))
        result = importer.twice_atan2_nonnegative(y, x)
        atan_vectors.append([bits(y), bits(x), None if result is None else bits(result)])
    for y, x in ((0.0, 2.0), (2.0, 0.0), (1.0, 1.0), (1e-300, 1.0), (1.0, 1e-300), (5e-324, 1.0), (1e-3, 2.0),
                 (0.0, 0.0), (-1.0, 1.0)):
        result = importer.twice_atan2_nonnegative(y, x)
        atan_vectors.append([bits(y), bits(x), None if result is None else bits(result)])
    return {"repr": repr_vectors, "format_g6": g_vectors, "parse": parse_vectors, "hypot": hypot_vectors,
            "sum": sum_vectors, "correct_norm3": norm_vectors, "twice_atan2": atan_vectors,
            "json_errors": json_vectors(), "utf8_offsets": utf8_vectors()}


def exact_decimal(value: Fraction) -> str:
    """The terminating decimal expansion of a dyadic rational, exactly."""
    k = value.denominator.bit_length() - 1
    assert value.denominator == 1 << k
    digits = str(value.numerator * 5 ** k).rjust(k + 1, "0")
    whole, fraction = digits[:len(digits) - k], digits[len(digits) - k:]
    return whole + ("." + fraction if fraction else "")


def json_vectors() -> list:
    """parse_sidecar_json on malformed and edge JSON text: the value (as dumps text) or the findings."""
    texts = ['{"a":1,}', '[1,]', '{', '[', '{"a" 1}', '"\\u12"', '"abc', '"\\x"', '[1 2]', '-', '01', '1.', '1e',
             '{"a":NaN}', '{"a":Infinity}', '{"a":-Infinity}', '﻿{}', '{} x', '{"a":1,"a":2}',
             '{"a":{"b":1,"b":2},"a":{"c":3,"c":4}}', '{"node_map":{"x":1,"x":2}}', '[{"a":1,"a":1}]', '',
             ' ', 'null', 'true', 'false', 'nul', 'tru', 'fals', '1e400', '-1e400', '1e-400', '-0', '-0.0',
             '1E+2', '1e-2', '0.5e', '"\\ud83d\\ude00"', '"\\u00e9"', '"\t"', '"\\/"', '{"a"', '{"a":', '{"a":1',
             '[1', '[1,', '{"a":1 "b":2}', '{1:2}', '"x" "y"', '\n\n  {\n"a"  :  [1, 2, ,3]}', '[-]', '[- 1]',
             '[1.5e+3, 2E-2, -0, 0e0]', '{"a":"\\"\\\\\\b\\f\\n\\r\\t"}', '"\\u0041\\u004"', '"\\uD800\\u0041"',
             '1' * 4301, '-' + '1' * 4301, '[' * 3 + ']' * 3, '{"sidecar_format": 3}', '{"a": [1, {"b": null}]}']
    out = []
    for text in texts:
        try:
            value = importer.parse_sidecar_json(text.encode("utf-8"))
        except importer.SidecarJsonInvalid as error:
            out.append([text, {"invalid": [list(item) for item in error.findings]}])
            continue
        try:
            out.append([text, {"value": dumps(lossless(value))}])
        except Unrepresentable as error:
            out.append([text, {"unrepresentable": str(error)}])
    for raw in (b"\xff", b'{"a":"\xc3"}', b'{"a":"\xed\xa0\x80"}', b"\xf4\x90\x80\x80", b"\xc0\xaf", b"ab\xe2\x82",
                b'{"a":1}\xe2\x28\xa1'):
        try:
            importer.parse_sidecar_json(raw)
            out.append([base64.b64encode(raw).decode(), {"value": "?"}])
        except importer.SidecarJsonInvalid as error:
            out.append(["b64:" + base64.b64encode(raw).decode(), {"invalid": [list(item) for item in error.findings]}])
    return out


def utf8_vectors() -> list:
    out = []
    for raw in (b"\xff", b"a\xe2\x28\xa1", b"\xed\xa0\x80", b"\xf4\x90\x80\x80", b"\xc0\xaf", b"ab\xe2\x82",
                b"\xe2\x82", b"\xf0\x9f\x98", b"ok\x80", b"\xef\xbf\xbe", b"\xf8\x88\x80\x80\x80", b"\xe0\x80\x80"):
        try:
            raw.decode("utf-8")
            out.append([base64.b64encode(raw).decode(), None])
        except UnicodeDecodeError as error:
            out.append([base64.b64encode(raw).decode(), error.start])
    return out


# ---------------------------------------------------------------------------
# document, check, show
# ---------------------------------------------------------------------------
def harvest() -> dict:
    store = Store()
    recorder = Recorder(store)
    with recorder.installed():
        harvest_suites(recorder)
        harvest_corpus(recorder)
        harvest_coverage(recorder)
    sources = sorted(recorder.sources, key=recorder.sources.__getitem__)
    corpus = sorted({row[ROW["data"]] for row in recorder.imports if sources[row[ROW["src"]]].startswith("corpus:")})
    return {
        "document_kind": DOCUMENT_KIND,
        "description": ("MBF importer parity vectors, written by tools/fixtures/caepipe_mbf_import_vectors.py from "
                        "core/handoff/caepipe_mbf/importer.py over its own test suites and the committed invented MBF "
                        "corpus. tests/test_caepipe_mbf_import_parity.py replays them through the Python; "
                        "core/handoff/caepipe_mbf_engine/tests/parity.rs through the Rust port."),
        "python": ".".join(map(str, sys.version_info[:3])),
        "unicode": __import__("unicodedata").unidata_version,
        "counts": {"imports": len(recorder.imports), "orientations": len(recorder.orientations),
                   "regenerations": len(recorder.regenerations), "libm_calls": len(recorder.libm),
                   "corpus_inputs": len(corpus), "skipped": dict(sorted(recorder.skipped.items()))},
        "sources": sources,
        "blobs": store.blobs,
        "values": store.values,
        "regenerations": recorder.regenerations,
        "libm": dict(sorted(recorder.libm.items())),
        "imports": recorder.imports,
        "orientations": recorder.orientations,
        "sets": round_trip_sets(),
        "numeric": numeric_vectors(),
    }


def round_trip_sets() -> dict:
    """The round trip's reversible and byte-reproducible subsets (Design §18.2), by model name: each model's bound
    import carries its name as source_label."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("parity_round_trip", PROJECT / "tests/test_caepipe_mbf_round_trip.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return {"reversible": list(module.REVERSIBLE), "byte_reproducible": list(module.BYTE_REPRODUCIBLE)}


def render(document: dict) -> str:
    return json.dumps(document, indent=None, separators=(",", ":"), ensure_ascii=True, allow_nan=False) + "\n"


def load() -> dict:
    return json.loads(VECTORS.read_text(encoding="utf-8"))


def blob_bytes(document: dict, key: str, cache: dict) -> bytes:
    if key in cache:
        return cache[key]
    entry = document["blobs"][key]
    if "b64" in entry:
        raw = base64.b64decode(entry["b64"])
    elif entry.get("text") is not None:
        raw = entry["text"].encode("utf-8")
    else:
        base = blob_bytes(document, entry["base"], cache).decode("utf-8")
        tail = base[len(base) - entry["suffix"]:] if entry["suffix"] else ""
        raw = (base[:entry["prefix"]] + entry["middle"] + tail).encode("utf-8")
    cache[key] = raw
    return raw


def value_of(document: dict, key: str, cache: dict) -> Any:
    if key in cache:
        return copy.deepcopy(cache[key])
    entry = document["values"][key]
    value = entry["value"] if "value" in entry else apply(value_of(document, entry["base"], cache), entry["ops"])
    cache[key] = value
    return copy.deepcopy(value)


def value_ref(document: dict, key: str | None, cache: dict) -> Any:
    return None if key is None else restore(value_of(document, key, cache))


def replay(document: dict, row: list, caches: tuple[dict, dict]):
    vector = dict(zip(ROW, row))
    data = blob_bytes(document, vector["data"], caches[0])
    sidecar = vector["sc"]
    if sidecar is not None:
        sidecar = (blob_bytes(document, sidecar[1], caches[0]) if sidecar[0] == "json"
                   else value_ref(document, sidecar[1], caches[1]))
    saved = importer.PD1_VERSIONS
    importer.PD1_VERSIONS = () if vector["pd1"] else saved
    try:
        return importer.import_caepipe_mbf(data, sidecar=sidecar,
                                           material_provenance=value_ref(document, vector["mp"], caches[1]),
                                           pressure_closure=value_ref(document, vector["pc"], caches[1]),
                                           shear_model=value_ref(document, vector["sm"], caches[1]),
                                           rebind_by_node_number=bool(vector["rb"]),
                                           source_label=value_ref(document, vector["lb"], caches[1]))
    finally:
        importer.PD1_VERSIONS = saved


def version_matches(document: dict) -> bool:
    return document["python"].split(".")[:2] == [str(part) for part in sys.version_info[:2]]


def check_vector(document: dict, row: list, caches: tuple[dict, dict]) -> str | None:
    want = row[ROW["expect"]]
    try:
        result = replay(document, row, caches)
    except Exception as error:  # noqa: BLE001 - the recorded exception is the expectation
        got = [type(error).__name__, str(error)]
        return None if want.get("raises") == got else f"raises {got}, recorded {want}"
    if "raises" in want:
        return f"returns {result.status}, recorded a raise {want['raises']}"
    got = expected(result, full="report" in want)
    return None if got == want else f"outputs differ (now {result.status})"


def check(document: dict, part: int | None = None, parts: int = 1) -> list[str]:
    failures = []
    caches: tuple[dict, dict] = ({}, {})
    for index, row in enumerate(document["imports"]):
        if part is not None and index % parts != part:
            continue
        problem = check_vector(document, row, caches)
        if problem is not None:
            failures.append(f"import vector {index} ({document['sources'][row[ROW['src']]]}): {problem}")
            if len(failures) >= 20:
                break
    if part in (None, 0):
        numeric = numeric_vectors()
        if version_matches(document):
            if document["numeric"] != numeric:
                failures.append("numeric vectors differ")
        else:  # another Python minor version: its JSON error texts for a trailing comma differ (3.13 names it)
            recorded = {key: value for key, value in document["numeric"].items() if key != "json_errors"}
            if recorded != {key: value for key, value in numeric.items() if key != "json_errors"}:
                failures.append("numeric vectors differ")
            kept = [item for item in document["numeric"]["json_errors"] if "trailing comma" not in json.dumps(item)]
            texts = {item[0] for item in kept}
            if kept != [item for item in numeric["json_errors"] if item[0] in texts]:
                failures.append("json error vectors differ")
        for entry in document["orientations"]:
            data = blob_bytes(document, entry["data"], caches[0])
            derived = importer.documented_orientations(data)
            got = None if derived is None else [[a, b, list(vector)] for (a, b), vector in derived.items()]
            if "derived" in entry and got != entry["derived"]:
                failures.append(f"orientations of {entry['data'][:12]} differ")
    return failures


def check_regenerations(document: dict) -> list[str]:
    """Replays the bound imports with the writer's regeneration recorded: each recorded call gives what it did."""
    seen: dict[str, Any] = {}
    original = importer.regenerate_caepipe_sidecar

    def regenerate(model, *, mbf_sha256, derived_y_reference, **reference):
        sidecar, refusals = original(model, mbf_sha256=mbf_sha256, derived_y_reference=derived_y_reference, **reference)
        seen[regeneration_key(model, mbf_sha256, derived_y_reference, reference)] = (
            {"refused": sorted({item["code"] for item in refusals})} if sidecar is None else {"sidecar": lossless(sidecar)})
        return sidecar, refusals

    importer.regenerate_caepipe_sidecar = regenerate
    try:
        caches: tuple[dict, dict] = ({}, {})
        for row in document["imports"]:
            if row[ROW["sc"]] is not None and "raises" not in row[ROW["expect"]]:
                with contextlib.suppress(Exception):
                    replay(document, row, caches)
    finally:
        importer.regenerate_caepipe_sidecar = original
    failures = [f"regeneration {key[:12]} differs" for key, outcome in document["regenerations"].items()
                if key in seen and not same(seen[key], outcome)]
    failures += [f"regeneration {key[:12]} is no longer made" for key in document["regenerations"] if key not in seen]
    return failures[:20]


def main(argv: list[str]) -> int:
    os.chdir(PROJECT)
    if argv[:1] == ["harvest"]:
        document = harvest()
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(render(document), encoding="utf-8")
        print(f"wrote {VECTORS.relative_to(PROJECT)}: {json.dumps(document['counts'])}")
        return 0
    if argv[:1] == ["show"] and len(argv) == 2:
        document = load()
        row = document["imports"][int(argv[1])]
        result = replay(document, row, ({}, {}))
        sys.stdout.write(json.dumps({"source": document["sources"][row[ROW["src"]]], "status": result.status,
                                     "report": result.to_dict()}, indent=2) + "\n")
        model = result.model_bytes()
        sys.stdout.write(model.decode("ascii") if model else "(no model)\n")
        return 0
    if argv[:1] == ["--check"]:
        document = load()
        failures = check(document) + check_regenerations(document)
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1 if failures else 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
