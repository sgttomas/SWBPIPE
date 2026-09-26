"""Regenerate every committed producer raw fixture from its request with the
current producer and compare. Run from WORKING_ROOT after building the
product_physics examples; pass the examples directory as argv[1].
Byte comparison for pretty-printed raws; source_blocks raws are stored
key-sorted, so they are compared as parsed JSON."""
import json, pathlib, subprocess, sys

examples = pathlib.Path(sys.argv[1])
root = pathlib.Path("fixtures/product_preview")
modes = {"sparse_interactive": "sparse", "dense_scrutiny": "dense"}
results = []
for d in ["load_reference", "load_reference_source", "physics_source"]:
    for request in sorted((root / d).glob("*.request.json")):
        name = request.name[: -len(".request.json")]
        for suffix, mode in modes.items():
            raw = root / d / f"{name}-{suffix}.raw.json"
            if not raw.exists():
                continue
            outcome = []
            for example in ["physics_source_connected", "exact_pressure_connected"]:
                out = subprocess.run([examples / example, mode, request], capture_output=True).stdout
                if out == raw.read_bytes():
                    outcome.append(f"{example}:BYTE_MATCH")
            results.append((f"{d}/{raw.name}", outcome or ["NO_MATCH"]))
for request in sorted((root / "source_blocks").glob("*.request.json")):
    name = request.name[: -len(".request.json")]
    mode = "sparse" if "sparse" in name else "dense"
    raw = root / "source_blocks" / f"{name}.raw.json"
    out = subprocess.run([examples / "physics_source_connected", mode, request], capture_output=True).stdout
    results.append((f"source_blocks/{raw.name}", ["physics_source_connected:JSON_EQUAL" if json.loads(out) == json.loads(raw.read_text()) else "NO_MATCH"]))
for path, outcome in results:
    print(path, " ".join(outcome))
print("TOTAL", len(results), "NO_MATCH", sum("NO_MATCH" in o for _, o in results))
