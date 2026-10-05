import sys, re, subprocess
from pathlib import Path
sys.path.insert(0, sys.argv[1])
import surface_roles as sr
git_dir, commit, listfile, t3rel = sys.argv[2:6]
S = '/'  # path roots are assembled so this file carries no literal machine path
ROOTS = [S+'Users'+S+r'[^\s`"\')]+', S+'private'+S+r'(?:tmp|var)[^\s`"\')]*', S+'var'+S+'folders'+S+r'[^\s`"\')]*',
         r'(?<![\w.<])'+S+'tmp'+S+r'[^\s`"\')]*', S+'Volumes'+S+r'[^\s`"\')]+', S+'home'+S+r'[a-z][^\s`"\')]*', r'[A-Z]:\\\\[^\s]+']
pat = re.compile('(' + '|'.join(ROOTS) + ')')
for n in open(listfile).read().split("\n"):
    if not n: continue
    data = subprocess.run(["git","-C",git_dir,"show",f"{commit}:{n}"],capture_output=True,check=True).stdout
    try: text = data.decode("utf-8")
    except UnicodeDecodeError: print("BINARY", n); continue
    sr_hits = list(sr.iter_machine_path_lines(text))
    rx = [ (i+1, m.group(0)) for i,l in enumerate(text.split("\n")) for m in pat.finditer(l)]
    rxlines = sorted(set(i for i,_ in rx))
    short = n.replace(t3rel, "T3")
    print(f"{len(sr_hits)}\t{len(rxlines)}\t{short}\t" + (";".join(f"{i}:{s[:60]}" for i,s in rx[:3])))
