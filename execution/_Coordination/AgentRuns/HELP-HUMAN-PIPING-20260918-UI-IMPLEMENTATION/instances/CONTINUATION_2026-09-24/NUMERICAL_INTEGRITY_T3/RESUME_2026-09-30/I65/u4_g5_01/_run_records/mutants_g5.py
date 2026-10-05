"""I65 U4 G5, control 3: mutants for every D1 clause, every gate, the bound comparison,
the build-identity check and the stack refusal (stdlib only).

Each mutant is one exact text replacement in a scratch copy of the candidate. The copy is
built and the retained_memory tests run (`cargo test --lib retained_memory`, the default
toolchain, --locked --offline, one cargo job). A mutant is:
  KILLED      the build succeeds and a test fails;
  SURVIVED    the build succeeds and every test passes;
  COMPILE     the build fails (a control failure: no mutant may be killed only this way).
Equivalent mutants (no observable difference while `REGISTERED_PROFILES` is empty, by
decision 7, or by construction) are listed with their reason and expected to survive.
Usage: python3 mutants_g5.py <candidate product_physics dir> <scratch copy product_physics dir>
       <target dir> > mutants.out.jsonl
"""
import json, os, shutil, subprocess, sys

CAND, COPY, TARGET = sys.argv[1:4]
RM = "src/retained_memory.rs"
BI = "src/build_identity.rs"
BS = "build.rs"
EQUIVALENT = {
    "D1.8 deletion": "D1.8 restates D1.4 (no components): D1.4 refuses first, so D1.8's own return is unreachable",
    "build_status ignores bindings": "decision 7: no profile is registered, so build_status returns Missing before the bindings",
    "admission ignores the refusal": "decision 7: REGISTERED_PROFILES is empty, so no index selects a profile",
}
def family(clause, fact):
    return (f"{clause} {fact} deletion", RM, f"return refuse(C::{clause}, F::{fact});", "let _ = 0;")
MUTANTS = [
    ("D1.0 admits Headless", RM, "        caller => Err(AdmissionRefusal::Caller(caller)),\n", "        _caller => Ok(()),\n"),
    ("D1.1 ignores the build", RM, "    let index = build.map_err(AdmissionRefusal::Profile)?;", "    let index = build.unwrap_or(0);"),
    ("D1.2 raw ignored", RM, "    incomplete(CensusPart::Raw, f.raw.status)?;", "    let _ = incomplete(CensusPart::Raw, f.raw.status);"),
    ("D1.2 raw text ignored", RM, "    incomplete(CensusPart::RawText, f.raw_text.status)?;", "    let _ = incomplete(CensusPart::RawText, f.raw_text.status);"),
    ("D1.2 dof ignored", RM, "    if let Err(status) = f.typed.dof_upper {", "    if let (false, Err(status)) = (true, f.typed.dof_upper) {"),
    ("D1.2 nested ignored", RM, "    incomplete(CensusPart::TypedNested, f.nested.status)?;", "    let _ = incomplete(CensusPart::TypedNested, f.nested.status);"),
    ("D1.2 headless ignored", RM, "        incomplete(CensusPart::Headless, h.invocation.status)?;", "        let _ = incomplete(CensusPart::Headless, h.invocation.status);"),
    family("Namespace", "SchemaVersion"), family("Namespace", "PressureContract"), family("Namespace", "ReferenceConfigurations"),
    family("Namespace", "MaterialExpansionLaw"), family("Namespace", "RequestExpansionLaws"), family("Namespace", "Sections"),
    family("Namespace", "SectionRef"), family("Invocation", "LoadCases"), family("Invocation", "Combinations"),
    family("Invocation", "Components"), family("Case", "PressureRegions"), family("Case", "EquivalentStatic"),
    family("Case", "ModulusBasisRef"), family("Case", "ModulusBasisTemperature"), family("Case", "AnalysisState"),
    family("Supports", "Hanger"), family("Supports", "Nonlinear"), family("Supports", "SupportFamily"),
    family("Loads", "LoadTarget"), family("Loads", "LoadDimension"),
    ("D1.8 deletion", RM, "return refuse(C::Members, F::Components);", "let _ = 0;"),
    ("D1.3 drops 0.2.0", RM, 'matches!(m.schema_version.as_str(), "0.1.0" | "0.2.0")', 'matches!(m.schema_version.as_str(), "0.1.0")'),
    ("D1.6 drops spring", RM, '"anchor" | "guide" | "line_stop" | "vertical_support" | "spring"', '"anchor" | "guide" | "line_stop" | "vertical_support"'),
    ("D1.7 drops moment", RM, 'matches!(load.dimension.as_str(), "force" | "moment")', 'matches!(load.dimension.as_str(), "force")'),
    ("D1.9 off by one", RM, "find(|r| r.observed > r.cap)", "find(|r| r.observed > r.cap + 1)"),
    ("D1.9 refuses at the cap", RM, "find(|r| r.observed > r.cap)", "find(|r| r.observed >= r.cap)"),
    ("D1.9 l cap 129", RM, "pub(crate) const LOADS: usize = 128;", "pub(crate) const LOADS: usize = 129;"),
    ("D1.9 nodes row reads members", RM, "row(K::Nodes, t.nodes.length, NODES),", "row(K::Nodes, t.members.length, NODES),"),
    ("D1.9 raw depth row shifted", RM, "row(K::RawDepth, raw.maximum_depth, RAW_DEPTH),", "row(K::RawDepth, raw.maximum_depth.saturating_sub(1), RAW_DEPTH),"),
    ("D1.9 restraints not summed", RM, "add(&mut f.restraints, support.restraints.len())?;", "add(&mut f.restraints, 0)?;"),
    ("D1.9 springs not counted", RM, "add(&mut self.facts.springs, 1)?;", "add(&mut self.facts.springs, 0)?;"),
    ("D1.9 units not read", RM, "self.facts.units = borrowed_value_census(units);", "self.facts.units = borrowed_value_census(&Value::Null);"),
    ("D1.10 skipped", RM, "    provenance_clause(request)?;\n", "    let _ = provenance_clause(request);\n"),
    ("D1.10 wrong byte", RM, ".starts_with('{')", ".starts_with('[')"),
    ("D1.10 no whitespace trim", RM, ".trim_start_matches(|c: char| c.is_ascii_whitespace())", ""),
    ("D1.11 skipped", RM, "    first_cap_violation(&rows[CAP_ROWS - 1..])\n", "    Ok(())\n"),
    ("D1.11 misses 0x1F", RM, "*b < 0x20 || *b == 0x7F", "*b < 0x1F || *b == 0x7F"),
    ("D1.11 misses DEL", RM, "*b < 0x20 || *b == 0x7F", "*b < 0x20"),
    ("gate refuses at the cap", RM, "find(|(o, cap)| o.observed > **cap)", "find(|(o, cap)| o.observed >= **cap)"),
    ("gate off by one", RM, "find(|(o, cap)| o.observed > **cap)", "find(|(o, cap)| o.observed > **cap + 1)"),
    ("G-B nodes read supports", RM, "o(P::BuiltNodes, count(f.built.nodes.len())),", "o(P::BuiltNodes, count(f.built.supports.len())),"),
    ("G-B restrained read springs", RM, "o(P::Restrained, count(f.restrained.len())),", "o(P::Restrained, count(f.springs.len())),"),
    ("G-B observation zero", RM, "o(P::LateObservationBytes, capture_bytes(f.capture)),", "o(P::LateObservationBytes, 0),"),
    ("G-C diagnostics read results", RM, "o(P::EnvelopeDiagnostics, count(e.diagnostics.len())),", "o(P::EnvelopeDiagnostics, count(e.results.len())),"),
    ("G-C row text drops id", RM, "strings([&row.id, &row.kind, &row.unit, &row.entity_ref])", "strings([&row.kind, &row.unit, &row.entity_ref])"),
    ("G-C longest string unread", RM, "        o(P::EnvelopeMaxStringBytes, count(longest_string(e).max(", "        o(P::EnvelopeMaxStringBytes, 0 * count(longest_string(e).max("),
    ("G-C diagnostic id unread", RM, "        o(P::DiagnosticIdMaxBytes, count(e.diagnostics.iter().map(|d| d.id.len()).max().unwrap_or(0))),", "        o(P::DiagnosticIdMaxBytes, 0),"),
    ("G-C recovery unread", RM, "o(P::SourceBlockRecovery, u64::from(e.source_block_recovery.is_some())),", "o(P::SourceBlockRecovery, 0),"),
    ("G-C evidence status unread", RM, "o(P::ContractEvidenceStatus, u64::from(!evidence_complete)),", "o(P::ContractEvidenceStatus, 0),"),
    ("G-C error text drops observable", RM, ".add(text(&capture.error)).add(text(&capture.observable_error))", ".add(text(&capture.error))"),
    ("gate k unbounded", RM, "let k = if 6 * n < RESTRAINTS as u64 { 6 * n } else { RESTRAINTS as u64 };", "let k = 6 * n + 1;"),
    ("gate diag text undoubled", RM, "            2 * text_atoms::DIAG_ENV,", "            text_atoms::DIAG_ENV,"),
    ("bound refuses at M", RM, "    if required <= threshold {", "    if required < threshold {"),
    ("bound saturates", RM, "let required = maximum.checked_add(reserved_stack).ok_or(BoundRefusal::Overflow)?;", "let required = maximum.saturating_add(reserved_stack);"),
    ("bound skipped", RM, "    bound(index).map_err(AdmissionRefusal::Bound)?;", "    let _ = bound(index);"),
    ("bound priced at zero", RM, "    Err(BoundRefusal::Unpriced)\n}", "    Ok(0)\n}"),
    ("identity: nothing registered is not Missing", RM, "    if registered.len() == 0 {", "    if registered.len() == 99 {"),
    ("identity: unavailable may match", RM, "        Some(text) if text != build_identity::IDENTITY_UNAVAILABLE => text,", "        Some(text) => text,"),
    ("identity: prefix match", RM, "registered.position(|identity| identity == compiled)", "registered.position(|identity| compiled.starts_with(identity))"),
    ("bindings ignore witnesses", RM, "    witnesses && compiled_inputs == Some(registered_inputs)", "    compiled_inputs == Some(registered_inputs)"),
    ("bindings ignore layouts", RM, "&& build_layouts == registered_layouts", ""),
    ("bindings ignore inputs", RM, "    witnesses && compiled_inputs == Some(registered_inputs)", "    witnesses"),
    ("build_status ignores bindings", RM, "    if bindings_hold(LAYOUT_WITNESSES, COMPILED_REVIEWED_INPUTS, p.reviewed_inputs, &READER_LAYOUTS, &p.reader_layouts) {", "    if true {"),
    ("layout witness wrong Number", RM, "size_of::<serde_json::Number>() == 16", "size_of::<serde_json::Number>() == 24"),
    ("encoder passes space", BI, "    byte >= 0x21 && byte < 0x7F && byte != b'%'", "    byte >= 0x20 && byte < 0x7F && byte != b'%'"),
    ("encoder passes =", BI, " && byte != b';' && byte != b'='", " && byte != b';'"),
    ("sha256 constant", BI, "0x428a2f98, 0x71374491", "0x428a2f98, 0x71374490"),
    ("reviewed inputs reordered", BI, '    "../../schemas/physics_source_recovery.schema.json",\n    "../../schemas/retained_precision_mp_v2.schema.json",',
     '    "../../schemas/retained_precision_mp_v2.schema.json",\n    "../../schemas/physics_source_recovery.schema.json",'),
    ("build.rs: empty value is a failure", BS, '    let var = |name: &str| std::env::var(name).ok();', '    let var = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());'),
    ("build.rs: os and env swapped", BS, '        var("CARGO_CFG_TARGET_OS")?,\n        var("CARGO_CFG_TARGET_ENV")?,', '        var("CARGO_CFG_TARGET_ENV")?,\n        var("CARGO_CFG_TARGET_OS")?,'),
    ("build.rs: debug_assertions inverted", BS, '{ "true" } else { "false" }', '{ "false" } else { "true" }'),
    ("stack override ignored", RM, "    if let Some(bytes) = RESERVED_STACK_OVERRIDE.with(std::cell::Cell::get) {", "    if let Some(bytes) = None::<usize> {"),
    ("stack R halved", RM, "pub(super) const RESERVED_STACK_BYTES: usize = 64 << 20;", "pub(super) const RESERVED_STACK_BYTES: usize = 32 << 20;"),
    ("stack refusal kind", RM, "pub(super) const STACK_RESERVATION_PRECONDITION: UnavailablePrecondition = UnavailablePrecondition::ResourceAdmission;",
     "pub(super) const STACK_RESERVATION_PRECONDITION: UnavailablePrecondition = UnavailablePrecondition::SourceFamily;"),
    ("admission ignores the refusal", RM, "        (None, Some(profile)) => Ok(CapturePermit { _profile: profile }),", "        (_, Some(profile)) => Ok(CapturePermit { _profile: profile }),"),
    ("admit drops the domain verdict", RM, "    report.law.domain = domain.err();\n", ""),
    ("admit drops the refusal", RM, "        Err(refusal) => report.law.refusal = Some(refusal),", "        Err(_refusal) => {}"),
]

def run():
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--lib", "--target-dir", TARGET, "retained_memory"],
                       cwd=COPY, env=env, capture_output=True, text=True, timeout=2400)
    out = p.stdout + p.stderr
    if "could not compile" in out or "error[E" in out:
        return "COMPILE", out[-600:]
    return ("KILLED" if p.returncode != 0 else "SURVIVED"), [l for l in out.splitlines() if l.startswith("test ") and "FAILED" in l][:4]

for name, rel, old, new in MUTANTS:
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
    print(json.dumps({"mutant": name, "file": rel, "result": result, "equivalent": EQUIVALENT.get(name), "detail": detail}), flush=True)
for f in (RM, BI, BS):
    shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))
