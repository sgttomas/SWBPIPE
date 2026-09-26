"""Negative controls: each mutation must make the named repaired test fail."""
import importlib.util, json, sys, tempfile, pathlib, traceback
spec = importlib.util.spec_from_file_location("t", "tests/test_headless_runner_contract.py")
t = importlib.util.module_from_spec(spec); spec.loader.exec_module(t)
target = t.cargo_target_debug_directory(t.HEADLESS_CRATE)
bins = (target / "openpipestress-runner", target / "headless_preview_runner")
import subprocess
md = json.loads(subprocess.run(["cargo","metadata","--format-version","1","--no-deps"],cwd=t.PRODUCT_PHYSICS_CRATE,capture_output=True,text=True).stdout)
linked = (md["packages"][0]["name"], md["packages"][0]["version"])
print("linked identity", linked)

def expect_fail(label, fn):
    try:
        fn()
    except AssertionError as e:
        msg = str(e).splitlines()[0][:160] if str(e) else "AssertionError"
        print(f"MUTATION DETECTED  {label}: {msg}")
        return True
    except StopIteration:
        print(f"MUTATION DETECTED  {label}: StopIteration (expected diagnostic absent)")
        return True
    print(f"MUTATION MISSED    {label}")
    return False

def tmp():
    return pathlib.Path(tempfile.mkdtemp())

ok = True
orig_legacy, orig_authored, orig_export_input = t.LEGACY_PRESSURE_PREVIEW_FIXTURE, t.AUTHORED_PREVIEW_MODEL, t.export_input

# A: a solvable model in the legacy control slot must fail the control.
t.LEGACY_PRESSURE_PREVIEW_FIXTURE = orig_authored
ok &= expect_fail("A final legacy control given a solvable model", lambda: t.test_final_runner_refuses_legacy_pressure_demo_as_explicit_control(bins))
ok &= expect_fail("A compat legacy control given a solvable model", lambda: t.test_compatibility_runner_refuses_legacy_pressure_demo_as_explicit_control(bins))
t.LEGACY_PRESSURE_PREVIEW_FIXTURE = orig_legacy

# B: the legacy model in the success slot must fail the success tests (the original CI failure).
t.AUTHORED_PREVIEW_MODEL = orig_legacy
ok &= expect_fail("B every-verb success given legacy model", lambda: t.test_final_runner_subprocess_covers_every_active_verb_and_stdout(bins, linked))
ok &= expect_fail("B compat success given legacy model", lambda: t.test_compatibility_runner_subprocess_controlled_stdout_and_exits(bins))
t.AUTHORED_PREVIEW_MODEL = orig_authored

# C: unbound witness (as recorded, 0.1.0) must fail the export success tests (the original CI failure).
t.export_input = lambda path, identity: json.loads(path.read_text(encoding="utf-8"))
ok &= expect_fail("C deterministic zip-exact with recorded 0.1.0 witness", lambda: t.test_final_runner_subprocess_export_results_success_is_deterministic_and_zip_exact(bins, linked))
ok &= expect_fail("C named-file output with recorded witness", lambda: t.test_final_runner_export_results_output_is_only_named_json_file(bins, linked, tmp()))
ok &= expect_fail("C producer-blocked with recorded witness", lambda: t.test_final_runner_export_results_failures_have_no_payload_or_file(bins, linked, tmp(), t.EXPORT_BLOCKED_INPUT, "HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED"))
ok &= expect_fail("C intent-once with recorded witness", lambda: t.test_final_runner_export_results_requires_intent_once_and_writes_no_file(bins, linked, tmp()))
ok &= expect_fail("C invalid-wire code with recorded witness", lambda: t.test_final_runner_export_results_invalid_wire_payload_preserves_report_code(bins, linked))
ok &= expect_fail("C native size with recorded witness", lambda: t.test_final_runner_export_results_native_size_is_exact_with_constant_cardinality(bins, linked))
t.export_input = orig_export_input

# D: a wrong 'linked' identity must be refused by the runner (identity is enforced, not ignored).
wrong = (linked[0], "9.9.9")
ok &= expect_fail("D success test bound to a non-linked identity", lambda: t.test_final_runner_subprocess_export_results_success_is_deterministic_and_zip_exact(bins, wrong))
ok &= expect_fail("D every-verb producer identity check vs non-linked identity", lambda: t.test_final_runner_subprocess_covers_every_active_verb_and_stdout(bins, wrong))

# E: identity control given the linked version as the 'stale' one must fail.
orig_prior = t.PRIOR_PRODUCT_PHYSICS_VERSION
t.PRIOR_PRODUCT_PHYSICS_VERSION = linked[1]
ok &= expect_fail("E identity control with no actual staleness", lambda: t.test_final_runner_export_results_refuses_unlinked_solver_identity(bins, linked, tmp(), True, False, "request identity does not match"))
t.PRIOR_PRODUCT_PHYSICS_VERSION = orig_prior

print("ALL MUTATIONS DETECTED" if ok else "SOME MUTATIONS MISSED")
sys.exit(0 if ok else 1)
