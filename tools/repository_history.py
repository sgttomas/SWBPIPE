"""Resolve original Chirality revisions through the explicit extraction map.

Original identities remain documentary facts. These helpers select equivalent
local Git inputs; they never fetch objects or substitute another revision.
"""
from pathlib import Path
import re
import subprocess


def resolve_revision(repo, revision):
    explicit = revision.startswith('source:')
    source = revision.removeprefix('source:')
    if not explicit:
        result = subprocess.run(['git', '-C', str(repo), 'rev-parse', '--verify',
                                 revision + '^{commit}'], capture_output=True, text=True)
        if result.returncode == 0:
            return result.stdout.strip()
    if not re.fullmatch('[0-9a-fA-F]{7,40}', source):
        raise ValueError('Unknown source revision: ' + source)
    path = Path(repo) / 'reference/migration/commit-map.txt'
    if not path.is_file():
        raise ValueError('Source revision unavailable and no extraction map: ' + source)
    rows = [line.split(':') for line in path.read_text().splitlines() if line]
    if any(len(row) != 2 or any(not re.fullmatch('[0-9a-f]{40}', value) for value in row)
           for row in rows):
        raise ValueError('Invalid extraction commit map')
    matches = [(old, new) for old, new in rows if old.startswith(source.lower())]
    if len(matches) != 1:
        raise ValueError('Source revision missing or ambiguous: ' + source)
    result = subprocess.run(['git', '-C', str(repo), 'rev-parse', '--verify',
                             matches[0][1] + '^{commit}'], capture_output=True, text=True)
    if result.returncode:
        raise ValueError('Mapped source commit unavailable: ' + source)
    import json
    trees_path = Path(repo) / 'reference/migration/source-trees.json'
    if not trees_path.is_file():
        raise ValueError('Source tree bindings unavailable: ' + source)
    expected = json.loads(trees_path.read_text()).get(matches[0][0])
    tree = subprocess.run(['git', '-C', str(repo), 'rev-parse', '--verify',
                           result.stdout.strip() + '^{tree}'], capture_output=True, text=True)
    if not isinstance(expected, str) or tree.returncode or tree.stdout.strip() != expected:
        raise ValueError('Mapped source tree differs from recorded original: ' + source)
    return result.stdout.strip()


def project_prefix(repo, revision):
    for prefix in ('', 'projects/chirality-piping/'):
        result = subprocess.run(['git', '-C', str(repo), 'cat-file', '-t',
                                 revision + ':' + prefix + 'core'], capture_output=True, text=True)
        if result.returncode == 0 and result.stdout.strip() == 'tree':
            return prefix
    raise ValueError('Revision has no SWBPIPE source tree: ' + revision)


def source_spec(repo, revision, path):
    """Translate a historical Git address while retaining its original citation."""
    resolved = resolve_revision(repo, revision)
    relative = path.removeprefix('projects/chirality-piping/').removeprefix('./')
    return resolved + ':' + project_prefix(repo, resolved) + relative
