import json, sys, collections, re, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.py')).read().split("roots = [")[0].replace("I65, SRC = sys.argv[1], sys.argv[2]", "I65, SRC = sys.argv[1], sys.argv[2]"))
d = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.out.json')))
big = set([x for x in d['new_cyclic'] if len(x) == 50][0])
start = [k for k in big if k.endswith(':step')][0]
# BFS shortest cycle back to start within big
prev = {start: None}; q = collections.deque([start]); found = None
while q and not found:
    v = q.popleft()
    for w in sorted(edges[v]):
        if w not in big: continue
        if w == start: found = v; break
        if w not in prev: prev[w] = v; q.append(w)
path = [start]; v = found; chain = []
while v is not None: chain.append(v); v = prev[v]
chain = chain[::-1]
print(" -> ".join(k.rsplit('/', 1)[1] for k in chain + [start]))
