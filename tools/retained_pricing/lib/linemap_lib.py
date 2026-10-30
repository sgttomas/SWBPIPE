"""O-10: one line map for the TEXT chain's keys, shared by the rule tables' carry (g7_linemap) and SQ2's decision
tables (sq2_rules.py). Stdlib only; Git reads only (GIT_OPTIONAL_LOCKS=0).

A key is `file:line` (a site) or `file:line:fn` (a function), the file short (`lib.rs`, `structural/sparse.rs`) or
full (`core/.../src/x.rs`). It maps from OLD to NEW in this order:
1. a file unchanged between OLD and NEW: the key as written;
2. a line outside every changed hunk: the cumulative offset, checked (the fn still defined there, or the same text);
3. otherwise (a line inside a changed hunk, or a failed check):
   - a fn key by function: the same ordinal definition of `fn name` in the file when OLD and NEW define it the
     same number of times. When the OLD definition lies in a changed hunk and NEW defines `fn name` no more in that
     file, the key is reported "deleted" (its function is gone). When NEW defines it fewer times but still at least
     once, the key is "fewer_definitions": which definition went cannot be read from the count, so it is unmapped (the
     run stops) unless `resolutions` names it: {"as": "deleted", "why": ...} (its own definition is the one gone) or
     {"as": <n>, "why": ...} (it is NEW's n-th definition, 1-based). Anything else is "replaced";
   - a site key by text: the OLD line's text (stripped) found exactly once in the hunk's NEW range, else exactly once in
     the NEW file; none is "deleted" when the hunk removed the line with nothing in its place, else "replaced". A
     "replaced" site whose enclosing OLD function NEW no longer defines in that file is "deleted" only when
     `resolutions` names it with that function ({"as": "deleted", "fn": name, "why": ...}; B7b).
A key never maps silently: every key that is not left as written is reported with how it was mapped.
"""
import gzip, json, os, re, subprocess
from source_history import resolve_revision, project_prefix
P = "projects/chirality-piping/"


def load_rows(paths):
    """The (file, line) pairs of TEXT rows: each path a TEXT run's text_budget json (or .json.gz; `rows` of dicts with
    `file` and `line`) or the tool's data/basis_text_rows.json (`rows` of [file, line])."""
    rows = set()
    for rp in paths:
        data = json.load(gzip.open(rp) if rp.endswith(".gz") else open(rp))
        for r in data["rows"]:
            f, ln = (r["file"], r["line"]) if isinstance(r, dict) else r
            rows.add((f.split(P)[-1], ln))
    return rows


def load_resolutions(path):
    """The reviewed resolutions: {key: {"as": "deleted" | n, "why": ...}} for "fewer_definitions" fn keys, and
    {key: {"as": "deleted", "fn": name, "why": ...}} for a site key inside a function deleted from its file (B7b)."""
    if not path:
        return {}
    keys = json.load(open(path)).get("keys", {})
    for k, v in keys.items():
        if not (v.get("as") == "deleted" or (isinstance(v.get("as"), int) and v["as"] >= 1)) or not v.get("why"):
            raise SystemExit(f"resolution for {k}: 'as' must be \"deleted\" or a definition number >= 1, with a 'why'")
        if "fn" in v and (v.get("as") != "deleted" or not isinstance(v["fn"], str) or not v["fn"]):
            raise SystemExit(f"resolution for {k}: a site resolution names its deleted enclosing 'fn' with 'as': \"deleted\"")
    return keys


class Mapper:
    def __init__(self, repo, old, new, crates, rows=None, resolutions=None):
        old, new = resolve_revision(repo, old), resolve_revision(repo, new)
        self.repo, self.old, self.new, self.crates = repo, old, new, crates
        self.prefix = P = project_prefix(repo, old)
        if project_prefix(repo, new) != P:
            raise ValueError("Mixed project layouts in one pricing comparison")
        self.env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
        changed = [f[len(P):] for f in self.git("diff", "--name-only", old, new, "--", P + "core").split("\n") if f.endswith(".rs")]
        self.changed = [f for f in changed if "/tests/" not in f and not f.endswith("_tests.rs") and any(f.startswith(c + "/") for c in crates)]
        self.hunks, self._old, self._new, self._len = {}, {}, {}, {}
        for f in self.changed:
            hs = []
            for line in self.git("diff", "-U0", old, new, "--", P + f).split("\n"):
                m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
                if m:
                    hs.append((int(m.group(1)), int(m.group(2) or 1), int(m.group(3)), int(m.group(4) or 1)))
            self.hunks[f] = hs
        self.all_old = [f[len(P):] for c in crates for f in self.git("ls-tree", "-r", "--name-only", old, "--", P + c).split("\n") if f.endswith(".rs")]
        self.rows = rows or set()
        self.resolutions = dict(resolutions or {})      # key (as written) -> {"as": "deleted" | n, "why": ...}
        self.resolutions_used = []
        self.report, self.unmapped, self.resolved = [], [], []

    def git(self, *a):
        return subprocess.run(["git", "-C", self.repo, *a], capture_output=True, text=True, env=self.env, check=True).stdout

    def text(self, rev, f):
        cache = self._old if rev == self.old else self._new
        if f not in cache:
            try:
                cache[f] = self.git("show", f"{rev}:{self.prefix}{f}").split("\n")
            except subprocess.CalledProcessError:
                cache[f] = []
        return cache[f]

    def offset(self, f, ln):
        off = 0
        for o, oc, n, nc in self.hunks[f]:
            start = o if oc > 0 else o + 1
            if oc > 0 and o <= ln < o + oc:
                return None, (o, oc, n, nc)
            if ln >= start:
                off += nc - oc
        return ln + off, None

    def files(self, path):
        """The OLD files a key's path can name: a full path, or every chain-crate file with that tail."""
        if path.startswith("core/"):
            return [path] if path in self.all_old else []
        return [f for f in self.all_old if f.endswith("/" + path)]

    @staticmethod
    def defs(lines, name):
        return [i + 1 for i, l in enumerate(lines) if re.search(r"\bfn\s+" + re.escape(name) + r"\b", l)]

    @staticmethod
    def enclosing(lines, ln):
        """The name of the nearest `fn` definition at or above line ln (None if none)."""
        for i in range(min(ln, len(lines)) - 1, -1, -1):
            m = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?fn\s+([A-Za-z_][A-Za-z_0-9]*)", lines[i])
            if m:
                return m.group(1)
        return None

    def pick(self, path, ln, name, ctx, whole):
        """The one OLD file the key names, or None (reported)."""
        cands = self.files(path)
        if name:
            cands = [f for f in cands if ln <= len(self.text(self.old, f)) and re.search(r"\bfn\s+" + re.escape(name) + r"\b", self.text(self.old, f)[ln - 1])]
        if len(cands) > 1 and all(f not in self.hunks for f in cands):
            return ""          # every file the short key can name is unchanged: the key holds as written for each
        if len(cands) > 1:
            byrow = [f for f in cands if (f, ln) in self.rows]
            bylen = [f for f in cands if ln <= len(self.text(self.old, f))]
            how, c2 = ("the one file with a TEXT row there at the basis", byrow) if byrow else ("the one file with that line", bylen)
            if len(c2) != 1:
                self.unmapped.append([ctx, whole, "ambiguous: " + ", ".join(cands)])
                return None
            self.resolved.append([ctx, whole, c2[0], how])
            cands = c2
        if not cands:
            return ""          # names no chain file at OLD (another crate's same basename, or a citation): as written
        return cands[0]

    def map_key(self, whole, ctx=""):
        """The NEW spelling of `whole`, or `whole` itself when unmapped (then reported in self.unmapped)."""
        m = re.fullmatch(r"((?:core/[a-z_/]+/src/)?(?:[a-z_0-9]+/)*[a-z_0-9]+\.rs):(\d+)(?::([A-Za-z_][A-Za-z_0-9]*))?", whole)
        if not m:
            return whole
        path, ln, name = m.group(1), int(m.group(2)), m.group(3)
        f = self.pick(path, ln, name, ctx, whole)
        if not f or f not in self.hunks:
            return whole                                   # unresolved (reported) or an unchanged file
        old, new = self.text(self.old, f), self.text(self.new, f)
        nl, hunk = self.offset(f, ln)
        how = "offset"
        if nl is not None:
            ok = (re.search(r"\bfn\s+" + re.escape(name) + r"\b", new[nl - 1]) if name else old[ln - 1].strip() == new[nl - 1].strip()) if nl <= len(new) else False
            if not ok:
                nl, how = None, "offset check failed"
        if nl is None:
            if name:
                od, nd = self.defs(old, name), self.defs(new, name)
                if len(od) == len(nd) and ln in od:
                    nl, how = nd[od.index(ln)], f"by function: definition {od.index(ln) + 1} of {len(od)} of fn {name}"
                else:
                    res = self.resolutions.get(whole) if (hunk is not None and 0 < len(nd) < len(od)) else None
                    if res is not None and isinstance(res.get("as"), int) and 1 <= res["as"] <= len(nd):
                        nl, how = nd[res["as"] - 1], f"resolved: definition {res['as']} of {len(nd)} of fn {name} ({res['why']})"
                        self.resolutions_used.append([ctx, whole, res])
                    else:
                        if hunk is not None and not nd:
                            kind = "deleted"
                        elif hunk is not None and len(nd) < len(od):
                            kind = "fewer_definitions"
                            if res is not None and res.get("as") == "deleted":
                                kind = "deleted"
                                self.resolutions_used.append([ctx, whole, res])
                        else:
                            kind = "replaced"
                        self.unmapped.append([ctx, whole, f"fn {name}: {len(od)} definitions at OLD, {len(nd)} at NEW", kind])
                        return whole
            else:
                t = old[ln - 1].strip() if ln <= len(old) else None
                rng = range(hunk[2], hunk[2] + hunk[3]) if hunk else range(0)
                inh = [i for i in rng if 0 < i <= len(new) and new[i - 1].strip() == t]
                inf = [i + 1 for i, l in enumerate(new) if l.strip() == t]
                if t and len(inh) == 1:
                    nl, how = inh[0], "by text in the hunk"
                elif t and len(inf) == 1:
                    nl, how = inf[0], "by text in the file"
                else:
                    kind = "deleted" if (not new or (hunk and hunk[3] == 0)) else "replaced"
                    # B7b: a site inside a function the revision deleted from this file is
                    # pruned only by a reviewed resolution naming that function; the
                    # enclosing OLD fn must be named in the entry and absent at NEW.
                    res = self.resolutions.get(whole) if (kind == "replaced" and hunk is not None) else None
                    if res is not None and res.get("as") == "deleted" and res.get("fn"):
                        owner = self.enclosing(old, ln)
                        if owner == res["fn"] and not self.defs(new, owner):
                            kind = "deleted"
                            self.resolutions_used.append([ctx, whole, res])
                    self.unmapped.append([ctx, whole, f"line {ln} of {f}: text {'not found' if not inf else 'not unique'} at NEW", kind])
                    return whole
        out = whole.replace(f":{ln}", f":{nl}", 1)
        if out != whole or how != "offset":
            self.report.append([ctx, whole, out, how])
        return out

    def summary(self):
        return {"old": self.old, "new": self.new, "changed_production_rs": self.changed, "hunks": {f: len(h) for f, h in self.hunks.items()},
                "remapped": len(self.report), "unmapped": self.unmapped, "resolved_ambiguous": self.resolved,
                "resolutions_used": self.resolutions_used}
