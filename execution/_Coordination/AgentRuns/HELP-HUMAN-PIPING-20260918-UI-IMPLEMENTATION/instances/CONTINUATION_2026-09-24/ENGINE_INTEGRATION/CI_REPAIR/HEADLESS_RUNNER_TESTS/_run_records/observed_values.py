import importlib.util, json, hashlib, subprocess
spec = importlib.util.spec_from_file_location("t", "tests/test_headless_runner_contract.py")
t = importlib.util.module_from_spec(spec); spec.loader.exec_module(t)
target = t.cargo_target_debug_directory(t.HEADLESS_CRATE); final = target/"openpipestress-runner"
md = json.loads(subprocess.run(["cargo","metadata","--format-version","1","--no-deps"],cwd=t.PRODUCT_PHYSICS_CRATE,capture_output=True,text=True).stdout)
linked = (md["packages"][0]["name"], md["packages"][0]["version"])
out = {"linked_identity": linked}
body = t.export_input(t.EXPORT_SUCCESS_INPUT, linked)
a = t.run_final(final, "export-results", body, "--explicit-local-private-intent")
b = t.run_final(final, "export-results", body, "--explicit-local-private-intent")
pkg = json.loads(a.stdout)["payload"]["report_package"]
out["success"] = {"exit": [a.returncode, b.returncode], "stdout_identical": a.stdout == b.stdout,
  "stdout_sha256": hashlib.sha256(a.stdout.encode()).hexdigest(),
  "container_len": len(pkg["container_bytes"]), "container_sha256_hex": pkg["container_sha256_hex"],
  "members": [(m["file_name"], m["byte_length"]) for m in pkg["members"]]}
body["export_results"]["state_comparison_handoff_records"][0]["invented_padding"] = "x" * 3_200_000
c = t.run_final(final, "export-results", body, "--explicit-local-private-intent")
p2 = json.loads(c.stdout)["payload"]["report_package"]
out["native_size"] = {"exit": c.returncode, "container_len": len(p2["container_bytes"]), "lower_bound_asserted": 3_189_621,
  "container_sha256_hex": p2["container_sha256_hex"]}
m = t.authored_preview_model()
s = t.run_final(final, "solve", {"request": t.final_runner_request("solve"), "solve": {"preview_model": {"model": m, "materials": []}}}, "--explicit-local-private-intent")
me = json.loads(s.stdout)["payload"]["mechanics_envelope"]
out["authored_solve"] = {"exit": s.returncode, "mechanics": me["status"]["mechanics"], "results": len(me["results"]), "producer": me["producer"]}
print(json.dumps(out, indent=1))
