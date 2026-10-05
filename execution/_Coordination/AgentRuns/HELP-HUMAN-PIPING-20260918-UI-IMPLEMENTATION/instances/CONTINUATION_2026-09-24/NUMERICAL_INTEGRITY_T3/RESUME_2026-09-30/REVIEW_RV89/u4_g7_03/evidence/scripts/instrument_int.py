"""RV89 G9a addition (copy only): on 92a5a9da1c's source_blocks.rs, counters on `integer` (calls) and
`validate_in` (calls); optionally `integer` set back to 6b9bb19a5f's order."""
import sys
core, mode = sys.argv[1], sys.argv[2]
p = f"{core}/reporting/result_export/src/source_blocks.rs"; s = open(p).read()
inc = lambda n: f"{n}.fetch_add(1, std::sync::atomic::Ordering::SeqCst);"
new = ("fn integer(v: &Value) -> Result<usize, String> {\n    v.as_u64()\n        .filter(|n| *n <= 9_007_199_254_740_991)\n"
       "        .and_then(|n| usize::try_from(n).ok())\n")
old = ("fn integer(v: &Value) -> Result<usize, String> {\n    v.as_u64()\n        .and_then(|n| usize::try_from(n).ok())\n"
       "        .filter(|n| *n <= 9_007_199_254_740_991)\n")
assert s.count(new) == 1, "92a5a9da1c's integer"
body = old if mode == "old" else new
s = s.replace(new, body.replace("    v.as_u64()\n", "    " + inc("RV89_INTEGER_CALLS") + "\n    v.as_u64()\n", 1))
a = ") -> Result<bool, String> {\n    require!(\n        keys("
i = s.index("pub(crate) fn validate_in("); j = s.index(a, i); assert j - i < 200
s = s[:j] + ") -> Result<bool, String> {\n    " + inc("RV89_VALIDATE_IN_CALLS") + "\n    require!(\n        keys(" + s[j + len(a):]
s += "\npub static RV89_INTEGER_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\npub static RV89_VALIDATE_IN_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\npub static RV89_LOOP_ROWS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n"
open(p, "w").write(s); print("instrumented integer", mode)
