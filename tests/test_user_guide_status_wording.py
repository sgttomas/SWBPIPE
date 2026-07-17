#!/usr/bin/env python3
"""Focused checks for DEL-11-01 user-guide analysis status wording."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
USER_GUIDE = ROOT / "docs" / "user_guide" / "index.md"


def test_user_guide_lists_external_human_approval_status_without_software_claim():
    text = USER_GUIDE.read_text(encoding="utf-8")
    lower = text.lower()

    assert "`HUMAN_APPROVED_FOR_PROJECT`" in text
    assert "external human acceptance record" in lower
    assert "exact reviewed hashes" in lower
    assert "not emitted by the solver or rule-pack evaluator" in lower
    # DEC-081 claims taxonomy: the row carries the registry BS-ACCEPT
    # standalone variant instead of an ad-hoc prohibition litany.
    assert (
        "acceptance, professional judgment, and any certification, sealing, "
        "or code-compliance determination remain with the responsible "
        "engineer and project authority" in lower
    )


if __name__ == "__main__":
    test_user_guide_lists_external_human_approval_status_without_software_claim()
