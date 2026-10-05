"""RV89's own mutants of U4 G5 part 1 (stdlib only). Same method as I65's harness:
one exact replacement in a scratch copy, `cargo test --lib retained_memory`, one job.
KILLED = builds and a test fails; SURVIVED = builds and all pass; COMPILE = build fails."""
import json, os, shutil, subprocess, sys
CAND, COPY, TARGET = sys.argv[1:4]
RM, BI, BS = "src/retained_memory.rs", "src/build_identity.rs", "build.rs"
MUTANTS = [
    ("V01 D1.10 trims only spaces", RM, ".trim_start_matches(|c: char| c.is_ascii_whitespace())", ".trim_start_matches(' ')"),
    ("V02 D1.6 trims the family", RM, 'if !matches!(family.as_str(), "anchor"', 'if !matches!(family.trim(), "anchor"'),
    ("V03 D1.3 expansion laws any->all", RM, "m.material_expansion_laws.iter().any(|law| !matches!(law, Authored::Absent))", "m.material_expansion_laws.iter().all(|law| !matches!(law, Authored::Absent)) && !m.material_expansion_laws.is_empty()"),
    ("V04 raw depth cap 17", RM, "pub(crate) const RAW_DEPTH: usize = 16;", "pub(crate) const RAW_DEPTH: usize = 17;"),
    ("V05 text cap 129", RM, "pub(crate) const TEXT_BYTES: usize = 128;", "pub(crate) const TEXT_BYTES: usize = 129;"),
    ("V06 restraint total reads the maximum", RM, "row(K::RestraintCapacityTotal, n.restraint_capacity, RESTRAINTS),", "row(K::RestraintCapacityTotal, n.max_restraint_capacity, RESTRAINTS),"),
    ("V07 walk skips request materials", RM, "for material in request.materials.iter().chain(&m.materials) {", "for material in m.materials.iter() {"),
    ("V08 walk skips temperature point ids", RM, "            self.string(&p.id)?;\n", ""),
    ("V09 D1.11 keys not counted", RM, "                    add(&mut facts.control_bytes, control_bytes_in(key))?;\n", ""),
    ("V10 encoder passes %", BI, "byte >= 0x21 && byte < 0x7F && byte != b'%' && byte != b';'", "byte >= 0x21 && byte < 0x7F && byte != b';'"),
    ("V11 encoder lowercase hex", BI, "    const HEX: &[u8; 16] = b\"0123456789ABCDEF\";\n    for &byte in value {", "    const HEX: &[u8; 16] = b\"0123456789abcdef\";\n    for &byte in value {"),
    ("V12 lock-hash comparison is presence only", RM, "witnesses && compiled_inputs == Some(registered_inputs)", "witnesses && compiled_inputs.is_some()"),
    ("V13 build.rs hashes the lock for every input", BS, "let path = root.as_ref()?.join(REVIEWED_INPUTS[i]);", "let path = root.as_ref()?.join(REVIEWED_INPUTS[0]);"),
    ("V14 G-B materials ignored", RM, "o(P::Materials, count(f.materials.len())),", "o(P::Materials, 0),"),
    ("V15 G-C result text ignored", RM, "o(P::EnvelopeResultTextBytes, e.results.iter().fold(Bytes::ZERO, |sum, r| sum.plus(result_text(r))).get()),", "o(P::EnvelopeResultTextBytes, 0),"),
    ("V16 bound comparison inverted", RM, "    if required <= threshold {", "    if required >= threshold {"),
    ("V17 overflowed gate sum reads 0", RM, "self.0.and_then(|sum| u64::try_from(sum).ok()).unwrap_or(u64::MAX)", "self.0.and_then(|sum| u64::try_from(sum).ok()).unwrap_or(0)"),
    ("V18 sha256 padding edge at 56", BI, "    if rest.len() >= 56 {", "    if rest.len() > 56 {"),
    ("V19 build.rs accepts a duplicated rustc field", BS, "if found.next().is_some() || value.is_empty() {", "if value.is_empty() {"),
    ("V20 G-C evidence key bytes ignored", RM, "o(P::ContractEvidenceKeyBytes, fact(|c| c.key_capacity_bytes)),", "o(P::ContractEvidenceKeyBytes, 0),"),
    ("V21 D1.9 drops the last D1.9 row", RM, "first_cap_violation(&rows[..CAP_ROWS - 1])?;", "first_cap_violation(&rows[..CAP_ROWS - 2])?;"),
    ("V22 law: domain after the bound", RM, "    domain?;\n    bound(index).map_err(AdmissionRefusal::Bound)?;", "    bound(index).map_err(AdmissionRefusal::Bound)?;\n    domain?;"),
    ("V23 G-C diag id max reads code", RM, "o(P::DiagnosticIdMaxBytes, count(e.diagnostics.iter().map(|d| d.id.len()).max().unwrap_or(0))),", "o(P::DiagnosticIdMaxBytes, count(e.diagnostics.iter().map(|d| d.code.len()).max().unwrap_or(0))),"),
    ("V24 longest string ignores affected refs", RM, "d.affected_refs.iter().map(String::len).fold(n.max(d.source.as_ref().map_or(0, String::len)), usize::max)", "n.max(d.source.as_ref().map_or(0, String::len))"),
]
def run():
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--lib", "--target-dir", TARGET, "retained_memory"],
                       cwd=COPY, env=env, capture_output=True, text=True, timeout=2400)
    out = p.stdout + p.stderr
    if "could not compile" in out or "error[E" in out:
        return "COMPILE", out[-600:]
    return ("KILLED" if p.returncode != 0 else "SURVIVED"), [l for l in out.splitlines() if l.startswith("test ") and "FAILED" in l][:4]
only = set(sys.argv[4].split(",")) if len(sys.argv) > 4 else None
for name, rel, old, new in MUTANTS:
    if only and name.split()[0] not in only:
        continue
    if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
        print(json.dumps({"mutant": name, "result": "STOPPED: memory guard not running"})); sys.exit(9)
    for f in (RM, BI, BS):
        shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))
    path = os.path.join(COPY, rel)
    text = open(path).read()
    if text.count(old) != 1:
        print(json.dumps({"mutant": name, "result": "NOT APPLIED", "count": text.count(old)}), flush=True)
        continue
    open(path, "w").write(text.replace(old, new))
    result, detail = run()
    print(json.dumps({"mutant": name, "file": rel, "result": result, "detail": detail}), flush=True)
for f in (RM, BI, BS):
    shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))
