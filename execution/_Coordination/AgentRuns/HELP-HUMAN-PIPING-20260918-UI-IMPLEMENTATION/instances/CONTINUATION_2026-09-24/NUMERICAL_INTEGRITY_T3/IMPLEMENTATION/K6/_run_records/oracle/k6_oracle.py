"""K6 C3: an independent oracle for the RCM profile, its half-bandwidth and the
identity-order profile (K6 plan section 4.3, item 3).

Usage: python3 k6_oracle.py <counts.jsonl> <pattern dir>

It is a standard-library port, written from the Rust source, of
sparse_direct's adjacency_from_symmetric_entries, reverse_cuthill_mckee (with
pseudo_peripheral_start, bfs_reachable, bfs_eccentricity and the (degree, index)
tie-break; SD/lib.rs:480-632), of the skyline first-column rule of
SymmetricProfileMatrix::from_entries_with_order (SD/lib.rs:215-275), and of
F1b's identity-order rule (observation_lane_profile, PP@F1b:2966-3002).

Its input is the binary's `--dump-pattern` file: the nonzero lower entries of the
prepared free block (the RCM's input), then the identity lane's entry positions.
It recomputes the counts from those positions only and compares them with the
counts line the binary printed. Standard library only.
"""
import json
import os
import sys
from collections import deque


def read_pattern(path):
    with open(path) as fh:
        lines = fh.read().split('\n')
    assert lines[0] == 'k6-pattern v1', path
    model = lines[1].split(' ', 1)[1]
    free = int(lines[2].split()[1])
    count = int(lines[3].split()[1])
    rcm = [tuple(map(int, l.split())) for l in lines[4:4 + count]]
    at = 4 + count
    lane_dim = int(lines[at].split()[1])
    lane_count = int(lines[at + 1].split()[1])
    lane = [tuple(map(int, l.split())) for l in lines[at + 2:at + 2 + lane_count]]
    return model, free, rcm, lane_dim, lane


def adjacency(dimension, entries):
    adj = [[] for _ in range(dimension)]
    for r, c in entries:
        if r != c:
            adj[r].append(c)
            adj[c].append(r)
    return adj


def rcm(adjacency_lists):
    n = len(adjacency_lists)
    neighbors = [[] for _ in range(n)]
    for node, raw in enumerate(adjacency_lists):
        for other in raw:
            if other != node:
                neighbors[node].append(other)
                neighbors[other].append(node)
    neighbors = [sorted(set(l)) for l in neighbors]
    degrees = [len(l) for l in neighbors]
    neighbors = [sorted(l, key=lambda m: (degrees[m], m)) for l in neighbors]

    def min_by_degree(nodes):
        return min(nodes, key=lambda m: (degrees[m], m)) if nodes else None

    def reachable(seed):
        marked = {seed}
        out = [seed]
        queue = deque([seed])
        while queue:
            node = queue.popleft()
            for nxt in neighbors[node]:
                if nxt not in marked:
                    marked.add(nxt)
                    out.append(nxt)
                    queue.append(nxt)
        return out

    def eccentricity(start):
        marked = {start}
        ecc, current, last = 0, [start], [start]
        while True:
            nxt_level = []
            for node in current:
                for nxt in neighbors[node]:
                    if nxt not in marked:
                        marked.add(nxt)
                        nxt_level.append(nxt)
            if not nxt_level:
                break
            ecc += 1
            last = list(nxt_level)
            current = nxt_level
        return ecc, last

    def start_of(seed):
        candidate = min_by_degree(reachable(seed))
        if candidate is None:
            candidate = seed
        cand_ecc, last = eccentricity(candidate)
        while True:
            nxt = min_by_degree(last)
            if nxt is None:
                break
            nxt_ecc, nxt_last = eccentricity(nxt)
            if nxt_ecc > cand_ecc:
                candidate, cand_ecc, last = nxt, nxt_ecc, nxt_last
            else:
                break
        return candidate

    visited = [False] * n
    order = []
    for seed in range(n):
        if visited[seed]:
            continue
        start = start_of(seed)
        visited[start] = True
        queue = deque([start])
        while queue:
            node = queue.popleft()
            order.append(node)
            for nxt in neighbors[node]:
                if not visited[nxt]:
                    visited[nxt] = True
                    queue.append(nxt)
    order.reverse()
    return order


def skyline(dimension, entries, order):
    position = [0] * dimension
    for k, original in enumerate(order):
        position[original] = k
    first = list(range(dimension))
    for r, c in entries:
        a, b = position[r], position[c]
        hi, lo = (a, b) if a >= b else (b, a)
        if lo < first[hi]:
            first[hi] = lo
    profile = sum(row - f + 1 for row, f in enumerate(first))
    bandwidth = max((row - f for row, f in enumerate(first)), default=0)
    return profile, bandwidth


def identity(dimension, entries):
    first = list(range(dimension))
    for r, c in entries:
        hi, lo = (r, c) if r >= c else (c, r)
        if lo < first[hi]:
            first[hi] = lo
    return sum(row - f + 1 for row, f in enumerate(first)), max((row - f for row, f in enumerate(first)), default=0)


def main():
    counts_path, pattern_dir = sys.argv[1], sys.argv[2]
    counts = {}
    with open(counts_path) as fh:
        for line in fh:
            d = json.loads(line)
            if d.get('kind') == 'counts':
                counts[d['model']] = d
    failures = 0
    print('model free_lower_nonzero rcm_profile rcm_half_bandwidth identity_profile identity_half_bandwidth lane_entries verdict')
    for model_id in counts:
        path = os.path.join(pattern_dir, model_id.replace(':', '_') + '.pattern')
        model, free, entries, lane_dim, lane = read_pattern(path)
        assert model == model_id
        order = rcm(adjacency(free, entries))
        profile, bandwidth = skyline(free, entries, order)
        id_profile, id_bandwidth = identity(lane_dim, lane)
        c = counts[model_id]
        ours = (len(entries), profile, bandwidth, id_profile, id_bandwidth, len(lane))
        theirs = (c['free_lower_nonzero'], c['rcm_profile_entries'], c['rcm_half_bandwidth'],
                  c['identity_profile_entries'], c['identity_half_bandwidth'], c['lane_entries'])
        ok = ours == theirs
        failures += not ok
        print(model_id, *ours, 'EQUAL' if ok else 'DIFFERS binary=%s' % (theirs,))
    print('summary: %d of %d equal' % (len(counts) - failures, len(counts)))
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
