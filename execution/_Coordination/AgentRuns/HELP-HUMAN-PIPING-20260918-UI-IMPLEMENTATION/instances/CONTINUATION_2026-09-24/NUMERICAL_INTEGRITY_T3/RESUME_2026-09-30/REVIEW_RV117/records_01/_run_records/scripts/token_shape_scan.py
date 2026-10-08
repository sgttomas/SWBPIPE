#!/usr/bin/env python3
"""RV117 (method of RV103, as RV110). Strict token-shaped credential patterns over every changed file's full content at H
(added files whole, .gz decompressed; modified files whole)."""
import re, subprocess, sys, gzip
repo, M, H = sys.argv[1:4]
def git(*a): return subprocess.run(["git","-C",repo,*a],capture_output=True,check=True).stdout
pats = {
 "ghp/gho/ghs/ghu/ghr token": r"\bgh[pousr]_[A-Za-z0-9]{36}\b",
 "github_pat token": r"\bgithub_pat_[A-Za-z0-9_]{50,}",
 "sk- key": r"\bsk-(?:ant-|proj-)?[A-Za-z0-9_-]{20,}",
 "AKIA key": r"\bAKIA[0-9A-Z]{16}\b",
 "PEM block": r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
 "password=value": r"(?i)password\s*[:=]\s*['\"]?[^\s'\"<>{}$]{6,}",
 "token=value": r"(?i)token=[A-Za-z0-9._-]{12,}",
 "Authorization header value": r"(?i)Authorization:\s*(Bearer|token|Basic)\s+[A-Za-z0-9._=-]{12,}",
 "slack": r"xox[abprs]-[A-Za-z0-9-]{10,}",
 "jwt": r"\beyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.",
 "sk- anywhere": r"sk-",
}
ns = [r.split("\t",1) for r in git("diff","--no-renames","--name-status",M,H).decode().splitlines()]
for k,v in pats.items():
    c = re.compile(v); n=0; where=set()
    for st,p in ns:
        raw = git("show",f"{H}:{p}")
        if p.endswith(".gz"): raw = gzip.decompress(raw)
        data = raw.decode("utf-8","replace")
        for i,l in enumerate(data.splitlines(),1):
            if c.search(l): n+=1; where.add((p.rsplit('/',1)[-1], st))
    print(f"{k}\t{n}\t{sorted(where)[:6]}")
