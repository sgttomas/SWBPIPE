"""RV102: machine-absolute path counts per changed file at the PR head.

Usage: python abs_scan.py <copy_root> <name_status_tsv> <t3_rel_prefix>
  copy_root        the git-archive copy of the PR head
  name_status_tsv  `git diff --no-renames --name-status <main> <head>` output
  t3_rel_prefix    the T3 folder's repository-relative path (shortened to T3)

For every changed file it prints: GEN-8 detector hit lines
(surface_roles.iter_machine_path_lines), broad-pattern hit lines, status,
short path, and up to three samples. Path roots are assembled at run time
so this file carries no literal machine path.
"""
import re
import sys
from pathlib import Path

copy_root, listfile, t3rel = sys.argv[1:4]
sys.path.insert(0, str(Path(copy_root) / "tools" / "practitioner_harness"))
import surface_roles as sr  # noqa: E402

S = "/"
ROOTS = [
    S + "Users" + S + r"[^\s`\"')]+",
    S + "private" + S + r"(?:tmp|var)[^\s`\"')]*",
    S + "var" + S + "folders" + S + r"[^\s`\"')]*",
    r"(?<![\w.<])" + S + "tmp" + S + r"[^\s`\"')]*",
    S + "Volumes" + S + r"[^\s`\"')]+",
    S + "home" + S + r"[a-z][^\s`\"')]*",
    r"[A-Z]:\\\\[^\s]+",
]
pat = re.compile("(" + "|".join(ROOTS) + ")")

for row in open(listfile).read().splitlines():
    if not row:
        continue
    status, name = row.split("\t", 1)
    data = (Path(copy_root) / name).read_bytes()
    short = name.replace(t3rel, "T3")
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        print(f"BINARY\tBINARY\t{status}\t{short}\t")
        continue
    sr_hits = list(sr.iter_machine_path_lines(text))
    rx = [(i + 1, m.group(0)) for i, line in enumerate(text.split("\n")) for m in pat.finditer(line)]
    rxlines = sorted(set(i for i, _ in rx))
    samples = ";".join(f"{i}:{s[:60]}" for i, s in rx[:3])
    print(f"{len(sr_hits)}\t{len(rxlines)}\t{status}\t{short}\t{samples}")
