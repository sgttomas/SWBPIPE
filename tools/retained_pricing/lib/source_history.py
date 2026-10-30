"""Retained-pricing import of the local repository-history adapter."""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from repository_history import resolve_revision, project_prefix
