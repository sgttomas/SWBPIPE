"""Reproduce only RV51's two import fixtures; no maintained source is modified."""
from pathlib import Path
import hashlib, json, os, subprocess
ROOT = Path(__file__).resolve().parent
SOURCE = Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a')
CANDIDATE = "fdae294643b798c1849da8b2e643085562593686"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
assert subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=SOURCE, env=env, text=True).strip() == CANDIDATE
K = SOURCE / "projects/chirality-piping/core/solver/frame_kernel"
imports = ROOT / "imports"
imports.mkdir(exist_ok=True)
spec = json.loads((ROOT / "import_recipe.json").read_text())
for name, item in spec.items():
    original = (SOURCE / item["source"]).read_bytes()
    assert hashlib.sha256(original).hexdigest() == item["source_sha256"]
    text = original.decode()
    for before, after in item["replacements"]:
        assert text.count(before) == 1
        text = text.replace(before, after)
    assert hashlib.sha256(text.encode()).hexdigest() == item["prepared_sha256"]
    (imports / name).write_text(text)
print("Two import-only fixtures prepared and hash verified.")
