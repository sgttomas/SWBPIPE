#!/usr/bin/env python3
"""RV117 publication screen over the text PR #1111 adds (method of RV103's scan, same patterns).
Added files: every line (a .gz file is decompressed first). Modified files: '+' lines of `git diff M H`.
Usage: publication_scan.py <num_repo> <M> <H> <out_dir>"""
import re, subprocess, sys, os, collections, gzip
repo, M, H, out = sys.argv[1:5]
def git(*a): return subprocess.run(["git","-C",repo,*a],capture_output=True,check=True).stdout
ns = git("diff","--no-renames","--name-status",M,H).decode().splitlines()
units = []  # (path, lineno, text)
for row in ns:
    st, path = row.split("\t",1)
    if st == "A":
        raw = git("show", f"{H}:{path}")
        if path.endswith(".gz"): raw = gzip.decompress(raw)
        data = raw.decode("utf-8","replace")
        for i,l in enumerate(data.splitlines(),1): units.append((path,i,l))
    elif st == "M":
        d = git("diff","-U0",M,H,"--",path).decode("utf-8","replace")
        ln = 0
        for l in d.splitlines():
            m = re.match(r"@@ -\S+ \+(\d+)", l)
            if m: ln = int(m.group(1)); continue
            if l.startswith("+") and not l.startswith("+++"):
                units.append((path,ln,l[1:])); ln += 1
home = "/" + "Users" + "/"
pats = {
 # credentials
 "ghp_": r"ghp_", "gho_": r"gho_", "ghs_/ghu_/ghr_": r"gh[sur]_[A-Za-z0-9]{8,}", "github_pat_": r"github_pat_",
 "sk- (any)": r"sk-", "sk- token-shaped": r"\bsk-[A-Za-z0-9_-]{16,}", "AKIA": r"AKIA[0-9A-Z]{8,}|AKIA",
 "PRIVATE KEY": r"BEGIN .*PRIVATE KEY", "password": r"(?i)password", "secret": r"(?i)secret",
 "token=": r"(?i)token=", "Authorization:": r"(?i)Authorization:", "Bearer": r"Bearer [A-Za-z0-9._-]{12,}",
 "slack token": r"xox[abprs]-", "cred vars": r"GH_TOKEN|GITHUB_TOKEN|ANTHROPIC_API_KEY|OPENAI_API_KEY|AWS_SECRET|SSH_AUTH_SOCK|NPM_TOKEN",
 "api_key": r"(?i)api[_-]?key",
 # personal data
 "email": r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}", "owner name": r"(?i)ryan|tufts",
 # whole-host data
 "Applications": r"/Applications/", "apps": r"(?i)\b(Microsoft Teams|Parallels|ChatGPT|Claude\.app|Messages\.app|Mail\.app|Slack|Google Chrome|Safari|Spotify|Dropbox|Docker Desktop|Zoom\.us|1Password|Discord|WhatsApp|Firefox|Arc\.app|Cursor\.app|Visual Studio Code)\b",
 "session ids": r"--resume|--session-id|session_id|sessionId|user-data-dir|conversation[_-]?id",
 "uuid": r"\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b",
 "toolcall ids": r"\btoolu_[A-Za-z0-9]+|\bmsg_[A-Za-z0-9]{10,}|call_[A-Za-z0-9]{16,}",
 "transcripts": r"\.claude/projects/|\.jsonl\b.*transcript|/\.codex/sessions|rollout-",
 "system paths": r"/System/Library|/usr/libexec|launchd|" + "/" + "private/var/|" + "/" + "var/folders/",
 "host names": r"(?i)MacBook|\b[\w-]+\.local\b",
 "ps headers": r"\bPID\s+(TTY|PPID|USER)|\bUSER\s+PID\b|%CPU|%MEM",
 "ps/pgrep/lsof": r"\bps (aux|-e|-ax|-A)|\bpgrep\b|\blsof\b|\btop -l",
 "home path": re.escape(home), "tilde home": "~" + "/" + r"(dev|Library|Documents|Desktop)",
 # the brief's strict pattern, assembled at run time
 "strict": "~" + "/" + "|" + home + "|" + "/" + "private" + "/" + r"|\." + "claude" + "/" + "worktrees" + "|" + "swbpipe" + "-control-layer",
}
owner_emails = {git("log","-1","--format=%ae",H).decode().strip(), git("log","-1","--format=%ce",H).decode().strip()}
def sanitize(t):
    for e in owner_emails:
        if e: t = t.replace(e, "<owner-email>")
    t = t.replace(home, "/U-sers/").replace("/"+"private"+"/", "/pri-vate/").replace("~"+"/", "~-/")
    t = t.replace("."+"claude"+"/"+"worktrees", ".claude/work-trees").replace("swbpipe"+"-control-layer", "swbpipe-c-l")
    return t
cre = {k: re.compile(v) for k,v in pats.items()}
hits = collections.defaultdict(list)
for path,ln,txt in units:
    for k,c in cre.items():
        if c.search(txt): hits[k].append((path,ln,txt.strip()[:220]))
os.makedirs(out, exist_ok=True)
T3 = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
def short(p): return p.replace(T3,"T3/").replace("projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/","WG-dir/")
with open(os.path.join(out,"publication_scan_summary.txt"),"w") as f:
    f.write(f"M={M}\nH={H}\nchanged paths: {len(ns)}\nlines scanned: {len(units)}\n")
    for k in pats:
        h = hits.get(k,[])
        files = len({p for p,_,_ in h})
        f.write(f"{k}\t{len(h)} hits\t{files} files\n")
with open(os.path.join(out,"publication_hits.tsv"),"w") as f:
    f.write("pattern\tpath\tline\ttext\n")
    for k in pats:
        for p,ln,t in hits.get(k,[]):
            t = sanitize(t)
            f.write(f"{k}\t{short(p)}\t{ln}\t{t}\n")
print(open(os.path.join(out,"publication_scan_summary.txt")).read())
