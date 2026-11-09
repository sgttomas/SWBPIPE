"""Python-only type guards of the SI MBF writer's format-6 settings.

The Rust port's `WriterSettings.max_layout_rows` is an `Option<i64>`, so a float, a bool or a string cannot reach it,
and these calls have no Rust behaviour to compare. They are kept outside the suites that
`tools/fixtures/caepipe_mbf_writer_vectors.py` harvests, whose every recorded call must replay in Rust; the integer
range refusals (−1 and 2^53 + 1) are in `test_caepipe_mbf_si_writer.py` and are replayed there.
"""
from __future__ import annotations

from pathlib import Path
import sys

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))
from test_caepipe_mbf_si_writer import assert_refused, cantilever  # noqa: E402
from core.handoff.caepipe_mbf.si_writer import render_caepipe_mbf_si  # noqa: E402


@pytest.mark.parametrize("limit", [1.5, True, "20"])
def test_a_row_limit_of_a_type_the_rust_signature_cannot_hold_is_refused(limit):
    assert_refused(render_caepipe_mbf_si(cantilever(), title="ROWS", max_layout_rows=limit), "MBF-SI-INPUT-INVALID",
                   "max_layout_rows", "from 0 to 2^53")
