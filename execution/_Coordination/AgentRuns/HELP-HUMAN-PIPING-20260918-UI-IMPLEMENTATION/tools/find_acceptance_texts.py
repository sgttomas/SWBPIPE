#!/usr/bin/env python3
"""List every live file that carries a registered acceptance text (BS-ACCEPT).

Run from the project root (`projects/chirality-piping`). The search normalizes
whitespace, Markdown emphasis and source-string continuations, so a sentence
wrapped across lines or split across string literals is still found. Ruled
history and frozen evidence are skipped. `--scope-of-work` also counts the live
ScopeOfWork.md files, which the owner's direction of 2026-09-18 leaves as they are.
"""
import os
import re
import subprocess
import sys

TEXTS = {
    "canonical-1st": "results are engineering decision-support information.",
    "canonical-2nd": (
        "acceptance, professional judgment, and any certification, sealing, or "
        "code-compliance determination remain with the responsible engineer and "
        "project authority"
    ),
    "short-A": "acceptance and professional judgment remain with the responsible engineer",
    "short-B": "acceptance stays with the responsible engineer",
    "short-C": "decision-support information for review by the responsible engineer",
}
LOOSE = re.compile(r"acceptance[^.]{0,160}responsible engineer", re.I)
SKIP = (
    "execution/", "validation/evidence/", "validation/witness/", "validation/hand_calcs/",
    "plans/", "loop/", "provenance/", "docs/_history/", "docs/_ScopeChange/",
)


def normalize(text: str) -> str:
    text = re.sub(r"[`*_>|]", " ", text)
    text = re.sub(r"\\\s*\n\s*", " ", text)          # Rust string continuation
    text = re.sub(r"\"\s*\+?\s*\n?\s*\"", "", text)  # adjacent string literals
    return re.sub(r"\s+", " ", text).lower()


def main() -> int:
    files = subprocess.check_output(["git", "ls-files"], text=True).splitlines()
    hits = 0
    for path in files:
        if path.startswith(SKIP) or not os.path.isfile(path):
            continue
        try:
            body = normalize(open(path, encoding="utf-8").read())
        except (UnicodeDecodeError, OSError):
            continue
        found = [name for name, text in TEXTS.items() if text in body]
        loose = len(LOOSE.findall(body))
        if found or loose:
            hits += 1
            print(f"{path}: {','.join(found) or '-'} loose={loose}")
    print(f"{hits} live file(s) outside execution and frozen history")
    if "--scope-of-work" in sys.argv:
        sow = [p for p in files if p.startswith("execution/") and p.endswith("ScopeOfWork.md")]
        carrying = 0
        for path in sow:
            body = normalize(open(path, encoding="utf-8").read())
            if any(t in body for t in TEXTS.values()) or LOOSE.search(body):
                carrying += 1
        print(f"live ScopeOfWork.md files carrying a registered text: {carrying} of {len(sow)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
