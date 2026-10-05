"""RV89 G6: its own mutants of the registration entry, the registered admission and G-C's
solve-attempt fact, in a registered copy (registration.diff applied). One exact replacement at a
time; `cargo test --lib retained_memory` (one cargo job). KILLED / SURVIVED / COMPILE."""
import json, os, shutil, subprocess, sys
CAND, COPY, TARGET = sys.argv[1:4]
RM = "product_physics/src/retained_memory.rs"
MUTANTS = [
    ("R1 registered identity: another rustc release", RM, '    identity: "v1;rustc.release=1.97.1;', '    identity: "v1;rustc.release=1.97.2;'),
    ("R2 registered reviewed input: one lock digit", RM, "Cargo.lock=4f494db6d8a6", "Cargo.lock=4f494db7d8a6"),
    ("R3 registered reader layout: Validation 64", RM, "        TypeLayout { size: 56, align: 8 },", "        TypeLayout { size: 64, align: 8 },"),
    ("R4 threshold below the maximum", RM, "    threshold_bytes: 4_026_531_840,", "    threshold_bytes: 3_500_000_000,"),
    ("R5 threshold unbounded", RM, "    threshold_bytes: 4_026_531_840,", "    threshold_bytes: u64::MAX,"),
    ("R6 admit drops R from the bound", RM, "bound_admits(maximum, RESERVED_STACK_BYTES as u64, REGISTERED_PROFILES[index].threshold_bytes)", "bound_admits(maximum, 0, REGISTERED_PROFILES[index].threshold_bytes)"),
    ("R7 G-C attempt fact inverted", RM, "o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture))),", "o(P::OrdinarySolveNotAttempted, u64::from(ordinary_solve_attempted(f.capture))),"),
    ("R8 a deferred-formation attempt not counted", RM, "capture.ordinary.iter().all(|seed| seed.initial.is_some())", "capture.ordinary.iter().all(|seed| !matches!(seed.initial, None | Some(crate::retained_product::InitialSeed::FormationFailure { .. })))"),
    ("R9 the attempt fact checks only the first seed", RM, "capture.ordinary.iter().all(|seed| seed.initial.is_some())", "capture.ordinary.first().is_some_and(|seed| seed.initial.is_some())"),
]
def run():
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--lib", "--target-dir", TARGET, "retained_memory"],
                       cwd=os.path.join(COPY, "product_physics"), env=env, capture_output=True, text=True, timeout=2400)
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
    shutil.copyfile(os.path.join(CAND, RM), os.path.join(COPY, RM))
    path = os.path.join(COPY, rel)
    text = open(path).read()
    if text.count(old) != 1:
        print(json.dumps({"mutant": name, "result": "NOT APPLIED", "count": text.count(old)}), flush=True); continue
    open(path, "w").write(text.replace(old, new))
    result, detail = run()
    print(json.dumps({"mutant": name, "file": rel, "result": result, "detail": detail}), flush=True)
shutil.copyfile(os.path.join(CAND, RM), os.path.join(COPY, RM))
