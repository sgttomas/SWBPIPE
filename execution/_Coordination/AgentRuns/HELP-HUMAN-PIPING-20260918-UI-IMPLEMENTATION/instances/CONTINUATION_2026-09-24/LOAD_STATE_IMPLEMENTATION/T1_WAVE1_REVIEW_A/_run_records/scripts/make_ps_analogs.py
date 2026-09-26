"""Physics-source-1 analogs of probes B01, B04 and C01, to separate inherited from new behaviour."""
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from make_probes import reseal  # noqa: E402  (re-runs make_probes into its out dir; harmless)
root = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[3]); out.mkdir(exist_ok=True)
src = lambda: json.loads((root / "fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json").read_text())
d = src(); d["diagnostics"].append({"id": "diagnostic:source-recovery:case", "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "severity": "info", "message": "invented", "affected_refs": [d["numerical_quality"]["cases"][0]["basis_ref"]["ref_id"]]})
(out / "PS-B04-selected-with-unavailable-resealed.json").write_text(json.dumps(reseal(d, physical="physics")))
d = src(); d["zz_review_extra"] = 2 ** 60
(out / "PS-C01-top-level-integer-above-2p53.json").write_text(json.dumps(d))
