#!/usr/bin/env python3
import argparse
import hashlib
import json
import pathlib
import subprocess


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--model-350", required=True)
    p.add_argument("--checked-json-bin", required=True)
    p.add_argument("--out", required=True)
    args = p.parse_args()
    source_path = pathlib.Path(args.model_350)
    source_bytes = source_path.read_bytes()
    model_350 = json.loads(source_bytes)
    model_500 = json.loads(source_bytes)
    primitive = model_500["load_cases"][0]["primitive_loads"][0]
    before = primitive["magnitude"]["value"]
    if before != 350:
        raise SystemExit(f"expected numeric 350, got {before!r}")
    primitive["magnitude"]["value"] = 500
    request = {
        "protocol_version": "1.0.0",
        "profile": "openpipestress_jcs_ijson_v1",
        "items": [
            {"id": "M350", "json_text": source_bytes.decode("utf-8")},
            {"id": "M500", "json_text": json.dumps(model_500, ensure_ascii=False, separators=(",", ":"))},
        ],
    }
    proc = subprocess.run(
        [args.checked_json_bin],
        input=json.dumps(request, ensure_ascii=False).encode("utf-8"),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        raise SystemExit(proc.stderr.decode("utf-8", errors="replace"))
    response = json.loads(proc.stdout)
    items = {item["id"]: item["canonical_json"].encode("utf-8") for item in response["items"]}
    if items["M350"] != source_bytes:
        raise SystemExit("stored M350 is not exact checked-profile canonical JSON")
    canonical_500 = items["M500"]
    canonical_500_model = json.loads(canonical_500)
    diffs = []
    def walk(a, b, path=""):
        if type(a) is not type(b):
            diffs.append({"path": path or "/", "before": a, "after": b})
        elif isinstance(a, dict):
            for key in sorted(set(a) | set(b)):
                if key not in a or key not in b:
                    diffs.append({"path": f"{path}/{key}", "before": a.get(key), "after": b.get(key)})
                else:
                    walk(a[key], b[key], f"{path}/{key}")
        elif isinstance(a, list):
            if len(a) != len(b):
                diffs.append({"path": path or "/", "before_length": len(a), "after_length": len(b)})
            else:
                for i, (left, right) in enumerate(zip(a, b)):
                    walk(left, right, f"{path}/{i}")
        elif a != b:
            diffs.append({"path": path or "/", "before": a, "after": b})
    walk(model_350, canonical_500_model)
    expected_diff = [{"path": "/load_cases/0/primitive_loads/0/magnitude/value", "before": 350, "after": 500}]
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "M350.checked.json").write_bytes(items["M350"])
    (out / "M500.checked.json").write_bytes(canonical_500)
    report = {
        "status": "PASS" if diffs == expected_diff else "FAIL",
        "checked_json_bin": str(pathlib.Path(args.checked_json_bin).resolve()),
        "checked_json_bin_sha256": sha256(pathlib.Path(args.checked_json_bin).read_bytes()),
        "profile": response["profile"],
        "source_model": str(source_path),
        "M350": {"byte_count": len(items["M350"]), "sha256": sha256(items["M350"])},
        "M500": {"byte_count": len(canonical_500), "sha256": sha256(canonical_500)},
        "exact_diffs": diffs,
        "expected_diffs": expected_diff,
    }
    (out / "DERIVED_H500_V4.json").write_text(json.dumps(report, indent=2) + "\n")
    if report["status"] != "PASS":
        raise SystemExit("unexpected model delta")


if __name__ == "__main__":
    main()
