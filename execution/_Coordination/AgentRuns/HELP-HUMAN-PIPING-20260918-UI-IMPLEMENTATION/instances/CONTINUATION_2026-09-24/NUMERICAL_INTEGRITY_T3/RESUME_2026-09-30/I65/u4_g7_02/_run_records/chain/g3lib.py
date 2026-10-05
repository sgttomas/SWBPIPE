"""I65 U4 G3: shared symbolic arithmetic for the cap evaluation (stdlib only).

An `X` is a linear form over named layout atoms with integer coefficients plus a
constant byte term '1'. Counts are substituted at the D1 caps before any layout
value, so every row evaluates to `sum(coef * atom) + bytes`. Atoms are:
  s(T)            size_of::<T>() in the qualified build (BUILD.md section 3);
  Node(K,V)       max(Leaf_up, Internal_up) of a BTreeMap<K,V> (BUILD.md section 4);
  named residual atoms (for example Text(...)) that another record closes.
`assumed()` substitutes ILLUSTRATIVE 64-bit values. They are not qualified facts; G5
evaluates every atom in the actual build. All arithmetic is exact integer arithmetic.

Container laws (I54 COEFFICIENTS, accepted with RV73):
  V(T,h)  push-built from empty: s(T) * PushCap(h), min nonzero capacity 4
          (premise 1 < s(T) <= 1024; a byte element uses 8).
  A(T,h)  append upper s(T) * max(4, 2h) for h > 0 (a sound upper of PushCap).
  X(T,h)  exact construction/clone s(T) * h.
  Mat(T,r,c) = s(Vec<T>) * r + s(T) * r * c.
  HashReq(K,h): hashbrown at Buckets(h): up(s*b, max(a,8)) + b + 8 <= s*b + b + 8 + 15
          (premise align(K) <= 16, a G5 witness). Moving adds the previous table.
  Tree(K,V,n): T2 node count 1 + floor((n-1)/5) times Node(K,V).
  Text capacity of a formatted String of length L: max(8, 2L) (J9/V3).
  serde_json::to_string of escaped length e: capacity <= max(128, 2e) (J3).
"""


class X(dict):
    def __init__(self, d=None, **kw):
        super().__init__()
        for k, v in (d or {}).items():
            if v:
                self[k] = self.get(k, 0) + v
        for k, v in kw.items():
            if v:
                self[k] = self.get(k, 0) + v

    def __add__(self, o):
        if isinstance(o, int):
            o = X({"1": o})
        r = X(self)
        for k, v in o.items():
            r[k] = r.get(k, 0) + v
            if r[k] == 0:
                del r[k]
        return r
    __radd__ = __add__

    def __mul__(self, c):
        assert isinstance(c, int) and c >= 0
        return X({k: v * c for k, v in self.items()})
    __rmul__ = __mul__

    def ev(self, table):
        tot = 0
        for k, v in self.items():
            if k == "1":
                tot += v
            else:
                if k not in table:
                    raise KeyError("no ASSUMED value for atom " + k)
                tot += v * table[k]
        return tot

    def show(self):
        parts = [f"{v}*{k}" for k, v in sorted(self.items()) if k != "1"]
        if "1" in self:
            parts.append(str(self["1"]))
        return " + ".join(parts) if parts else "0"


def B(n):
    return X({"1": n})


def s(t):
    return X({f"s({t})": 1})


def atom(name):
    return X({name: 1})


ZERO = X()


def total(xs):
    t = X()
    for x in xs:
        t = t + x
    return t


def pushcap(h, minimum=4):
    if h <= 0:
        return 0
    c = 0
    for length in range(1, h + 1):
        if length > c:
            c = max(2 * c, length, minimum)
    return c


def appendcap(h):
    return max(4, 2 * h) if h > 0 else 0


def V(t, h):
    return s(t) * pushcap(h)


def A(t, h):
    return s(t) * appendcap(h)


def Xc(t, h):
    return s(t) * h


def Vbytes(h, elem):
    """Push-built Vec of a primitive element of known byte size."""
    return B(elem * pushcap(h, 8 if elem == 1 else 4))


def Mat(t, r, c):
    """vec![vec![x; c]; r] of a primitive element t (bytes) or a typed atom."""
    if isinstance(t, int):
        return s("Vec") * r + B(t * r * c)
    return s("Vec") * r + s(t) * (r * c)


def buckets(h):
    """hashbrown capacity_to_buckets for h items (NEON group width 8)."""
    if h == 0:
        return 0
    if h < 4:
        return 4
    if h < 8:
        return 8
    adj = (h * 8 + 6) // 7
    b = 1
    while b < adj:
        b *= 2
    return b


def HashReq(k, h):
    """Requested bytes of a hashbrown table holding h items of key/value stride s(k)."""
    b = buckets(h)
    if b == 0:
        return X()
    return s(k) * b + B(b + 8 + 15)


def HashMov(k, h):
    """HashReq plus the previous table that coexists during the last resize."""
    b = buckets(h)
    prev = b // 2 if b > 4 else 0
    return HashReq(k, h) + (s(k) * prev + B(prev + 8 + 15) if prev else X())


def tree_nodes(n):
    return 0 if n <= 0 else 1 + (n - 1) // 5


def Tree(kv, n):
    return X({f"Node({kv})": tree_nodes(n)})


def textcap(length):
    return max(8, 2 * length) if length > 0 else 0


def j3cap(e):
    return max(128, 2 * e)


# Illustrative 64-bit layouts (ASSUMED). The same values as G2 where G2 used one.
ASSUMED = {
    "s(Value)": 32, "Node(String,Value)": 736, "s(String)": 24, "s(Vec)": 24,
    "s(Expansion)": 32, "s(Ratio)": 64, "s(StiffnessContribution)": 24,
    "s(ForceContribution)": 40, "s(BlockWitness)": 120, "s(FunctionalDescriptor)": 128,
    "s(AffineTerm)": 32, "s(QualifiedFunctionalProjection)": 96,
    "s(QualifiedProjection)": 96, "s(RetainedProjection)": 72,
    "s(RetainedFunctionalProjection)": 72, "s(MemberRecovery)": 696,
    "s(SpringAction)": 48, "s(SupportActions)": 128, "s(Option<[f64;3]>)": 32,
    "s((usize,usize,bool))": 24, "s(FrameElement)": 200, "s((usize,f64))": 16,
    "s((&str,usize))": 24, "s((&str,&T))": 24, "s(&str)": 16, "s(Option<usize>)": 16,
    "s(Option<f64>)": 16, "s((&str,usize,u64))": 32, "s((String,usize))": 32,
    "s(ForceTerm)": 48, "s(Option<FormationRecord>)": 64, "s(Vec<usize>)": 24,
}

# G3 additions. Where I54's DWARF observation of the existing artifact gives a node size
# (scratch i54_direct_container_profile/EXISTING_LAYOUT_ROWS.json) the stride is derived
# from it (ResultItem 296 from LeafNode<String,ResultItem> 3536; RowTreatment 104 from
# 1424); every other value is an estimate from the type's field list. ALL ILLUSTRATIVE.
ASSUMED.update({
    "s(MaterialInput)": 224, "s(TemperaturePoint)": 208, "s(PreviewNode)": 96, "s(PreviewPipe)": 384,
    "s(PreviewSupport)": 400, "s(PreviewLoadCase)": 512, "s(PrimitiveLoadInput)": 256,
    "s(Authored<Vec<ExpansionLawInput>>)": 32, "s(FrameNode)": 40, "s(StraightPipeElement)": 256,
    "s(LinearSupport)": 120, "s((String,DerivedSection))": 120, "s(SpringEntry)": 80,
    "s(SupportFinding)": 80, "s(PrimitiveLoad)": 120, "s(NodalLoadContribution)": 64,
    "s((usize,usize,Matrix12))": 1168, "s((usize,usize))": 16, "s((usize,f64,f64))": 24,
    "s(PivotEvidence)": 48, "s(ResidualRow)": 120, "s(ContributionRounding)": 96, "s(Vec<f64>)": 24,
    "s(Vec<Expansion>)": 24, "s(Vec<&ForceTerm>)": 24, "s(usize)": 8, "s(SymmetricMatrixEntry)": 24,
    "s((f64,f64))": 16, "s(LoadFidelityRow)": 160, "s(ExactAccumulator)": 1104, "s(Option<i32>)": 8,
    "s((usize,[f64;3],usize,[f64;3]))": 64, "s(RecordOutcome)": 48, "s(PublishedValue)": 48,
    "s((usize,PublishedValue))": 56, "s([PublishedValue;12])": 576, "s(StationResultants)": 104,
    "s((&str,StressRecoveryResult))": 216, "s(AnalysisStatus)": 24, "s(StressFinding)": 80,
    "s(MemberRecord)": 200, "s(RecoveryRecord)": 152, "s(SupportVector)": 80, "s((String,[f64;6]))": 72,
    "s(QuadraticStressSpan)": 64, "s(Node_SR)": 64, "s(ResultItem)": 296, "s(RowTreatment)": 104,
    "s(ResultBasisRef)": 48, "s((String,String))": 48, "s((String,ResultItem))": 320,
    "Node(String,ResultItem)": 3632, "Node(String,BTreeMap)": 640, "Node(&String,())": 200,
    "s(Diagnostic)": 152, "s(Content)": 32, "s((Content,Content))": 64, "s((usize,(f64,f64)))": 24,
    "s(Projection)": 80, "s(Derived)": 360, "s(FinalizedSourceBlockCase)": 1200,
    "Node(usize,String)": 464, "Node(String,RowTreatment)": 1520, "Node(&str,&ResultItem)": 376,
    "Node(usize,ResultItem)": 3456, "Node(&str,())": 288, "Node(String,())": 376, "Node(usize,())": 200,
})


# ---- G4: cap overrides for the sensitivity table (G4_CAPS='{"m": 16, ...}') -----------------
def g4_caps():
    """The D1 base caps with any G4_CAPS overrides, and every derived count the scripts use."""
    import json as _json, os as _os
    o = _json.loads(_os.environ.get("G4_CAPS", "{}"))
    c = {"n": 32, "m": 32, "g": 32, "s": 32, "r": 192, "l": 192, "ident": 128,
         "raw_values": 16_384, "raw_string_bytes": 65_536, "raw_key_bytes": 65_536, "D": None}
    c.update(o)
    n, m, g, s_, r, l = c["n"], c["m"], c["g"], c["s"], c["r"], c["l"]
    N = 6 * n
    c.update(N=N, F=N, k=min(N, r), C=144 * m + s_, Z=min(N * N, 144 * m + s_), E=78 * m + s_,
             H=N * (N + 1) // 2, P=7 * n + 51 * m + 8 * g + 3, R0=7 * n + 51 * m + g + 3,
             Q=7 * n + 30 * m + s_ + min(N, r) + 2 * g)
    return c
