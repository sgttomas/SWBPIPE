"""RV89 G7 (copy only): counters in result_export and PP for the reachability probe."""
import sys
core = sys.argv[1]
def edit(path, pairs, append=None):
    p = f"{core}/{path}"; s = open(p).read()
    for a, b in pairs:
        assert s.count(a) == 1, (path, a[:60], s.count(a)); s = s.replace(a, b)
    if append: s += append
    open(p, "w").write(s)
A = "std::sync::atomic::AtomicUsize"
inc = lambda n: f"{n}.fetch_add(1, std::sync::atomic::Ordering::SeqCst);"
edit("reporting/result_export/src/semantic_contract.rs", [
    ("fn is_retained(source: &Value) -> bool {\n    source[\"producer\"][\"semantic_contract_id\"] == PREVIEW_PHYSICS_RETAINED_ID\n}",
     "fn is_retained(source: &Value) -> bool {\n    let r = source[\"producer\"][\"semantic_contract_id\"] == PREVIEW_PHYSICS_RETAINED_ID;\n    " + inc("RV89_IS_RETAINED_CALLS") + "\n    if r { " + inc("RV89_IS_RETAINED_TRUE") + " }\n    r\n}"),
    ("    CONTRACT.get_or_init(|| {\n        verify_preview_physics_retained_table(",
     "    CONTRACT.get_or_init(|| {\n        " + inc("RV89_RETAINED_STATIC_INIT") + "\n        verify_preview_physics_retained_table("),
    ("    if !is_retained(source) && source.get(\"retained_precision\").is_some() {\n        return Err(",
     "    if !is_retained(source) && source.get(\"retained_precision\").is_some() {\n        " + inc("RV89_FORBID_ERR") + "\n        return Err("),
    ("    {\n        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());\n    }\n    Ok(())\n}\n/// Validated successor row classes",
     "    {\n        " + inc("RV89_FORBID_ERR") + "\n        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());\n    }\n    Ok(())\n}\n/// Validated successor row classes"),
], append="".join(f"\npub static {n}: {A} = {A}::new(0);" for n in ("RV89_IS_RETAINED_CALLS", "RV89_IS_RETAINED_TRUE", "RV89_RETAINED_STATIC_INIT", "RV89_FORBID_ERR")) + "\n")
edit("reporting/result_export/src/retained_precision.rs", [
    ("pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> VResult<Validation> {\n",
     "pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> VResult<Validation> {\n    " + inc("RV89_VALIDATE_CALLS") + "\n"),
    ("pub fn validate_transport_metadata(source: &Value) -> VResult<Validation> {\n",
     "pub fn validate_transport_metadata(source: &Value) -> VResult<Validation> {\n    " + inc("RV89_TRANSPORT_CALLS") + "\n"),
], append="".join(f"\npub static {n}: {A} = {A}::new(0);" for n in ("RV89_VALIDATE_CALLS", "RV89_TRANSPORT_CALLS")) + "\n")
# PP: the G6 solve-attempt oracle (test-only) and both probe modules
p = f"{core}/product_physics/src/lib.rs"; s = open(p).read()
anchor = "        BasisStiffness::RangeDeferred(error) => Err(OrdinaryFailure::Formation(error.clone())),\n    };\n"
assert s.count(anchor) == 1
s = s.replace(anchor, anchor + "    #[cfg(test)]\n    RV89_ATTEMPTS.with(|c| c.set(c.get() + 1));\n")
s += "\n#[cfg(test)]\nthread_local! { pub(crate) static RV89_ATTEMPTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }\n"
open(p, "w").write(s)
p = f"{core}/product_physics/src/retained_memory.rs"
open(p, "a").write('\n#[cfg(test)]\n#[path = "rv89_g6_probe_tests.rs"]\nmod rv89g6;\n#[cfg(test)]\n#[path = "rv89_g7_probe_tests.rs"]\nmod rv89g7;\n')
print("instrumented")
