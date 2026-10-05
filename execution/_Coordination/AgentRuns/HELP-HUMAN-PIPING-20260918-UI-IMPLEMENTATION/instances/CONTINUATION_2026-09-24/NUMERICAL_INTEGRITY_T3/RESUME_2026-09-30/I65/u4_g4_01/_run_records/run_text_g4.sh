#!/bin/bash
# I65 U4 G4: the T08 text run on the repaired call graph at the pinned basis b1f80234dc (stdlib
# Python; no Cargo; read-only). Root: the retained Direct entry only (D-2), which now reaches the
# W1 phases, the serializer and the precommit reader (U3 grant 1); product_physics depends on
# result_export at runtime (decision 5). The envelope-only variant (D_env, Text(diag_env)) zeroes
# the source-blocks finalization and every W1/serializer/reader file.
# Usage (from the snapshot's projects/chirality-piping): run_text_g4.sh <tag> <out dir>
set -euo pipefail
H="$(cd "$(dirname "$0")" && pwd)"
tag="$1"; out="$2"
ROOT=run_linear_static_preview_value_with_retained_direct
CG_REPAIR=1 CG_EXTRA_DEPS="core/product_physics:core/reporting/result_export" CG_RULES="$H/callgraph_rules.g4.json" \
  CG_EDGES_OUT="$out/edges_$tag.json" CG_AUDIT_OUT="$out/audit_$tag.json" python3 "$H/callgraph_g4.py" . \
  "$out/cg_$tag.out.json" $ROOT $(cat "$H/crate_dirs.txt") > /dev/null
python3 "$H/text_lexicon.py" . "$out/edges_$tag.json" $ROOT "$out/lexicon_$tag.json" > /dev/null
ENVZERO=core/product_physics/src/source_receipt,core/product_physics/src/retained_wire.rs,core/product_physics/src/retained_product.rs,core/product_physics/src/retained_receipt.rs,core/reporting/result_export
for w in caps milestone; do
  TB_LEXICON="$out/lexicon_$tag.json" TB_COMPOSITE="$H/composite_text.$w.json" python3 "$H/text_budget.py" . \
    "$H/template_inventory_head.out.json" "$out/edges_$tag.json" "$H/loop_bounds.g4.json" "$H/text_args.g4.json" $ROOT $w \
    > "$out/tb_${tag}_$w.json"
  TB_EXTRA_ZERO=$ENVZERO TB_LEXICON="$out/lexicon_$tag.json" TB_COMPOSITE="$H/composite_text.$w.json" \
    python3 "$H/text_budget.py" . "$H/template_inventory_head.out.json" "$out/edges_$tag.json" "$H/loop_bounds.g4.json" \
    "$H/text_args.g4.json" $ROOT $w > "$out/tb_${tag}_env_$w.json"
  # per-branch totals: X (exact-block selected: retained_w1 returns at its coexistence check,
  # PP lib.rs:2955-2957) and W (no selected source: the source-blocks finalization never runs,
  # lib.rs:2852-2862 and :5223-5231)
  TB_FN_ZERO=lib.rs:2950:retained_w1 TB_LEXICON="$out/lexicon_$tag.json" TB_COMPOSITE="$H/composite_text.$w.json" \
    python3 "$H/text_budget.py" . "$H/template_inventory_head.out.json" "$out/edges_$tag.json" "$H/loop_bounds.g4.json" \
    "$H/text_args.g4.json" $ROOT $w > "$out/tb_${tag}_X_$w.json"
  TB_FN_ZERO=source_receipt.rs:636:exact,source_receipt.rs:839:finalize,source_receipt.rs:853:finalize_composite,source_receipt.rs:867:finalize_for \
    TB_LEXICON="$out/lexicon_$tag.json" TB_COMPOSITE="$H/composite_text.$w.json" \
    python3 "$H/text_budget.py" . "$H/template_inventory_head.out.json" "$out/edges_$tag.json" "$H/loop_bounds.g4.json" \
    "$H/text_args.g4.json" $ROOT $w > "$out/tb_${tag}_W_$w.json"
done
python3 - "$out" "$tag" <<'PY'
import json, sys
out, tag = sys.argv[1], sys.argv[2]
for v in ("", "_env", "_X", "_W"):
    for w in ("caps", "milestone"):
        d = json.load(open(f"{out}/tb_{tag}{v}_{w}.json"))
        print(tag + v, w, {k: d[k] for k in ("reachable_fns", "complete", "sites_with_positive_multiplicity", "total_text_requested_bytes",
                                               "largest_single_site_bytes", "D_diagnostics", "retained_diagnostic_bytes")},
              "unmapped", d["unmapped_loop_headers"], "unclassified", d["unclassified_args"])
PY
