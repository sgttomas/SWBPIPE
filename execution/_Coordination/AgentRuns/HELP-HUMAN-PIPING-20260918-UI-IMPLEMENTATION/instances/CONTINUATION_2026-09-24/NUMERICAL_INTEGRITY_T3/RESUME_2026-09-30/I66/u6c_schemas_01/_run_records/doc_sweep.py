"""I66 U6c control (scratch only): every committed carrier document under
P/fixtures and P/tests, validated by the lane's own schemas through the
repository's validate_instance. Run once per lane; outputs compared."""
import json, sys
from pathlib import Path
P = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(P))
from tests.schema_validation import validate_instance
DISPATCH = {
    "results": "results.schema.yaml",
    "analysis_run": "analysis_run.schema.json",
    "stress_neutral": "stress_neutral_export.schema.json",
    "persistence": "project_persistence.schema.yaml",
    "handoff": "handoff_package.schema.json",
}
schemas = {k: json.loads((P / "schemas" / v).read_text()) for k, v in DISPATCH.items()}


def kind(v):
    if not isinstance(v, dict) or not isinstance(v.get("schema_version"), str):
        return None
    if "result_envelope" in v:
        return "results"
    if "analysis_run" in v and "run_contract_status" in v:
        return "analysis_run"
    if "csv_text" in v and "manifest" in v:
        return "stress_neutral"
    return None


def walk(v, ptr, out):
    k = kind(v)
    if k:
        out.append((ptr, k, v))
    if isinstance(v, dict):
        for key, child in v.items():
            walk(child, f"{ptr}/{key}", out)
    elif isinstance(v, list):
        for i, child in enumerate(v):
            walk(child, f"{ptr}/{i}", out)


rows = []
for root in ("fixtures", "tests"):
    for path in sorted((P / root).rglob("*.json")):
        try:
            doc = json.loads(path.read_text())
        except Exception:
            continue
        found = []
        walk(doc, "", found)
        for ptr, k, v in found:
            try:
                validate_instance(schemas[k], v, instance_label="sweep")
                verdict = "valid"
            except AssertionError as e:
                verdict = "invalid:" + " | ".join(str(e).splitlines()[1:3])[:300]
            except Exception as e:
                verdict = f"error:{type(e).__name__}"
            rows.append(f"{path.relative_to(P)}#{ptr}\t{k}\t{verdict}")
OUT.write_text("\n".join(rows) + "\n")
print(len(rows))
