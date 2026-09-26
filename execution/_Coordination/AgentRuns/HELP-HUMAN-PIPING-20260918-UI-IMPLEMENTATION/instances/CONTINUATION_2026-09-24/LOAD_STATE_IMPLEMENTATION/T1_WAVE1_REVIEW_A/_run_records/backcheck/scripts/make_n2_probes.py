"""Backcheck N-2 edge probes. Usage: make_n2_probes.py <WORKING_ROOT> <OUT_DIR>"""
import json, pathlib, sys
W, out = sys.argv[1], pathlib.Path(sys.argv[2])
sys.argv = [sys.argv[0], W, str(out.parent / "probes_n2_tmp")]
from make_probes import reseal  # noqa: E402
def base():
    return json.load(open(f"{W}/fixtures/product_preview/load_reference_source/mixed-sparse_interactive.raw.json"))
def add(d, diag):
    d["diagnostics"].append(dict({"id": "diagnostic:invented", "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "severity": "info", "message": "invented"}, **diag))
    return reseal(d, physical=None)
cases = {
    "D01-unavailable-refs-string-resealed": {"affected_refs": "case"},
    "D02-unavailable-warning-mixed-refs-resealed": {"severity": "warning", "affected_refs": [1, "case"]},
    "D03-unavailable-on-ordinary-case-resealed": {"affected_refs": ["case:ordinary-pressure"]},
    "D05-unavailable-no-refs-resealed": {"affected_refs": []},
}
for k, v in cases.items():
    (out / f"{k}.json").write_text(json.dumps(add(base(), v)))
(out / "D04-lr-fallback-sparse-control.json").write_text(open(f"{W}/core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json").read())
