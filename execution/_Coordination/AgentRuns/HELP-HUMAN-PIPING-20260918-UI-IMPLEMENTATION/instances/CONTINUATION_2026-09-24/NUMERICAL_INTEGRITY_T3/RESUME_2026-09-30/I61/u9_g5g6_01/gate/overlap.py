#!/usr/bin/env python3
"""Per-run overlap of the part-2 timing runs with foreign jobs: the watcher's 5 s samples against each run's interval (end - wall_seconds, end)."""
import re,sys,datetime as dt
d=sys.argv[1]
samples=[]
for line in open(d+"/host_watch.tsv"):
    parts=line.rstrip("\n").replace("\\t","\t").split("\t")
    t=dt.datetime.strptime(parts[0],"%Y-%m-%dT%H:%M:%SZ"); samples.append((t,int(parts[1].strip() or 0)))
print("run\tstart\tend\twall_s\tsamples\tforeign_samples")
for line in open(d+"/driver.log"):
    m=re.match(r"(\S+) (base|cand) (\S+) ok ([\d.]+)",line)
    if m:
        end=dt.datetime.strptime(m.group(1),"%Y-%m-%dT%H:%M:%SZ"); w=float(m.group(4)); st=end-dt.timedelta(seconds=w)
        inside=[s for s in samples if st<=s[0]<=end]
        print(f"{m.group(2)} {m.group(3)}\t{st:%H:%M:%S}\t{end:%H:%M:%S}\t{w}\t{len(inside)}\t{sum(1 for s in inside if s[1]>0)}")
