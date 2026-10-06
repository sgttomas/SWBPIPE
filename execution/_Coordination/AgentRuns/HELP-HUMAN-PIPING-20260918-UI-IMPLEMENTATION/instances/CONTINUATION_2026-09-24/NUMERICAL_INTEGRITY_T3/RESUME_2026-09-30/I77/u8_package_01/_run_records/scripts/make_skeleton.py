#!/usr/bin/env python3
"""I77: a citation index with #1082's document vocabulary and no entries, pinned at NUM fd3990a710,
so that main's check_citations.py --list enumerates every citation in U8's added lines.
Usage: python3 make_skeleton.py <main's F2A_D1 citations.json> <out.json>"""
import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
d.update(num_commit="fd3990a7100b3c5f191deaa647ea0855d2cc2fa3", citations=[], code_anchors=[], copies={})
json.dump(d, open(sys.argv[2], "w", encoding="utf-8"), indent=1, ensure_ascii=False)
