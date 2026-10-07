#!/usr/bin/env python3
"""RV103: machine-absolute paths in PR #1103's added and modified files.
Usage: abs_scan.py <repo> <M> <H> <harness_dir>
Per changed file: GEN-8 detector lines (surface_roles.iter_machine_path_lines) and
broad-pattern lines, over the whole file at H and over the lines the PR adds.
Path roots are assembled at run time so this file carries no literal machine path."""
import re, subprocess, sys
repo, M, H, hd = sys.argv[1:5]
sys.path.insert(0, hd)
import surface_roles as sr
def git(*a): return subprocess.run(["git","-C",repo,*a],capture_output=True,check=True).stdout
S="/"
ROOTS=[S+"Users"+S+r"[^\s`\"')]+", S+"private"+S+r"(?:tmp|var)[^\s`\"')]*", S+"var"+S+"folders"+S+r"[^\s`\"')]*",
       r"(?<![\w.<])"+S+"tmp"+S+r"[^\s`\"')]*", S+"Volumes"+S+r"[^\s`\"')]+", S+"home"+S+r"[a-z][^\s`\"')]*", r"[A-Z]:\\\\[^\s]+",
       r"~/(?:dev|Library|Documents|Desktop)[^\s`\"')]*"]
pat=re.compile("("+"|".join(ROOTS)+")")
T3="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
print("gen8_whole\tbroad_whole\tgen8_added\tbroad_added\tstatus\tpath\tbroad_samples(added)")
tot=[0,0,0,0]
for row in git("diff","--no-renames","--name-status",M,H).decode().splitlines():
    st,p=row.split("\t",1)
    text=git("show",f"{H}:{p}").decode("utf-8")
    lines=text.split("\n")
    if st=="A": added=set(range(1,len(lines)+1))
    else:
        added=set(); ln=0
        for l in git("diff","-U0",M,H,"--",p).decode().splitlines():
            m=re.match(r"@@ -\S+ \+(\d+)",l)
            if m: ln=int(m.group(1)); continue
            if l.startswith("+") and not l.startswith("+++"): added.add(ln); ln+=1
    g=set(sr.iter_machine_path_lines(text))
    g = {i if isinstance(i,int) else i[0] for i in g}
    b={i+1 for i,l in enumerate(lines) if pat.search(l)}
    ga=len([i for i in g if i in added]); ba=sorted(i for i in b if i in added)
    samp=";".join(f"{i}:{pat.search(lines[i-1]).group(0)[:50].replace(S+'Users'+S, S+'U-sers'+S)}" for i in ba[:3])
    r=[len(g),len(b),ga,len(ba)]; tot=[a+c for a,c in zip(tot,r)]
    if any(r): print(f"{r[0]}\t{r[1]}\t{r[2]}\t{r[3]}\t{st}\t{p.replace(T3,'T3/')}\t{samp}")
print(f"TOTAL\t{tot[0]}\t{tot[1]}\t{tot[2]}\t{tot[3]}")
