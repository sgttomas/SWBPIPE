"""I72 U8-4: I69's note 6. result_export's cfg(test) module u6e_reader_round_tests include_str!s the corpus and reads
only `d37` and `cases[0]`. Print, per revision, the corpus file sha256, the case count, and the sha256 of each of
those two members (canonical: sorted keys, compact). Git reads only (GIT_OPTIONAL_LOCKS=0).
Usage: python3 corpus_pins_check.py <repo>"""
import hashlib, json, os, subprocess, sys
repo = sys.argv[1]; f = "projects/chirality-piping/fixtures/results/retained_precision_cases.json"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
h = lambda v: hashlib.sha256(json.dumps(v, sort_keys=True, separators=(",", ":")).encode()).hexdigest()[:16]
for name, rev in [("Pass A", "ba1faa1c858ce3630a22767677310b1902a14b83"), ("F'", "5488136a193ef8921bd88c33a8124647c4cb352d"),
                  ("U8 base", "b1e2d7741e03b69427dd71045e024dfd2deff0f5"), ("07l", "69a925bd68"), ("U8 head", "bd6b4be2c33cc64edf3e273bc126083872d03e24")]:
    b = subprocess.run(["git", "-C", repo, "show", f"{rev}:{f}"], capture_output=True, check=True, env=env).stdout; d = json.loads(b)
    print(f"{name:8} {rev[:10]} file={hashlib.sha256(b).hexdigest()[:16]} cases={len(d['cases'])} d37={h(d['d37'])} cases[0]={h(d['cases'][0])}")
