"""Every file under fixtures/ stays well under GitHub's 100 MB per-file limit."""
import os
from pathlib import Path
import sys

if str(Path(__file__).resolve().parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).resolve().parent))
from retained_precision_corpus import CORPUS_FILES  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
LIMIT_BYTES = 50_000_000


def test_no_fixture_file_exceeds_50_mb():
    large = []
    for directory, _, names in os.walk(ROOT / "fixtures"):
        for name in names:
            path = Path(directory) / name
            if path.stat().st_size > LIMIT_BYTES:
                large.append(f"{path.relative_to(ROOT)} ({path.stat().st_size} bytes)")
    assert not large, (
        f"fixture files over {LIMIT_BYTES} bytes: {large}. Append a new corpus file (a new snapshot listed in each "
        "reader's corpus file list) instead of growing an existing one; GitHub refuses files over 100 MB.")


def test_retained_precision_corpus_files_are_the_listed_ones():
    """A corpus file the readers' list omits would be silently unread."""
    on_disk = sorted(p.relative_to(ROOT).as_posix() for p in (ROOT / "fixtures/results").glob("retained_precision_cases*.json"))
    assert on_disk == sorted(CORPUS_FILES)
    for harness in ("core/reporting/result_export/tests/retained_precision_contract.rs",
                    "apps/desktop/src/test-support/retainedPrecisionCorpus.ts"):
        text = (ROOT / harness).read_text()
        assert all(Path(name).name in text for name in CORPUS_FILES), harness
