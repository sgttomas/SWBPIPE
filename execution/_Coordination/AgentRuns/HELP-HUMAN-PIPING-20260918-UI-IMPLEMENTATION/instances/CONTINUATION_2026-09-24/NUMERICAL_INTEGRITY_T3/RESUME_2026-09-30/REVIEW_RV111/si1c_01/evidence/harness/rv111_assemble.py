#!/usr/bin/env python3
"""RV111: assemble REVIEW.md from the staged sections, substituting {{KEY}} placeholders from a JSON map.
Usage: rv111_assemble.py <stage dir> <values.json>"""
import json
import re
import sys

STAGE, VALUES = sys.argv[1], sys.argv[2]
values = json.load(open(VALUES, encoding="utf-8"))
order = ["head.md", "sections_1_6.md", "section_3.md", "section_4a.md", "section_4b.md", "section_4d.md",
         "section_4f.md", "section_4e.md", "section_4c.md", "section_5.md", "section_6.md", "section_host.md",
         "section_8.md"]
text = "\n".join(open(f"{STAGE}/{name}", encoding="utf-8").read().rstrip("\n") + "\n" for name in order)
for key, value in values.items():
    text = text.replace("{{" + key + "}}", value)
left = re.findall(r"\{\{[A-Z0-9_]+\}\}", text)
assert not left, left
open(f"{STAGE}/REVIEW.md", "w", encoding="utf-8").write(text)
print(len(text.splitlines()), "lines")
