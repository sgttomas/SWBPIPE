"""Original history identities must select the recorded project tree, offline."""
from pathlib import Path
import json
import pytest
from tools.repository_history import resolve_revision
ROOT = Path(__file__).resolve().parents[1]
BASIS = '2243be380ba6f9b45ef62ea25d2ee0dd968b2a25'


def test_required_source_basis_resolves_to_recorded_import():
    record = json.loads((ROOT / 'reference/migration/historical-inputs.json').read_text())['pricing_basis']
    assert resolve_revision(ROOT, 'source:' + BASIS) == record['imported_commit']


def test_wrong_existing_target_cannot_select_another_tree(monkeypatch):
    path = ROOT / 'reference/migration/commit-map.txt'
    records = json.loads((ROOT / 'reference/migration/historical-inputs.json').read_text())
    original = Path.read_text
    def altered(self, *args, **kwargs):
        text = original(self, *args, **kwargs)
        if self == path:
            rows = dict(line.split(':') for line in text.splitlines() if line)
            rows[BASIS] = records['baseline']['imported_commit']
            return ''.join(old + ':' + new + '\n' for old, new in rows.items())
        return text
    monkeypatch.setattr(Path, 'read_text', altered)
    with pytest.raises(ValueError, match='tree differs'):
        resolve_revision(ROOT, 'source:' + BASIS)


def test_missing_and_duplicate_identity_refuse(monkeypatch):
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(ROOT, 'source:' + '0' * 40)
    path = ROOT / 'reference/migration/commit-map.txt'
    original = Path.read_text
    def duplicated(self, *args, **kwargs):
        text = original(self, *args, **kwargs)
        if self == path:
            text += next(line for line in text.splitlines() if line.startswith(BASIS + ':')) + '\n'
        return text
    monkeypatch.setattr(Path, 'read_text', duplicated)
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(ROOT, 'source:' + BASIS)
