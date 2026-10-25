"""B7a: exact, source-bound removal of retired preview rule sites before carry."""
import hashlib
import json
import os
import re

# T4-U6 D3 (retained-pricing-rules.md, "T4-U6/U8: actual-source D3"): rows.rs changes only derived_for's
# source-derived stress rows (the D-5 basis twins); the retired sites stay retired. source_receipt.rs (433fc1f1e4)
# changes only `hash`, which serializes a borrowed wrapper through the new `hash_text` (U6-SH); the exact, ordinary
# and finalize constructors stay absent and finalize_composite/finalize_for stay unique.
# T4-S5 D3 (retained-pricing-rules.md §7.6, U4B-NQ): rows.rs adds only bind_for's derived_not_qualified loop and the
# new not_qualified_inputs/not_qualified_for; the four retired sites and the retired constructors stay absent.
# T4-S6 D3 (§8.8, S6-HS): source_receipt.rs's `hash` streams the checked canonical text into SHA-256 (hash_text and
# DomainPayload deleted, -9 lines before the constructors); the retired constructors stay absent and
# finalize_composite/finalize_for stay unique.
# Pre-0.4 failure unification D3 (§8.14): source_receipt.rs makes CapturedInvocation::requested pub(super) with a doc
# comment and the fallback test module pub(crate); the retired constructors stay deleted and finalize_composite and
# finalize_for stay unique.
SOURCE_FILES = {'core/product_physics/src/source_receipt/rows.rs': '539e1cbd60a0de35959e30b38076adcd6bae5451decaafc403970af1be1a26a4','core/product_physics/src/source_receipt.rs': '9ee47bc14c5fd2885010f16a9de396f0085335481de75d1df1af2bc6eba241c5'}
ROW_FILE = "core/product_physics/src/source_receipt/rows.rs"
RETIRED_IDS = {
    ROW_FILE + ":390": {"primary[&s.functional_indices[c]].id": {"bytes": 1024, "source": "RES"}},
    ROW_FILE + ":396": {"stable_suffix(&s.support_id)": {"bytes": 128, "source": "IN128"}},
    ROW_FILE + ":518": {"row.id": {"bytes": 1024, "source": "RES"}},
    ROW_FILE + ":529": {"primary[&function].id": {"bytes": 1024, "source": "RES"}},
}
ZW_OLD = "source_receipt.rs:634:exact,source_receipt.rs:836:finalize,source_receipt.rs:850:finalize_composite,source_receipt.rs:864:finalize_for"
ZW_NEW = "source_receipt.rs:850:finalize_composite,source_receipt.rs:864:finalize_for"
REASONS = {
    ":390": "B4/D1 retirement removes the non-composite support-resultant input-id clone.",
    ":396": "B4/D1 retirement removes the non-composite support-resultant ID format.",
    ":518": "B4/D1 retirement removes stress-summary stress_ids and its row-ID clone.",
    ":529": "B4/D1 retirement removes the non-composite stress primary-ID vector; composite inputs remain empty.",
}


def prepare(snapshot, base, output):
    """Preserve historical base files; adapt only this exact current-source retirement."""
    source_bytes = {}
    for file in SOURCE_FILES:
        path = os.path.join(snapshot, file)
        if not os.path.isfile(path):
            return {}, None
        with open(path, "rb") as stream:
            source_bytes[file] = stream.read()
    actual = {file: hashlib.sha256(raw).hexdigest() for file, raw in source_bytes.items()}
    source = {file: raw.decode("utf-8") for file, raw in source_bytes.items()}
    receipt = source["core/product_physics/src/source_receipt.rs"]
    old_methods = [bool(re.search(r"^\s+pub\(super\) fn " + name + r"\(", receipt, re.M))
                   for name in ("exact", "finalize")]
    if all(old_methods):
        return {}, None
    if any(old_methods) or actual != SOURCE_FILES:
        raise ValueError("B7a retirement source drift: re-review the full source files and surviving ownership")
    for name in ("finalize_composite", "finalize_for"):
        if len(re.findall(r"^\s+(?:pub\(super\) )?fn " + name + r"\(", receipt, re.M)) != 1:
            raise ValueError("B7a retirement changed a live finalizer")
    text = json.load(open(os.path.join(base, "text_args.g4.json"), encoding="utf-8"))
    for key, expected in RETIRED_IDS.items():
        if text["id_audit"].get(key) != expected:
            raise ValueError("B7a retirement base ID-audit preimage changed")
    with open(os.path.join(base, "sens.py"), "rb") as stream:
        sens = stream.read().decode("utf-8")
    old_line = 'ZW = "' + ZW_OLD + '"'
    if sens.count(old_line) != 1:
        raise ValueError("B7a retirement base ZW preimage changed")
    removals = []
    for key, expected in RETIRED_IDS.items():
        text["id_audit"].pop(key)
        removals.append({"file": "text_args.g4.json", "family": "id_audit", "key": key,
                         "rule": expected, "why": REASONS[":" + key.rsplit(":", 1)[1]]})
    sens = sens.replace(old_line, 'ZW = "' + ZW_NEW + '"', 1)
    for key in ZW_OLD.split(",")[:2]:
        removals.append({"file": "sens.py", "family": "ZW", "key": key,
                         "why": "B4/D1 retirement deletes the preview SourceBlocks constructor; the two live composite finalizers retain their zero-sensitivity tokens."})
    os.makedirs(output, exist_ok=True)
    out_text = os.path.join(output, "text_args.g4.json")
    out_sens = os.path.join(output, "sens.py")
    with open(out_text, "w", encoding="utf-8") as stream:
        json.dump(text, stream, indent=1)
    with open(out_sens, "wb") as stream:
        stream.write(sens.encode("utf-8"))
    report = {"source_fingerprint_scope": "full files", "source_fingerprints": actual,
              "adapter_sha256": hashlib.sha256(open(__file__, "rb").read()).hexdigest(),
              "base_preimages": {name: hashlib.sha256(open(os.path.join(base, name), "rb").read()).hexdigest()
                                for name in ("text_args.g4.json", "sens.py")},
              "prepared_files": {name: hashlib.sha256(open(os.path.join(output, name), "rb").read()).hexdigest()
                                 for name in ("text_args.g4.json", "sens.py")},
              "retired": removals}
    return {"text_args.g4.json": out_text, "sens.py": out_sens}, report

# These complete route-E tuples are the old preview-only zero edges. Their
# current resolved caller is bound as part of the preimage, not an alias.
ROUTE_E_RETIRED_EDGES = (
    (13, ("lib.rs:4442:solve_prepared_load_case", "source_receipt.rs:634:exact",
          "a selected exact case takes FinalizedSourceBlockCase::composite_exact (lib.rs:5714-5717); `exact` is the !is_exact arm (lib.rs:5718-5719)")),
    (14, ("lib.rs:4442:solve_prepared_load_case", "source_receipt.rs:717:ordinary",
          "an unselected exact case takes ordinary_physics (lib.rs:5726-5729); `ordinary` is the !is_exact arm (lib.rs:5730-5731)")),
    (15, ("lib.rs:2354:run_linear_static_preview_observed", "source_receipt.rs:836:finalize",
          "`composite` = source_selected && is_exact, so a selected exact invocation takes finalize_composite (lib.rs:2872-2875)")),
)


def route_e_keymap_allow(snapshot, edges, inherited):
    """Allow only these source-proven deleted keys; ordinary strict carry decides kind."""
    source = {}
    for file, expected in SOURCE_FILES.items():
        with open(os.path.join(snapshot, file), "rb") as stream:
            raw = stream.read()
        if hashlib.sha256(raw).hexdigest() != expected:
            raise ValueError("B7a route-E retirement source drift")
        source[file] = raw.decode("utf-8")
    receipt = source["core/product_physics/src/source_receipt.rs"]
    for name in ("exact", "ordinary", "finalize"):
        if re.search(r"^\s+(?:pub\(super\) )?fn " + name + r"\(", receipt, re.M):
            raise ValueError("B7a route-E constructor restored")
    for name in ("finalize_composite", "finalize_for"):
        if len(re.findall(r"^\s+(?:pub\(super\) )?fn " + name + r"\(", receipt, re.M)) != 1:
            raise ValueError("B7a route-E live finalizer changed")
    allowed = dict(inherited)
    # B7b: the two solve-case callers are resolved by name at the snapshot (rs_any in
    # sq2_decisions_E.py); bind the preimage to that same unique current definition.
    with open(os.path.join(snapshot, "core/product_physics/src/lib.rs"), encoding="utf-8") as stream:
        lib_lines = stream.read().split("\n")
    solve = [i + 1 for i, line in enumerate(lib_lines) if re.match(r"fn solve_prepared_load_case\(", line)]
    if len(solve) != 1:
        raise ValueError("B7a route-E solve-case caller is not unique")
    solve_key = f"lib.rs:{solve[0]}:solve_prepared_load_case"
    for index, expected in ROUTE_E_RETIRED_EDGES:
        if expected[0] == "lib.rs:4442:solve_prepared_load_case":
            expected = (solve_key,) + expected[1:]
        if len(edges) <= index or edges[index] != expected or edges.count(expected) != 1:
            raise ValueError("B7a route-E complete edge preimage or order changed")
        key = expected[1]
        if key in allowed:
            raise ValueError("B7a route-E inherited allowance overlaps")
        allowed[key] = ("deleted", "B4/D1 (b): preview receipt constructor retired; "
                        "current composite, ordinary_physics and composite finalizers remain positive.")
    return allowed
