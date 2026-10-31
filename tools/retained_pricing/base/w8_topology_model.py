"""W-8 topology warnings D3 (DEL-04-06 Design model-topology-checks.md; retained-pricing-rules.md §8.16): the positive
owners W-8 adds outside the chain's TEXT inventory.

W8-TW: `topology::append_topology_diagnostics` runs once per ordinary run, in run_linear_static_preview_observed_inner
after the build and before the boundary, on both routes. Its working set is transient (it drops before the boundary is
prepared); it is charged in full, every owner live at once, with each grown owner's previous backing beside it.
W8-ND: validation's `validate_components` builds a node-degree map over the pipe segments, transient within the call.

The owners are bound to the sha256 of topology.rs (the whole module is W-8's), of the two validation.rs bodies, and to
their exact statements; lib.rs is under the existing full-file pins (b7a_domain_model, w1_reductions_model,
kd5_text_model). A snapshot without topology.rs adds nothing, so earlier registered trees keep their coefficients; a
partly present or changed owner stops the run."""
import hashlib
import os
import re

from g3lib import ASSUMED, B, X, Xc, s, pushcap, buckets, HashMov

TOP = "core/product_physics/src/topology.rs"
VAL = "core/product_physics/src/validation.rs"
LIB = "core/product_physics/src/lib.rs"
FG = "core/product_physics/src/formation_guard.rs"

# Read at 91f59b7a56 (§8.18: the absolute node-proximity tolerance read from model_settings, cells 8 tolerances wide
# indexed with no offset and saturating, no bounding box; scaled norms, the pair lists bounded to NAMED samples with
# counts and a per-node `found` buffer as at 97e6fda00f; validation skips a component without a node). Re-read for
# the branch-warning statement (§8.18, W8-BF): validate_components' warning keeps its topology text only (the applied
# sides are stated after publication by state_applied_branch_factors); its node-degree map (W8-ND) is unchanged.
TOP_PIN = "cca9abee30dd68b4fe7d47785d92f4805a921d74742c64687cd7e4542f8be0f9"
# W-15 D3 (§8.21): validate_components' rigid checks are rigid_component::append_scaling_diagnostics (one call per
# rigid-family component, 0 on D1 by P2) and one append_mapping_conflicts after the loop (its member map is W15-PM,
# w15_owners_model); the node-degree map (W8-ND) and the branch check are unchanged.
BODY_PINS = {
    # §8.33 (main 7c457cf0b/6b34c4559, the bend SIF warning fix, carried with RT-A3 increment 1): the bend
    # modifier warning picks one of two static texts by bend_scalar_sif_on_realized_arc, inside the component loop (0 on
    # D1, D1.4); the node-degree map (W8-ND) is unchanged.
    (VAL, "validate_components"): "47c36c4e2006118a867d3ed9f608507aabc21d418eb9c65412a76d0d972c7471",
    (VAL, "branch_topology_invalid"): "22c8a42d2750e8145c94e009bd9c6fe7f3a5965926aae487a5948b6db34206ae",
}
NAMED = 6

# Each allocating statement the formulas price, once in its file.
STATEMENTS = (
    (TOP, ".map(|n| n.coordinates)\n        .collect::<Vec<_>>();"),
    (TOP, "let edges = element_edges(built);"),
    (TOP, ".map(|r| r.span_id.as_str())\n        .collect::<HashSet<_>>();"),
    (TOP, "let bodies = Bodies::new(coordinates, edges);"),
    (TOP, "let mut parts: BTreeMap<usize, (usize, usize)> = BTreeMap::new();"),
    (TOP, "let mut first_of_body: HashMap<usize, usize> = HashMap::new();"),
    (TOP, "let mut with_element = HashSet::new();"),
    (TOP, ".filter_map(|node| bodies.body_of_node(node))\n        .collect::<HashSet<_>>();"),
    (TOP, ".map(|&(a, b)| (a.min(b), a.max(b)))\n        .collect::<HashSet<_>>();"),
    (TOP, "let mut coincident = Vec::new();"),
    (TOP, "let mut near = Vec::new();"),
    (TOP, "let mut grid: HashMap<[i64; 3], Vec<usize>> = HashMap::new();"),
    (TOP, "grid.entry(cell(point)).or_default().push(node);"),
    (TOP, "let mut found = Vec::new();"),
    (TOP, "found.clear();"),
    (TOP, "found.push((other, None));"),
    (TOP, "found.push((other, Some(distance)));"),
    (TOP, "if coincident.len() < NAMED {\n                        coincident.push((node, other));"),
    (TOP, "if near.len() < NAMED {\n                        near.push((node, other, distance));"),
    (TOP, "let mut refs = Vec::new();"),
    (VAL, "let mut node_degree = HashMap::<&str, usize>::new();"),
    (VAL, "*node_degree.entry(pipe.from.as_str()).or_default() += 1;"),
    (VAL, "*node_degree.entry(pipe.to.as_str()).or_default() += 1;"),
    (LIB, "    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);\n"),
    (FG, "pub(crate) const NAMED: usize = 6;"),
)
# Inventory of topology.rs's owning constructs (outside comments): a new one needs a price. The nine pushes are the
# grid cell, the found buffer's two, the two sample lists, the refs list and the three diagnostics (the envelope's Vec,
# priced by D).
TOP_INVENTORY = {"collect": 9, "Vec::new()": 4, "HashMap::new()": 2, "HashSet::new()": 1, "BTreeMap::new()": 1,
                 "Bodies::new(": 1, ".push(": 9, "vec!": 0, "Vec::with_capacity": 0, ".to_vec()": 0, "Box::new": 0}


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if not hits:
        return None
    if len(hits) != 1:
        raise SystemExit(f"W8 owners: {name} is not one definition; re-review the owner it prices")
    indent = hits[0].group(1)
    end = text.find("\n" + indent + "}\n", hits[0].start())
    if end < 0:
        raise SystemExit(f"W8 owners: no end of fn {name}")
    return text[hits[0].start():end + len(indent) + 3]


def digests(snapshot):
    """The current digests (for review and re-pinning; not a pass condition)."""
    top = _read(snapshot, TOP)
    out = {TOP: hashlib.sha256(top.encode()).hexdigest() if top is not None else None}
    for (rel, name) in BODY_PINS:
        text = _read(snapshot, rel)
        body = _body(text, name) if text is not None else None
        out[f"{rel}:{name}"] = hashlib.sha256(body.encode()).hexdigest() if body else None
    return out


def inspect(snapshot):
    """True when W-8's topology checks are present with their reviewed digests; False on an earlier revision (no
    topology.rs and no node-degree map); anything else stops."""
    top = _read(snapshot, TOP)
    val = _read(snapshot, VAL) or ""
    lib = _read(snapshot, LIB) or ""
    if top is None:
        if "node_degree" in val or "topology::" in lib:
            raise SystemExit("W8 owners: the topology checks are partly present; re-review the owners they price")
        return False
    current = digests(snapshot)
    if current[TOP] != TOP_PIN:
        raise SystemExit("W8 owners: topology.rs changed; re-review the owners it prices")
    for (rel, name), pin in BODY_PINS.items():
        if current[f"{rel}:{name}"] != pin:
            raise SystemExit(f"W8 owners: {rel} fn {name} changed or is missing; re-review the owner it prices")
    for rel, needle in STATEMENTS:
        text = _read(snapshot, rel)
        if text is None or text.count(needle) != 1:
            raise SystemExit(f"W8 owners: {needle!r} is not one site in {rel}; re-review the owner it prices")
    code = re.sub(r"//[^\n]*", "", top)
    found = {k: code.count(k) for k in TOP_INVENTORY}
    found["collect"] = len(re.findall(r"\.collect\b", code))
    if found != TOP_INVENTORY:
        raise SystemExit(f"W8 owners: topology.rs's owning constructs changed; re-review: {found}")
    if lib.count("topology::append_topology_diagnostics(") != 1:
        raise SystemExit("W8 owners: append_topology_diagnostics' callers changed; re-review its once-per-run charge")
    # The chain's illustrative strides for the two registered atoms these owners add to the ordinary rows (the
    # in-build collector supplies both at G5; this is only the chain's own check).
    ASSUMED.setdefault("s(f64)", 8)
    ASSUMED.setdefault("s(u64)", 8)
    return True


def _grown(stride, h):
    """A push-grown Vec of h elements of `stride`, with its previous backing beside it (the last doubling)."""
    cap = pushcap(h)
    return stride * (cap + cap // 2)


def _grid(n):
    """HashMap<[i64; 3], Vec<usize>> of <= n cells (one per node at most), grown by insertion, with its previous table:
    each slot a [i64; 3] key (3 * s(u64)) and a Vec<usize> header."""
    stride = s("u64") * 3 + s("Vec<usize>")
    b = buckets(n)
    if b == 0:
        return X()
    prev = b // 2 if b > 4 else 0
    return stride * (b + prev) + B(b + 8 + 15) + (B(prev + 8 + 15) if prev else X())


def topology_workspace(n, m, g):
    """W8-TW (per ordinary run): every owner of append_topology_diagnostics live at once, at the caps.
    - coordinates: n [f64; 3] (an exact-size collect);
    - element edges: <= m (usize, usize), push-grown (straight members; no curved span or connector on D1, P2/P3; the
      replaced-span set is empty there, P3, and is charged as a table of m &str anyway);
    - a second Bodies: parent and body_of_node (n usize each), the root map (<= n entries, grown), the two box arrays
      (<= n [f64; 3] each) and the extents (<= n f64);
    - the part map: BTreeMap<usize, (usize, usize)> of <= m parts (each part holds an element and parts are disjoint),
      each node <= one internal node, 11 keys, 11 values, 12 edges, parent pointer and two u16 (= 11*(8+16)+13*8+8 B),
      and a node holds >= 1 entry, so <= m nodes;
    - first_of_body (<= m (usize, usize)), with_element (<= m usize), grounded (<= g supports usize), joined (<= m
      (usize, usize)): hash tables grown by insertion, each with its previous table;
    - the cell grid (<= n cells) and its cells' Vec<usize> (sum of capacities <= 4n: pushcap(k) <= 4k), with one
      previous cell backing (<= pushcap(n) / 2);
    - the per-node `found` buffer of (usize, Option<f64>): cleared per node (capacity kept), it holds that node's later
      neighbours within the 27 cells, each other node at most once (`other > node`, each node in one cell): <= n - 1,
      push-grown with the previous backing;
    - the pair samples: coincident (usize, usize) and near (usize, usize, f64), each pushed only while shorter than
      NAMED (<= 6, capacity 8), with the previous backing; the counts are integers;
    - three Vec<String> of <= NAMED headers (the description and the two pair lists). Their texts are TEXT sites, and
      the affected-refs Vecs move into the diagnostics (W8-RF in the TEXT model)."""
    usize, f64 = s("usize"), s("f64")
    coordinates = Xc("f64", 3 * n)
    edges = _grown(s("(usize,usize)"), m) + HashMov("&str", m)
    bodies = Xc("usize", 2 * n) + HashMov("(usize,usize)", n) + Xc("f64", 2 * 3 * n) + Xc("f64", n)
    node = usize * 11 + s("(usize,usize)") * 11 + usize * 13 + B(8)
    parts = node * m + HashMov("(usize,usize)", m) + HashMov("usize", m) + HashMov("usize", g)
    joined = HashMov("(usize,usize)", m)
    grid = _grid(n) + Xc("usize", 4 * n) + usize * (pushcap(n) // 2)
    found = _grown(usize + s("Option<f64>"), max(n - 1, 0))
    pair_lists = found + _grown(s("(usize,usize)"), NAMED) + _grown(usize * 2 + f64, NAMED)
    named = Xc("String", 3 * NAMED)
    return coordinates + edges + bodies + parts + joined + grid + pair_lists + named


def node_degrees(m):
    """W8-ND (per validation call): validate_components' HashMap<&str, usize> of the pipe segments' end nodes, <= 2m
    distinct ids (two ends per segment, D1.9 m <= 32), grown by insertion, with its previous table."""
    return HashMov("(&str,usize)", 2 * m)
