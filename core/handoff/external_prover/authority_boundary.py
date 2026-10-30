"""Shared authority-boundary text screening for PKG-15 handoff metadata."""

from __future__ import annotations

import json
from typing import Any, Mapping


PROHIBITED_AUTHORITY_TERMS = {
    "approval",
    "approved",
    "certification",
    "cert" + "ified",
    "code_compliance",
    "code-compliance",
    "code " + "compliant",
    "compliant",
    "seal",
    "se" + "aled",
    "sealing",
    "authentication",
    "authentic" + "ated",
    "external_validation",
    "external " + "validation",
    "validated",
    "professional_acceptance",
    "professional " + "acceptance",
    "engineering_acceptance",
    "engineering " + "acceptance",
    "prover_status",
    "prover status",
    "lifecycle",
}


def contains_prohibited_authority_term(value: Any) -> bool:
    """Return true when user-controlled metadata contains authority wording."""

    return bool(prohibited_authority_terms(value))


def prohibited_authority_terms(value: Any) -> list[str]:
    """Return prohibited authority terms found in metadata-like values."""

    text = _scan_text(value)
    return [term for term in sorted(PROHIBITED_AUTHORITY_TERMS) if term in text]


def _scan_text(value: Any) -> str:
    if isinstance(value, (Mapping, list)):
        return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).lower()
    return str(value).lower()
