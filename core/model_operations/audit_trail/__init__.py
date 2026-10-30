"""User-acceptance audit trail for structured model operations."""

from .engine import (
    AUDIT_TRAIL_VERSION,
    canonical_json,
    record_operation_audit_trail,
)

__all__ = [
    "AUDIT_TRAIL_VERSION",
    "canonical_json",
    "record_operation_audit_trail",
]
