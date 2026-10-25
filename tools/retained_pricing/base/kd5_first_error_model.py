"""KD5C6: source-bound first-return allocation limits; no visit/edge limits."""
import collections
import hashlib
import os
import re
import runpy

HELPERS = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
CPP = "core/product_physics/src/certified_publication.rs"
LIB = "core/product_physics/src/lib.rs"
# T4-U6 (D-5): fallback_circular_maximum takes (state, basis) and binds that basis's area (two added comment lines
# move its sites at offsets 25/142/162 to 27/144/164); straight_stress_spans and the two pressure helpers read the
# pressure bore from ExactSectionBases. No allocation is added or removed in these bodies, and every first-return
# alternative still returns (retained-pricing-rules.md, "T4-U6/U8: actual-source D3").
# PP: straight_stress_spans takes the case's formation source and binds its member loads to it (a borrow).
# Epoch addbdc030e: solve_prepared_load_case's `pipe_sections` is `exact_pipe_sections(built)` (no first-return change).
# T4-S5 D3 (epoch): certified_publication.rs adds only connector_record's tie-rod groups (connectors only: P3); every
# pinned private helper, root and C6 site is unchanged. solve_prepared_load_case adds the U4b station rows, realized-arc
# maxima (arcs only) and the corrected displacement; its consumption of the fallback attempts is unchanged.
# O4 S5 (retained-pricing-rules.md §8.9, O4-SR): solve_prepared_load_case's selected-source maximum takes one
# composite_member_maximum call per distinct basis (no fallback attempt) and drops SOURCE_MAXIMUM_BASES_DISTINCT; its
# consumption of the fallback attempts and every first-return alternative are unchanged.
CPP_FILE_PIN = '8d0fd1f1dba13e8eca74aa62130a0956d3c1ca5c1d197749cf58ee00c501184a'
CPP_PREFIX_PIN = '41f4e67c56477e24ace4746e058be2c5863f2a53733d338e411850edaf660348'
PINS = {'checked': 'f6462ce3e9b91123fc8c424e4e932ca6ae2e06773c3c8ea4ca9a233ea0060e97', 'exact': '1722765731354ee41c0650743cbe5e3f6bd9bbe80d8bf67fb5e3e2edb7db79ac', 'rounded': '50e699a9e3ca58e8aae1425c397fc84b2bae7596fcaa665e749ba941fb21053a', 'accumulate': 'fa801055fcc8cdba9fa63b8c9e1a43d13ab6a4b531140fd05cc86636c78d47b3', 'add': '6cf32ee08ae1324229b3332932b16be33f710e3db38f38799285b70a170b555d', 'mul': 'b7a13d3aa95f1a81250596c2894aa53c08261ef50678482907ca925eb89fd375', 'divided': '42236c96706c6fd58796004eca2d9791bd37e011f0e77c1ee482673d74a501d8', 'exact_scaled': 'ce309e689bd86000332875400f5c116223ac722bc7a98a561aee67175b3d54d9', 'divided_scaled': 'aaeb7baff5bbc828cb9919c77c789e02c6c3b0d7292cfc2f24e395862231160f', 'min': '5f9dbece75030436c8445ea973d65501847f6a89f2f1aa716280eaf2c84995f0', 'max': '2b09f8df493dc541baa172795f41a10196b6e7e782333679916ed4531779de79', 'active_length': '31f1a47315b78ee62cf5e53ad68b1bdf7399a11bdab0a4df47f0550002786c95', 'fallback_section_errors': '77d7713b9a110474f69c9c98522feaedb5a615db9f5334d4db3541108a6696c9', 'fallback_circular_maximum': 'bf5ab4f4c3577ffb0b068e1a9b918793e22b04a8c68764610cec3fe6e648210c', 'straight_stress_spans': '8fa6015c0c0b6ae5c87f241c29b1187f6e3e4a3bc3e624969d37aff1c3164efe', 'directed_transport_sum': '6754f267ec85e4045fe19eabccac12312648039bb49a4c0e237639272ca7c5d8', 'maximum_cover': 'e6b1a37db23c2da925e0b069eb3e3d3e85d0bd4c6ee6f413cfbdffd4d1ef5959', 'public_mpa_magnitude_in_pa_upper': '8daef91ad0c895e81b7ce00a72f2e098a665b4b696bb682472d7497aa827397f', 'maximum_summary_mpa': '2db64af82e567c32dd219848e7ae090b698c6a2426df3f02a4d04540dbd51a8c', 'circular_maximum': 'ec77f771a0d86d9cb4a7eb98626cbba3e5f77e8359e8e8c4bbe3a018341cea0a', 'l1_summary': '669870df23a039c3d65e5f51c721e101935bc1e1bee5218c8d22ab3d2864473e', 'summary_row': '82a68500450979089472fbc47df8ceebb7bfcc4b94b8eef16647b2501d2ebfe2', 'stress_mpa': '22ff193d88839b0e2f4ff6d60e445eafddbf3cd63af9b748037941cf9c6a99c9'}
# Pre-0.4 failure unification D3 (retained-pricing-rules.md §8.14): solve_prepared_load_case screens selection by the
# captured replay's reservation and declines a withheld join on both exact routes (0.4's withheld text, now reachable
# on route E: PU-TX), keeps a selected case's first support-norm refusal as its finalization cause (PU-TX) and records
# the S2/S3 cause on both routes. No record constructor, label, route string, carrier, staging, fallback attempt or
# first-return alternative changes.
# W-7 D3 (retained-pricing-rules.md §8.20, W7-GE): solve_prepared_load_case appends each member's twelve global end
# rows or one blocking ELEMENT_FORCE_RECOVERY_FAILED diagnostic after its published rows; its consumption of the
# fallback attempts and every first-return alternative are unchanged.
LIB_PINS = {'solve_load_case_observed': '5a70782515a2e66790c8d570cc13d49ead9d22614e0f99b288d2632e8fc4e061', 'solve_prepared_load_case': '533f046490187db7edca7dd19177046a893a48cd2f6eba5334c58d42608be820', 'exact_straight_summary_extrema': '1c15fd171ea49543bb3c7372e93a79545dfb63d5f877a5c232b423ec4ebd4ce0'}
PRIVATE = ("checked", "exact", "rounded", "accumulate", "add", "mul", "divided", "exact_scaled", "divided_scaled", "min", "max", "active_length")
ENTRY = ("summary_row", "stress_mpa", "maximum_summary_mpa", "fallback_section_errors", "fallback_circular_maximum")
PRIVATE_ROOTS = ("fallback_circular_maximum", "summary_row", "stress_mpa", "maximum_summary_mpa")
TRANSPORT_ROOTS = ("fallback_circular_maximum", "straight_stress_spans")
# Offsets are from each freshly source-resolved function header, in pinned bodies.
SITE_DEFS = (
    ("maximum_cover", 7, "format", ("maximum_cover",)),
    ("maximum_cover", 18, "format", ("maximum_cover",)),
    ("public_mpa_magnitude_in_pa_upper", 4, "to_string", ("maximum_cover",)),
    ("public_mpa_magnitude_in_pa_upper", 7, "to_string", ("maximum_cover",)),
    ("public_mpa_magnitude_in_pa_upper", 10, "to_string", ("maximum_cover",)),
    ("directed_transport_sum", 2, "into_text", TRANSPORT_ROOTS),
    ("directed_transport_sum", 6, "to_string", TRANSPORT_ROOTS),
    ("directed_transport_sum", 8, "to_string", TRANSPORT_ROOTS),
    ("directed_transport_sum", 11, "to_string", TRANSPORT_ROOTS),
    ("directed_transport_sum", 13, "into_text", TRANSPORT_ROOTS),
    ("directed_transport_sum", 16, "to_string", TRANSPORT_ROOTS),
    ("checked", 4, "into_text", PRIVATE_ROOTS),
    ("divided", 2, "into_text", PRIVATE_ROOTS),
    ("fallback_circular_maximum", 144, "into_text", ("fallback_circular_maximum",)),
)
# These nearby alternatives are inventoried but never capped by C6.
UNTOUCHED = (("fallback_circular_maximum", 13, "into_text"),
             ("fallback_circular_maximum", 27, "into_text"),
             ("fallback_circular_maximum", 164, "format"))


def inspect(snapshot):
    path = os.path.join(snapshot, CPP)
    if not os.path.exists(path):
        # Known original wrappers, not current-marker absence, establish history.
        if HELPERS["inspect_directed_helpers"](snapshot):
            raise ValueError("KD5C6: modern source missing CPP; refusing historical no-cap branch")
        return None
    with open(path, encoding="utf-8") as stream:
        source = stream.read()
    if hashlib.sha256(source.encode()).hexdigest() != CPP_FILE_PIN or hashlib.sha256(source.split("#[cfg(test)]", 1)[0].encode()).hexdigest() != CPP_PREFIX_PIN:
        raise ValueError("KD5C6: CPP file/prefix changed; re-review first-return/escape scope")
    keys = {}
    for name, digest in PINS.items():
        body, line = HELPERS["_indented_function"](source, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError("KD5C6: helper/root body changed: " + name)
        keys[name] = f"{CPP}:{line}:{name}"
    with open(os.path.join(snapshot, LIB), encoding="utf-8") as stream:
        consumer = stream.read()
    for name, digest in LIB_PINS.items():
        body, _ = HELPERS["_indented_function"](consumer, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError("KD5C6: attempt/clone consumer changed: " + name)
    sites = []
    tokens = {"format": "format!(", "to_string": ".to_string(", "into_text": ".into("}
    lines = source.splitlines()
    for name, offset, kind, roots in SITE_DEFS:
        line = int(keys[name].rsplit(":", 2)[1]) + offset
        if tokens[kind] not in lines[line - 1]:
            raise ValueError("KD5C6: source allocation kind/site changed")
        sites.append({"file": CPP, "line": line, "kind": kind, "fn": keys[name], "roots": [keys[root] for root in roots]})
    if len(sites) != 14 or len({(r["file"], r["line"], r["kind"]) for r in sites}) != 14:
        raise ValueError("KD5C6: target allocation inventory changed")
    headers = [(source.count("\n", 0, hit.start()) + 1, f"{CPP}:{source.count(chr(10), 0, hit.start()) + 1}:{hit.group(1)}") for hit in re.finditer(r"^[ \t]*(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b", source, re.M)]
    return {"keys": keys, "sites": sites, "headers": sorted(headers)}


def short(key):
    return key.split("/src/")[-1]


def site_rules(state):
    return [(f"certified_publication.rs:{r['line']}", {
        "from": [short(k) for k in r["roots"]], "per": "1",
        "why": "KD5C6: at most one allocation at this returning alternative per complete root attempt; preserve all root/member/case/replay counts, visits, nested formatters and outer clones"}) for r in state["sites"]]


def override_aliases(site):
    # Cover each actual short/kind and lexical fallback lookup, plus the
    # qualified/core-stripped spellings carried by manual rule tables.
    file, line, kind = site["file"], site["line"], site["kind"]
    stems = {short(file), file, file.split("core/")[-1],
             file.split("core/")[-1].split("/", 1)[-1]}
    return {f"{stem}:{line}{suffix}" for stem in stems for suffix in ("", ":" + kind)}


def verify_rules(state, table):
    if table.get("first_error_guard") != "KD5C6":
        raise ValueError("KD5C6: missing/versioned first-error guard")
    expected = dict(site_rules(state))
    marked = {k for k, v in table.get("site_from", {}).items() if "KD5C6:" in v.get("why", "")}
    if marked != set(expected):
        raise ValueError("KD5C6: added/missing rule inventory")
    for key, value in expected.items():
        actual = table["site_from"][key]
        if actual.get("per") != "1" or actual.get("from") != value["from"]:
            raise ValueError("KD5C6: root/per rule changed: " + key)
        site = next(r for r in state["sites"] if f"certified_publication.rs:{r['line']}" == key)
        for family in ("site_from", "site_zero", "site_total", "site_aggregate", "site_size", "lex_site_size"):
            for alias in override_aliases(site):
                if family == "site_from" and alias == key:
                    continue  # the sole canonical record was validated above
                if alias in table.get(family, {}):
                    raise ValueError(f"KD5C6: conflicting allocation override: {family}:{alias}")


def verify_graph(state, graph):
    # Use ALL unfiltered, fully qualified graph identities; never reach/M filters.
    edges = graph["edges"]
    keys = state["keys"]
    if any(k not in edges for k in keys.values()):
        raise ValueError("KD5C6: missing qualified helper/root node")
    closure = {keys[n] for n in PRIVATE}
    incoming = [(caller, target) for caller, targets in edges.items() for target in targets if target in closure and caller not in closure]
    if any(v != 1 for v in collections.Counter(incoming).values()) or {c for c, _ in incoming} != {keys[n] for n in ENTRY}:
        raise ValueError("KD5C6: private helper entry cutset changed")
    for target, parents in [
        ("directed_transport_sum", TRANSPORT_ROOTS),
        ("public_mpa_magnitude_in_pa_upper", ("maximum_cover",)),
        ("maximum_cover", ("circular_maximum",)),
        ("fallback_section_errors", ("fallback_circular_maximum",)),
    ]:
        hits = [c for c, ts in edges.items() for t in ts if t == keys[target]]
        if collections.Counter(hits) != collections.Counter(keys[n] for n in parents):
            raise ValueError("KD5C6: qualified parent inventory changed: " + target)


def verify_inventory(state, formats, lexical):
    keys = state["keys"]
    selected = {r["fn"] for r in state["sites"]}
    expected = [(r["file"], r["line"], r["kind"]) for r in state["sites"]]
    expected += [(CPP, int(keys[n].rsplit(":", 2)[1]) + offset, kind) for n, offset, kind in UNTOUCHED]
    headers = state["headers"]
    observed = []
    for row in formats + lexical:
        if row["file"] != CPP:
            continue
        owner = next((k for line, k in reversed(headers) if line <= row["line"]), None)
        if "fn" in row and row["fn"] != owner:
            raise ValueError("KD5C6: misqualified allocation owner")
        if owner in selected:
            observed.append((row["file"], row["line"], row["kind"]))
    if collections.Counter(observed) != collections.Counter(expected):
        raise ValueError("KD5C6: missing/duplicate/new exact kind+site inventory")


def positive_witness(state, file, line, kind, mult, counts):
    matches = [r for r in state["sites"] if (r["file"], r["line"], r["kind"]) == (file, line, kind)]
    if not matches:
        return
    if mult <= 0 or any(k not in counts or counts[k] <= 0 for k in matches[0]["roots"]):
        raise ValueError("KD5C6: each allocation/root must have a positive witnessed count")


def rebound_witness(state, file, line, kind, mult, counts):
    matches = [r for r in state["sites"] if (r["file"], r["line"], r["kind"]) == (file, line, kind)]
    if not matches:
        return
    positive_witness(state, file, line, kind, mult, counts)
    expected = sum(counts[k] for k in matches[0]["roots"])
    if mult != expected:
        raise ValueError("KD5C6: actual post-rebind allocation multiplicity changed")
