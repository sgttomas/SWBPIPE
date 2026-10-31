"""The pricing tool's git reads. Every read runs in one sanitized environment and decodes as strict UTF-8, so nothing
outside the repository's committed objects and the explicit command line reaches a priced result: no global or system
configuration, no GIT_* variable that changes output (GIT_DIFF_OPTS, GIT_EXTERNAL_DIFF, GIT_CONFIG_*, GIT_ATTR_SOURCE,
...), no global attributes file and no locale. The repository's own .git/config still applies; callers therefore also
pin every output-affecting option on the command line (linemap_lib.DIFF_PIN), and price.py's snapshot reads raw blobs.
"""
import os, subprocess

# The GIT_* variables kept: those that locate the repository and its objects, and change no output for given objects.
KEEP = frozenset(("GIT_DIR", "GIT_WORK_TREE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES",
                  "GIT_CEILING_DIRECTORIES", "GIT_DISCOVERY_ACROSS_FILESYSTEM"))


def env(base=None):
    e = {k: v for k, v in (os.environ if base is None else base).items() if not k.startswith("GIT_") or k in KEEP}
    e.update(GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1", GIT_OPTIONAL_LOCKS="0")
    return e


def run(repo, *args, text=True, input=None):
    """git -C repo args: its stdout as strict UTF-8 text, or bytes with text=False."""
    out = subprocess.run(["git", "-C", str(repo), "-c", "core.quotePath=true", "-c", f"core.attributesFile={os.devnull}", *args],
                         input=input, env=env(), capture_output=True, check=True).stdout
    return out.decode("utf-8", errors="strict") if text else out
