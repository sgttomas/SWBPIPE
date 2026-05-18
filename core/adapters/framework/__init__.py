"""Format-neutral adapter framework checks for OpenPipeStress.

This module validates already-declared adapter metadata and invented fixture
payloads. It does not parse external file formats, access files or network
resources, choose transport/runtime behavior, or make legal/professional
determinations.
"""

from .adapter_framework import (
    AdapterFinding,
    AdapterValidationResult,
    build_result,
    validate_adapter_declaration,
)

__all__ = [
    "AdapterFinding",
    "AdapterValidationResult",
    "build_result",
    "validate_adapter_declaration",
]
