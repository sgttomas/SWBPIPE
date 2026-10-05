"""RV89 G9a (copy only): counters in source_blocks::validate_in (calls; loop rows), and optionally the
loop header set back to PR1080's parent (`for (&id, treatment) in &treatments {`)."""
import sys
core, mode = sys.argv[1], sys.argv[2]
p = f"{core}/reporting/result_export/src/source_blocks.rs"; s = open(p).read()
inc = lambda n: f"{n}.fetch_add(1, std::sync::atomic::Ordering::SeqCst);"
a = ") -> Result<bool, String> {\n    require!(\n        keys("
i = s.index("pub(crate) fn validate_in(")
j = s.index(a, i)
assert j - i < 200
s = s[:j] + ") -> Result<bool, String> {\n    " + inc("RV89_VALIDATE_IN_CALLS") + "\n    require!(\n        keys(" + s[j + len(a):]
row = "            let row = raw[id];\n"
assert s.count(row) == 1
s = s.replace(row, row + "            " + inc("RV89_LOOP_ROWS") + "\n")
new = ("        // Receipt row order, never hash order: the first refusal must be stable\n"
       "        // across runs and match the Python reader's insertion-ordered walk.\n"
       "        for treatment in rows {\n            let id = text(&treatment[\"result_id\"])?;\n")
assert s.count(new) == 1
if mode == "old":
    s = s.replace(new, "        for (&id, treatment) in &treatments {\n")
s += "\npub static RV89_VALIDATE_IN_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\npub static RV89_LOOP_ROWS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n"
open(p, "w").write(s)
print("instrumented", mode)
