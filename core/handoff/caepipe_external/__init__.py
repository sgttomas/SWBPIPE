"""CAEPIPE external-run evidence foundation."""

from .run import (
    CAEPIPE_EXTERNAL_RUN_VERSION,
    build_caepipe_external_run_package,
    build_skipped_caepipe_external_run_package,
    canonical_csv,
    canonical_json,
    diagnostics_for_caepipe_external_run_package,
    parse_caepipe_csv_text,
    write_caepipe_external_run_package,
)
from .print_csv import (
    PARSER_VERSION,
    PRINT_CSV_COVERAGE,
    parse_caepipe_print_csv,
    read_caepipe_print_csv,
)
from .harness import (
    HarnessConfig,
    HarnessSkip,
    case_paths,
    resolve_harness_config,
    run_caepipe_round_trip,
)
from .correlation import (
    build_comparison_record,
    canonical_model_hash,
)

__all__ = [
    "CAEPIPE_EXTERNAL_RUN_VERSION",
    "HarnessConfig",
    "HarnessSkip",
    "PARSER_VERSION",
    "PRINT_CSV_COVERAGE",
    "build_caepipe_external_run_package",
    "build_comparison_record",
    "build_skipped_caepipe_external_run_package",
    "canonical_csv",
    "canonical_json",
    "canonical_model_hash",
    "case_paths",
    "diagnostics_for_caepipe_external_run_package",
    "parse_caepipe_csv_text",
    "parse_caepipe_print_csv",
    "read_caepipe_print_csv",
    "resolve_harness_config",
    "run_caepipe_round_trip",
    "write_caepipe_external_run_package",
]
