import hashlib, pathlib, subprocess, sys
L = "src/lib.rs"
KEEP = "            charged: self.charged,\n            failed_charged: self.failed_charged,\n            publication_charged: self.publication_charged,\n            rejected: self.rejected,\n            attempts: self.attempts,\n"
M = [
 ("B1 ledger reset (=N2-M8), exhaustion test only", KEEP, KEEP.replace("self.charged","0").replace("self.failed_charged","0").replace("self.publication_charged","0").replace("self.rejected","0").replace("self.attempts","0"),
  "source_receipt::load_state_fallback_tests::a_fallback_after_invocation_limit_exhaustion"),
 ("B2 charges reset, attempts kept, whole fallback module", KEEP, KEEP.replace("self.charged","0").replace("self.failed_charged","0"),
  "source_receipt::load_state_fallback_tests"),
 ("B3 withheld cause dropped from republication diagnostics", 'let withheld = match (&load_state, &source_budget.load_state_join_withheld) {', 'let withheld = match (&load_state, &None::<String>) {',
  "source_receipt::load_state_fallback_tests"),
]
p = pathlib.Path(L); orig = p.read_bytes(); h = hashlib.sha256(orig).hexdigest()
for name, old, new, filt in M:
    t = orig.decode(); assert t.count(old) == 1, name
    p.write_text(t.replace(old, new))
    try:
        r = subprocess.run(["cargo","+1.97.1","test","--locked","--offline","-j","2","--lib",filt], capture_output=True, text=True)
        failed = [l.split()[1].split("::")[-1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        ce = "could not compile" in r.stderr
        print(name, "KILLED" if r.returncode and not ce else ("COMPILE_ERROR" if ce else "SURVIVED"), failed, flush=True)
    finally:
        p.write_bytes(orig); assert hashlib.sha256(p.read_bytes()).hexdigest() == h
print("restored and sha256-verified")
