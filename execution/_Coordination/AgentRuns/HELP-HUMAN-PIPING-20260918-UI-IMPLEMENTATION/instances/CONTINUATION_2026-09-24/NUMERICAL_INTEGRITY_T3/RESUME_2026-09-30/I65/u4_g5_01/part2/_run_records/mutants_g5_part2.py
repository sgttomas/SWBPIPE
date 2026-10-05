"""I65 U4 G5 part 2, control 3: the mutant run on the part-2 candidate (stdlib only).

Three sets, each one exact text replacement in a scratch copy of the candidate core tree, then
`cargo test --lib retained_memory` in product_physics (the default toolchain, --locked --offline,
one cargo job, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2):
  P1  part 1's 84 (../../_run_records/mutants_g5.py), re-run on part 2, with the anchors part 2
      changed restated below (ANCHOR_UPDATES);
  RV  RV89's 24 (../../../../REVIEW_RV89/u4_g5_01/evidence/mutants_rv89.py), unchanged, so its
      seven survivors (V03, V06, V07, V08, V17, V19, V24) are re-run against the new tests;
  P2  part 2's own: the profile (the Estimate gate, the maximum, the node law, the push law, the
      forms and combinations, the in-build bindings), the profile-derived gate and budget
      bounds, and RV89 N-4's reviewed-input guard.
Outcomes: KILLED (builds, a test fails), SURVIVED (builds, every test passes), COMPILE (the build
fails: a control failure, no mutant may be killed only this way). Equivalent mutants are listed
with their reason. Before each mutant every mutated file is restored from the candidate.
Usage: python3 mutants_g5_part2.py <candidate core dir> <scratch copy core dir> <target dir> [P1,RV,P2]
"""
import json, os, shutil, subprocess, sys

CAND, COPY, TARGET = sys.argv[1:4]
SETS = set(sys.argv[4].split(",")) if len(sys.argv) > 4 else {"P1", "RV", "P2"}
HERE = os.path.dirname(os.path.abspath(__file__))
PP = "product_physics/"
RM, BI, BS = PP + "src/retained_memory.rs", PP + "src/build_identity.rs", PP + "build.rs"
LT = PP + "src/retained_memory_law_tests.rs"
FKR = "solver/frame_kernel/src/structural/retained_resource.rs"
RESTORE = (RM, BI, BS, LT, FKR)

def load(path):
    """The MUTANTS (and EQUIVALENT) of a harness, read without running it."""
    text = open(path).read()
    namespace = {"__name__": "loaded", "sys": type("S", (), {"argv": ["x", "a", "b", "c"]})}
    exec(text.split("\ndef run():")[0].replace("import json, os, shutil, subprocess, sys", "import json, os, shutil, subprocess"), namespace)
    return namespace["MUTANTS"], namespace.get("EQUIVALENT", {})

P1, EQUIVALENT = load(os.path.join(HERE, "../../_run_records/mutants_g5.py"))
RV, _ = load(os.path.join(HERE, "../../../../REVIEW_RV89/u4_g5_01/evidence/mutants_rv89.py"))
# Part 1 anchors that part 2 rewrote (same mutation, the new text).
ANCHOR_UPDATES = {
    "bound priced at zero": ("    priced_maximum(profile::ESTIMATES, mode)\n", "    Ok(0)\n"),
    "bindings ignore witnesses": ("    witnesses\n        && compiled_inputs.is_some_and(every_input_read)", "    true\n        && compiled_inputs.is_some_and(every_input_read)"),
    "bindings ignore inputs": ("        && compiled_inputs == Some(registered_inputs)\n", ""),
}
# RV89 anchors that part 2 rewrote (RV89 N-4's guard moved the reviewed-input comparison).
RV_UPDATES = {
    "V12 lock-hash comparison is presence only": ("        && compiled_inputs == Some(registered_inputs)\n", "        && compiled_inputs.is_some()\n"),
}
# Part 1's equivalents (part 2 adds none in advance: any survivor is reported with its reason).
EQUIVALENT = dict(EQUIVALENT)

P2 = [
    # The Estimate gate and the maximum.
    ("P2 profile: Estimate gate removed", RM, "    if estimates != 0 {\n        return Err(BoundRefusal::Unpriced);\n    }\n", ""),
    ("P2 profile: Estimate gate inverted", RM, "    if estimates != 0 {\n        return Err(BoundRefusal::Unpriced);", "    if estimates == 0 {\n        return Err(BoundRefusal::Unpriced);"),
    ("P2 profile: Estimate count ignored", RM, "    priced_maximum(profile::ESTIMATES, mode)\n", "    priced_maximum(0, mode)\n"),
    ("P2 profile: estimates not counted", RM, "            if matches!(ATOM_BINDINGS[i], Binding::Estimate) {\n                n += 1;", "            if matches!(ATOM_BINDINGS[i], Binding::Estimate) {\n                n += 0;"),
    ("P2 profile: modes swapped", RM, "crate::PreviewSolverMode::SparseInteractive => profile::SPARSE,", "crate::PreviewSolverMode::SparseInteractive => profile::DENSE,"),
    ("P2 profile: maximum keeps the smallest", RM, "                Some((b, j)) if b >= e => Some((b, j)),", "                Some((b, j)) if b <= e => Some((b, j)),"),
    ("P2 profile: maximum ignores moving", RM, "            let Some(e) = add(phases[i].0, phases[i].1) else { return None };", "            let Some(e) = phases[i].0 else { return None };"),
    ("P2 profile: max takes the smaller", RM, "            (Some(a), Some(b)) => Some(if a > b { a } else { b }),", "            (Some(a), Some(b)) => Some(if a < b { a } else { b }),"),
    ("P2 profile: add saturates", RM, "            (Some(a), Some(b)) => a.checked_add(b),", "            (Some(a), Some(b)) => Some(a.saturating_add(b)),"),
    ("P2 profile: form drops its constant", RM, "        let mut total = f.constant;", "        let mut total = 0 * f.constant;"),
    ("P2 profile: form drops its last term", RM, "        while i < f.terms.len() {", "        while i + 1 < f.terms.len() {"),
    ("P2 profile: form coefficient ignored", RM, "            let Some(x) = v[a].checked_mul(c) else { return None };", "            let Some(x) = v[a].checked_mul(1 + 0 * c) else { return None };"),
    ("P2 profile: T16 is a sum", RM, "        max(max(max(form(&FORMS[18], v), form(&FORMS[19], v)), form(&FORMS[20], v)), form(&FORMS[21], v))",
     "        add(max(max(form(&FORMS[18], v), form(&FORMS[19], v)), form(&FORMS[20], v)), form(&FORMS[21], v))"),
    ("P2 profile: W3 drops T16 (sparse)", RM, "form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[7], v)), t16(v)),",
     "form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[7], v)), Some(0)),"),
    ("P2 profile: X2 drops the reserve (dense)", RM, "(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[0], v)), form(&FORMS[45], v)), form(&FORMS[3], v)),",
     "(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[0], v)), form(&FORMS[45], v)), Some(0)),"),
    # The node law and the in-build bindings.
    ("P2 node: drops the internal node", RM, "        max_usize(leaf, internal)\n", "        leaf\n"),
    ("P2 node: alignment floor 8 dropped", RM, "        let a = max_usize(8, max_usize(ak, av));", "        let a = max_usize(ak, av);"),
    ("P2 node: 11 keys read as 10", RM, "        let leaf = up(8, a) + up(2, a) + up(2, a) + up(11 * sk, a) + up(11 * sv, a);", "        let leaf = up(8, a) + up(2, a) + up(2, a) + up(10 * sk, a) + up(11 * sv, a);"),
    ("P2 binding: ResultItem read as Diagnostic", RM, "        (size_of::<crate::ResultItem>()) as u64, // s(ResultItem)", "        (size_of::<crate::Diagnostic>()) as u64, // s(ResultItem)"),
    ("P2 binding: SR node stride dropped", RM, "        (open_pipe_stress_stress_recovery::elastic_extrema::NODE_STRIDE) as u64,", "        (64) as u64,"),
    ("P2 binding: Content pair halved", RM, "        (64) as u64, // s((Content,Content))", "        (32) as u64, // s((Content,Content))"),
    ("P2 binding: Projection upper loses a String", RM, "        (up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + ",
     "        (0 * up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + "),
    ("P2 FK export: Wide<16> read as Wide<8>", FKR, "WIDE_4 = Wide<4>; WIDE_8 = Wide<8>; WIDE_16 = Wide<16>;", "WIDE_4 = Wide<4>; WIDE_8 = Wide<8>; WIDE_16 = Wide<8>;"),
    ("P2 FK export: ProductFinalRow read as ProductRecipe", FKR, "PRODUCT_FINAL_ROW = ProductFinalRow<'static>;", "PRODUCT_FINAL_ROW = ProductRecipe;"),
    ("P2 FK export: Formation align 1", FKR, "pub const FORMATION_ALIGN: usize = align_of::<Formation>();", "pub const FORMATION_ALIGN: usize = 1;"),
    # The profile-derived gate caps and budgets.
    ("P2 gate: late cap keeps the late capture", RM, "            profile_bytes(profile::F_T11).saturating_sub(profile_bytes(profile::F_T11_LATE_CAPTURE))],", "            profile_bytes(profile::F_T11)],"),
    ("P2 gate: seed cap is T11", RM, "            profile_bytes(profile::F_T11_ORDINARY_SEED),", "            profile_bytes(profile::F_T11),"),
    ("P2 gate: observation cap is the seed's", RM, "            profile_bytes(profile::F_T11),\n            profile_bytes(profile::F_T11_ORDINARY_SEED),", "            profile_bytes(profile::F_T11_ORDINARY_SEED),\n            profile_bytes(profile::F_T11_ORDINARY_SEED),"),
    ("P2 gate: D_env slots unpushed", RM, "            push_capacity(text_atoms::D_ENV),", "            text_atoms::D_ENV,"),
    ("P2 gate: P_final slots unpushed", RM, "            push_capacity(P_FINAL),", "            P_FINAL,"),
    ("P2 gate: overflowed form reads u64::MAX", RM, "        Some(bytes) => bytes,\n        None => 0,", "        Some(bytes) => bytes,\n        None => u64::MAX,"),
    ("P2 budget: reader unchecked", RM, "    precommit_reader_bytes: checked_or_zero(profile::t17(&profile::ATOM_VALUES)),", "    precommit_reader_bytes: 0,"),
    ("P2 push: doubles from 8", RM, "    let mut c = 4;\n    while c < h {", "    let mut c = 8;\n    while c < h {"),
    ("P2 push: off by one", RM, "    while c < h {\n        c *= 2;", "    while c <= h {\n        c *= 2;"),
    ("P2 push: zero is four", RM, "    if h == 0 {\n        return 0;\n    }\n    let mut c = 4;", "    if h == 0 {\n        return 4;\n    }\n    let mut c = 4;"),
    ("P2 text: D_env is D", RM, "    pub(crate) const D_ENV: u64 = p::TEXT_D_ENV;", "    pub(crate) const D_ENV: u64 = p::TEXT_D;"),
    ("P2 budget: staged and successor swapped", RM, "staged_copy_bytes: profile_bytes(profile::F_STAGED),", "staged_copy_bytes: profile_bytes(profile::F_SUCC),"),
    ("P2 budget: reader is the successor's", RM, "precommit_reader_bytes: checked_or_zero(profile::t17(&profile::ATOM_VALUES)),", "precommit_reader_bytes: profile_bytes(profile::F_SUCC),"),
    # RV89 N-4.
    ("P2 N-4: unavailable inputs bind", RM, "        && compiled_inputs.is_some_and(every_input_read)\n", ""),
    ("P2 N-4: only the first input checked", RM, "!text.split(';').any(|field| field.ends_with(\"=unavailable\"))", "!text.split(';').take(2).any(|field| field.ends_with(\"=unavailable\"))"),
]

def run():
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--lib", "--target-dir", TARGET, "retained_memory"],
                       cwd=os.path.join(COPY, "product_physics"), env=env, capture_output=True, text=True, timeout=2400)
    out = p.stdout + p.stderr
    if "could not compile" in out or "error[E" in out:
        return "COMPILE", out[-600:]
    return ("KILLED" if p.returncode != 0 else "SURVIVED"), [l for l in out.splitlines() if l.startswith("test ") and "FAILED" in l][:4]

def restore():
    for f in RESTORE:
        shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))

plan = []
if "P1" in SETS:
    for name, rel, old, new in P1:
        if name in ANCHOR_UPDATES:
            old, new = ANCHOR_UPDATES[name]
        plan.append(("P1", name, PP + rel, old, new))
if "RV" in SETS:
    plan += [("RV", name, PP + rel, *RV_UPDATES.get(name, (old, new))) for name, rel, old, new in RV]
if "P2" in SETS:
    plan += [("P2", name, rel, old, new) for name, rel, old, new in P2]
for group, name, rel, old, new in plan:
    if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
        print(json.dumps({"mutant": name, "result": "STOPPED: memory guard not running"})); sys.exit(9)
    restore()
    path = os.path.join(COPY, rel)
    text = open(path).read()
    if text.count(old) != 1:
        print(json.dumps({"set": group, "mutant": name, "result": "NOT APPLIED", "count": text.count(old)}), flush=True)
        continue
    open(path, "w").write(text.replace(old, new))
    result, detail = run()
    print(json.dumps({"set": group, "mutant": name, "file": rel, "result": result, "equivalent": EQUIVALENT.get(name), "detail": detail}), flush=True)
restore()
