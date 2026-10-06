"""I72 U8-4: where the L = 0 fixtures and corpus 07l are compiled in, read from this run's own build artefacts
(stdlib only; read-only). include_str! embeds a file's whole text contiguously, so the whole text is the probe.
Usage: python3 fixture_embedding_check.py <basis projects/chirality-piping> <WT/targets/i72-u8> <tag>"""
import sys, os, glob, hashlib
B, TG, TAG = sys.argv[1:4]
probes = {m: open(f"{B}/fixtures/results/retained_precision_l0_successor_{m}.json", "rb").read() for m in ("sparse_interactive", "dense_scrutiny")}
corpus = open(f"{B}/fixtures/results/retained_precision_cases.json", "rb").read()
print("probes: each L = 0 fixture's whole text; sizes and sha256",
      {m: (len(v), hashlib.sha256(v).hexdigest()[:12]) for m, v in probes.items()}, "; corpus 07l", (len(corpus), hashlib.sha256(corpus).hexdigest()[:12]))
ex = lambda p: os.path.isfile(p) and os.access(p, os.X_OK) and not p.endswith((".d", ".rlib", ".rmeta", ".dylib"))
W, WR = f"{TG}/work-pass_{TAG}/debug/deps", f"{TG}/work-runner-pass_{TAG}/debug/deps"
arts = {"PP lib test binary (cfg(test))": [p for p in glob.glob(f"{W}/open_pipe_stress_product_physics-*") if ex(p)],
        "PP rlib, PP target (no cfg(test))": glob.glob(f"{W}/libopen_pipe_stress_product_physics-*.rlib"),
        "PP integration-test binaries (PP lib without cfg(test))": [p for p in glob.glob(f"{W}/*") if ex(p) and "open_pipe_stress_product_physics-" not in p],
        "PP rlib, runner target (no cfg(test))": glob.glob(f"{WR}/libopen_pipe_stress_product_physics-*.rlib"),
        "runner/headless binaries": [p for p in glob.glob(f"{WR}/*") if ex(p)]}
for name, ps in arts.items():
    found = {m: 0 for m in probes}; fc = 0
    for p in ps:
        data = open(p, "rb").read()
        for m, v in probes.items(): found[m] += v in data
        fc += corpus in data
    print(f"{name}: {len(ps)} artefact(s); L0 fixture found in: {found}; corpus found in: {fc}")
