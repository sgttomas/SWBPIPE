"""Target mapping and unsupported-behavior contract helpers."""

from .contract import (
    PROFESSIONAL_BOUNDARY,
    TARGET_MAPPING_CONTRACT_VERSION,
    build_target_mapping_contract,
    canonical_json,
    diagnostics_for_target_mapping_contract,
)

__all__ = [
    "PROFESSIONAL_BOUNDARY",
    "TARGET_MAPPING_CONTRACT_VERSION",
    "build_target_mapping_contract",
    "canonical_json",
    "diagnostics_for_target_mapping_contract",
]
