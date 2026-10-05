#!/usr/bin/env python3
"""RV96 read-only secret/PII scan over the PR's added/modified execution files.
usage: secret_scan.py <archive_root> <paths_file> <tar_extract_dir> <out_tsv>
Scans plain files, decompressed .gz payloads and extracted tar members."""
import gzip, re, sys
from pathlib import Path

root, paths_file, tarx, out = sys.argv[1:5]
PATS = {
    "ghp_": re.compile(r"ghp_[A-Za-z0-9]{8,}"),
    "gho_": re.compile(r"gho_[A-Za-z0-9]{8,}"),
    "ghs_/ghu_/ghr_": re.compile(r"gh[sur]_[A-Za-z0-9]{20,}"),
    "github_pat_": re.compile(r"github_pat_[A-Za-z0-9_]{8,}"),
    "sk-(key-like)": re.compile(r"(?<![A-Za-z0-9])sk-(?:ant-|proj-|live-|test-)?[A-Za-z0-9_-]{20,}"),
    "sk-(any)": re.compile(r"(?<![A-Za-z0-9])sk-"),
    "AKIA": re.compile(r"AKIA[0-9A-Z]{16}"),
    "PRIVATE KEY": re.compile(r"BEGIN [A-Z ]*PRIVATE KEY"),
    "password": re.compile(r"password", re.I),
    "secret": re.compile(r"secret", re.I),
    "token=": re.compile(r"token=", re.I),
    "Authorization:": re.compile(r"Authorization:", re.I),
    "Bearer": re.compile(r"Bearer\s+[A-Za-z0-9._-]{20,}"),
    "xox[bp]-": re.compile(r"xox[abprs]-[A-Za-z0-9-]{10,}"),
    "email": re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"),
}

def scan_text(label, text, rows):
    for i, line in enumerate(text.splitlines(), 1):
        for name, rx in PATS.items():
            for m in rx.finditer(line):
                s = max(0, m.start() - 40)
                ex = line[s:m.end() + 40].replace("\t", " ")
                rows.append((name, label, i, m.group(0)[:60], ex[:200]))

rows = []
n = 0
for p in Path(paths_file).read_text().splitlines():
    f = Path(root) / p
    try:
        data = f.read_bytes()
    except OSError:
        continue
    n += 1
    if p.endswith(".gz"):
        try:
            data = gzip.decompress(data)
            if p.endswith(".tar.gz"):
                continue  # members scanned from tarx below
        except OSError:
            pass
    scan_text(p, data.decode("utf-8", "replace"), rows)
for f in sorted(Path(tarx).rglob("*")):
    if f.is_file():
        n += 1
        scan_text("TAR:" + str(f.relative_to(tarx)), f.read_bytes().decode("utf-8", "replace"), rows)

with open(out, "w") as fh:
    for r in rows:
        fh.write("\t".join(map(str, r)) + "\n")
from collections import Counter
print("files scanned", n)
for k, v in sorted(Counter(r[0] for r in rows).items()):
    print(f"{k}\t{v}")
