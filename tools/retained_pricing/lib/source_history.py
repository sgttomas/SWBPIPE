"""Retained-pricing import of the local repository-history adapter: its lookups run with pinned_git's sanitized
environment and options, as every other pricing git read does."""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
sys.path.insert(0, str(Path(__file__).resolve().parent))
import pinned_git
import repository_history


def resolve_revision(repo, revision):
    return repository_history.resolve_revision(repo, revision, env=pinned_git.env(), git_options=pinned_git.OPTIONS)


def project_prefix(repo, revision):
    return repository_history.project_prefix(repo, revision, env=pinned_git.env(), git_options=pinned_git.OPTIONS)
