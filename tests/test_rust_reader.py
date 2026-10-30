"""Product Python takes the Rust reader's verdict (`core/analysis_runs/rust_reader.py`).

`compatibility.py`'s retained calls go through `openpipestress-runner read`; they never
re-derive a verdict. Over every shared-corpus entry the product path returns the corpus's
Rust expectation: `expected_by_reader.rust` (else `expected`) bound,
`expected_unbound_by_reader.rust` (else `expected_unbound`) unbound and `expected_transport`
on transport, with each passing statement's eligibility and classes. Where the corpus
declares Rust's G7 code apart from Python's, the product takes Rust's.
"""
from copy import deepcopy
import json

import pytest

from core.analysis_runs import compatibility as c
from core.analysis_runs import rust_reader
from tests.test_retained_precision_contract import apply_entry, corpus


def test_the_runner_is_an_explicit_absolute_executable(monkeypatch, tmp_path):
    monkeypatch.delenv(rust_reader.ENV_EXECUTABLE, raising=False)
    with pytest.raises(RuntimeError, match="RUST-READER-MISSING"):
        rust_reader.runner_executable()
    monkeypatch.setenv(rust_reader.ENV_EXECUTABLE, "relative/openpipestress-runner")
    with pytest.raises(RuntimeError, match="RUST-READER-MISSING"):
        rust_reader.runner_executable()
    monkeypatch.setenv(rust_reader.ENV_EXECUTABLE, str(tmp_path / "absent"))
    with pytest.raises(RuntimeError, match="RUST-READER-MISSING"):
        rust_reader.runner_executable()


def test_a_response_outside_the_verdict_contract_is_never_taken(monkeypatch):
    class Completed:
        def __init__(self, code, stdout):
            self.returncode, self.stdout, self.stderr = code, json.dumps(stdout).encode(), b""
    head = {"artifact": rust_reader.ARTIFACT, "schema_version": rust_reader.SCHEMA_VERSION, "reader": rust_reader.RETAINED_PRECISION}
    validation = {"invocation_bound": False, "numerical_eligible": False, "publication_sha256": "0" * 64, "classifications": []}
    refusal = {"gate": "G1", "code": "RETAINED_PRECISION_RECEIPT_MISMATCH", "detail": None}
    passed, refused = {"ok": True, "validation": validation}, {"ok": False, "error": refusal}
    for code, stdout in [(0, dict(head, outcome=refused)), (1, dict(head, outcome=passed)),
                         (0, dict(head, outcome=passed, extra=1)), (0, dict(head, outcome=dict(passed, error=refusal))),
                         (0, dict(head, reader="other", outcome=passed)), (2, dict(head, outcome=passed)), (0, []),
                         (1, dict(head, outcome={"ok": False, "error": {"gate": "G1"}})),
                         (1, dict(head, outcome={"ok": False, "error": dict(refusal, code=None)})),
                         (1, dict(head, outcome={"ok": False, "error": dict(refusal, detail=1)}))]:
        monkeypatch.setattr(rust_reader.subprocess, "run", lambda *args, **kwargs: Completed(code, stdout))
        with pytest.raises(RuntimeError):
            rust_reader.retained_precision({"producer": {}})
    monkeypatch.setattr(rust_reader.subprocess, "run", lambda *args, **kwargs: Completed(1, dict(head, outcome=refused)))
    with pytest.raises(rust_reader.ReaderRefusal) as error:
        rust_reader.retained_precision({"producer": {}})
    assert (error.value.gate, error.value.code, error.value.detail, str(error.value)) == ("G1", refusal["code"], None, refusal["code"])
    with pytest.raises(ValueError):
        rust_reader.retained_precision({"value": float("nan")})


def _rust(entry, key):
    return entry.get(key + "_by_reader", {}).get("rust", entry.get(key))


def _product(read, *args):
    """The product path's verdict: the validation, or the reader's gate and code."""
    try:
        return "pass", read(*args)
    except rust_reader.ReaderRefusal as refusal:
        return "refuse", {"gate": refusal.gate, "code": refusal.code}


def _entries():
    """(entry, its base) for every base (its own base), mutation and must-pass entry."""
    data = corpus()
    bases = {fixture["id"]: fixture for fixture in data["cases"]}
    for fixture in data["cases"]:
        yield pytest.param(fixture, fixture, id=fixture["id"])
    for kind in ("mutations", "must_pass"):
        for entry in data[kind]:
            yield pytest.param(entry, bases[entry["base"]], id=entry["id"])


@pytest.mark.parametrize("entry,base", list(_entries()))
def test_the_product_path_takes_the_rust_verdict_on_every_corpus_entry(entry, base):
    label = entry["id"]
    if entry is base:
        source, invocation = deepcopy(base["source"]), deepcopy(base["invocation"])
    else:
        source, invocation = apply_entry(base, entry)
    bound = _product(rust_reader.retained_precision, source, invocation)
    expected = _rust(entry, "expected")
    if isinstance(expected, dict) and "gate" in expected:
        assert bound == ("refuse", expected), label
    else:
        assert bound[0] == "pass", (label, bound)
        validation = bound[1]
        assert validation["invocation_bound"] is True
        eligibility = entry.get("expected_eligibility", expected)
        assert validation["numerical_eligible"] is eligibility["numerical_eligible"], label
        assert validation["classifications"] == entry.get("expected_classifications", base["expected_classifications"]), label
        assert validation["publication_sha256"] == source["retained_precision"]["body"]["publication_sha256"]
    # compatibility.py's call is that verdict: a G7 refusal keeps the reader's detail, any other its code.
    try:
        assert c._retained_validation(source, invocation) == bound[1]
    except ValueError as error:
        assert bound[0] == "refuse" and str(error).split(":")[0] == bound[1]["code"], label
    unbound, transport = _rust(entry, "expected_unbound"), entry.get("expected_transport")
    for stated, read, args in ((unbound, rust_reader.retained_precision, (source,)), (transport, rust_reader.retained_precision_transport, (source,))):
        if stated is None:
            continue
        got = _product(read, *args)
        if stated == "pass":
            assert got[0] == "pass" and got[1]["numerical_eligible"] is False, (label, got)
        else:
            assert got == ("refuse", stated), label
