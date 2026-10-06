"""RV102: credential, personal-data and whole-host-data screen of the text
the PR adds.

Usage: python publication_scan.py <git_dir> <main> <head> <name_status_tsv> <t3_rel> <out_tsv>

Scanned text: every line of each added file, and the '+' lines of each
modified file (git diff main head). Prints a per-pattern summary and writes
every hit (pattern, short path, line number, first 160 characters) to out_tsv.
Patterns that would read as literal host paths are assembled at run time.
"""
import re
import subprocess
import sys
from collections import defaultdict

git_dir, main, head, listfile, t3rel, out_tsv = sys.argv[1:7]
S = "/"
PATTERNS = {
    # credentials and key material (brief's list, plus close relatives)
    "ghp_": r"ghp_[A-Za-z0-9]{10,}|ghp_",
    "gho_": r"gho_",
    "ghs_/ghu_/ghr_": r"gh[sur]_[A-Za-z0-9]{10,}",
    "github_pat_": r"github_pat_",
    "sk-": r"(?<![A-Za-z0-9])sk-[A-Za-z0-9_-]{8,}",
    "sk- (any)": r"sk-",
    "AKIA": r"AKIA[0-9A-Z]{8,}|AKIA",
    "PRIVATE KEY": r"BEGIN .*PRIVATE KEY",
    "password": r"(?i)password",
    "secret": r"(?i)secret",
    "token=": r"(?i)token=",
    "Authorization:": r"(?i)authorization:",
    "Bearer": r"Bearer [A-Za-z0-9._-]{8,}",
    "slack": r"xox[abprs]-",
    "cred env": r"GH_TOKEN|GITHUB_TOKEN|ANTHROPIC_API_KEY|OPENAI_API_KEY|AWS_SECRET|SSH_AUTH_SOCK|NPM_TOKEN",
    # personal data
    "email": r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}",
    # whole-host data
    "Applications dir": S + "Applications" + S,
    "app names": r"Microsoft Teams|Parallels|ChatGPT|Claude\.app|Messages\.app|Mail\.app|Xcode\.app|Slack\.app|Safari|Google Chrome",
    "session flags": r"--resume|--session-id|--session_id|session_id|sessionId|user-data-dir",
    "system dirs": S + r"System" + S + "Library|" + S + "usr" + S + "libexec|launchd",
    "host name": r"MacBook|\.local\b",
    "uuid": r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
    "ps header": r"\bPID\s+TTY\b|\bUSER\s+PID\s+%CPU|\bPID\s+PPID\b",
    "ps row": r"^\s*\d+\s+\d+\s+\S+\s+\d+:\d+\.\d+\s+" + S,
    "home/tmp roots": S + "Users" + S + "|" + S + "private" + S + "|" + S + "var" + S + "folders|" + S + "home" + S + "|" + S + "Volumes" + S,
    "claude/codex ids": r"claude-501|\.claude" + S + "projects|agent-[0-9a-f]{8,}|conversation[_-]?id",
}
compiled = {k: re.compile(v) for k, v in PATTERNS.items()}


def git(*args):
    return subprocess.run(["git", "-C", git_dir, *args], capture_output=True, check=True).stdout


hits = defaultdict(list)
scanned_lines = 0
for row in open(listfile).read().splitlines():
    status, name = row.split("\t", 1)
    short = name.replace(t3rel, "T3")
    if status == "A":
        text = git("show", f"{head}:{name}").decode("utf-8", "replace")
        lines = list(enumerate(text.split("\n"), start=1))
    else:
        diff = git("diff", "-U0", main, head, "--", name).decode("utf-8", "replace")
        lines = []
        new_no = 0
        for d in diff.split("\n"):
            m = re.match(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", d)
            if m:
                new_no = int(m.group(1))
                continue
            if d.startswith("+") and not d.startswith("+++"):
                lines.append((new_no, d[1:]))
                new_no += 1
    for ln, line in lines:
        scanned_lines += 1
        for k, rx in compiled.items():
            if rx.search(line):
                hits[k].append((short, ln, line.strip()[:160]))

with open(out_tsv, "w") as fh:
    fh.write("pattern\tpath\tline\ttext\n")
    for k in PATTERNS:
        for short, ln, t in hits[k]:
            fh.write(f"{k}\t{short}\t{ln}\t{t}\n")
print(f"scanned lines: {scanned_lines}")
for k in PATTERNS:
    files = {h[0] for h in hits[k]}
    print(f"{k}\thits={len(hits[k])}\tfiles={len(files)}")
