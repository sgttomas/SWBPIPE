"""§8.63, MODAL-CS, continued: the ID audit's eleven formats and its own source assertions.

test_863_modal_completed_set.py holds the rules' summary and the shared helpers and constants. The tests of
§8.63 are in six files so that a hosted run, which keeps one file on one worker, spreads them over its workers.
"""
import hashlib
import re

import pytest

from test_863_modal_completed_set import RS, copy_core, decisions, edit, sha, tool_with_pin


ID_FORMATS = ["n_c = {computed}", "r = {r}", "r × n = {r} × {}", "r × g = {r} × {}", "r × ρ = {r} × {}", "{value} as u64",
              "r × {symbol} = {completed} × {width}", "{count} entries × {element} B", "at most u64::MAX = {}",
              "at most usize::MAX = {}", "the bytes exceed the addressable size, at most isize::MAX = {} B"]


@pytest.mark.parametrize("fmt", ID_FORMATS)
def test_each_of_the_id_audits_eleven_formats_is_required(tmp_path, monkeypatch, fmt):
    """Past the whole-file pin, removing any one of the eleven expression or bound formats stops MODAL-CS-ID."""
    snap = copy_core(tmp_path / "src")
    target = snap / RS
    text = target.read_text(encoding="utf-8")
    assert text.count('format!("' + fmt + '"') >= 1
    target.write_text(text.replace('format!("' + fmt + '"', 'format!("' + fmt + ' (changed)"'), encoding="utf-8")
    with pytest.raises(SystemExit, match="MODAL-CS-ID: the boundary's format `" + re.escape(fmt) + "` is gone"):
        decisions(monkeypatch, snap, tool=tool_with_pin(tmp_path, sha(snap, RS)))


@pytest.mark.parametrize("old, new, message", [
    ('format!("n_c = {computed}")', 'format!("n_c := {computed}")', "the boundary's format `n_c = {computed}` is gone"),
    ('Some(limit) => format!("the caller\'s limit on it is {limit} and was not exceeded',
     'Some(limit) => format!("the caller\'s stated limit on it is {limit} and was not exceeded',
     "modal_completed_set_refusal's limit text changed"),
    ('None => "no completed-set limit was supplied".to_string(),', 'None => "no completed-set limit supplied".to_string(),',
     "modal_completed_set_refusal's limit text changed"),
    ('let normalized_entries = entries(ModalCompletedSetDimension::NormalizedEntries, "n", coordinates)?;',
     'let normalized_entries = entries(ModalCompletedSetDimension::NormalizedEntries, "nn", coordinates)?;',
     "modal_completed_set_sizes' symbols changed"),
])
def test_the_id_audits_own_source_assertions_stop(tmp_path, monkeypatch, old, new, message):
    """Past the whole-file pin (the copy's pin is the mutated file's hash), each MODAL-CS-ID assertion stops."""
    snap = copy_core(tmp_path / "src")
    edit(snap, RS, old, new)
    tool = tool_with_pin(tmp_path, hashlib.sha256((snap / RS).read_bytes()).hexdigest())
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap, tool=tool)
