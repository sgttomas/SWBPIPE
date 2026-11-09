"""The pressure-elongation switch shared by SWBPIPE's maintained producers.

DEL-04-07 `Work/pressure-elongation.md` §3.1 adds the model setting
`model_settings.pressure_elongation = {"operating_load_cases": [ids]}`; absent
means off. A producer that writes model documents takes one explicit switch:

- ``absent`` (default): today's output; the producer writes no member;
- ``all``: every produced pressure document names all its pressurised load
  cases, in document order (the §7 regression anchor: "included" in a case is
  today's mechanics, so the physical numbers must not move);
- ``none``: the member is omitted explicitly (the same bytes as ``absent``).

A load case is pressurised when one of its pressure regions carries a nonzero
pressure, or when the case's ``condition_use.pressure`` key resolves, on a pipe
of one of its regions, to a nonzero condition pressure. A document without a
``pressure_contract`` or without a pressurised case gets no member under any
switch value (the setting is refused there by name).

Standard library only, so the independent reference generators can import it
(`sys.path` insertion of the repository root, then
``from tools.validation import pressure_elongation``).
"""
from __future__ import annotations

import argparse
import os
from typing import Any, Iterator

MODES = ("absent", "all", "none")
DEFAULT = "absent"
ENV = "SWBPIPE_PRESSURE_ELONGATION"
SETTINGS = "model_settings"
MEMBER = "pressure_elongation"
CASES = "operating_load_cases"


def check_mode(mode: str) -> str:
    if mode not in MODES:
        raise ValueError(f"pressure elongation switch must be one of {', '.join(MODES)}; got {mode!r}")
    return mode


def resolve_mode(cli_value: str | None = None) -> str:
    """The switch value: the command-line value, else the environment, else ``absent``."""
    if cli_value is not None:
        return check_mode(cli_value)
    return check_mode(os.environ.get(ENV) or DEFAULT)


def add_argument(parser: argparse.ArgumentParser) -> None:
    """Add the producer's ``--pressure-elongation {absent,all,none}`` switch."""
    parser.add_argument(
        "--pressure-elongation", choices=MODES, default=None,
        help=f"model_settings.pressure_elongation in produced documents (default: ${ENV} or {DEFAULT})")


def argv_mode(argv: list[str]) -> str:
    """The switch from a hand-parsed argv (``--pressure-elongation=V`` or ``--pressure-elongation V``)."""
    for index, arg in enumerate(argv):
        if arg.startswith("--pressure-elongation="):
            return resolve_mode(arg.split("=", 1)[1])
        if arg == "--pressure-elongation":
            if index + 1 >= len(argv):
                raise ValueError("--pressure-elongation needs a value")
            return resolve_mode(argv[index + 1])
    return resolve_mode(None)


def is_document(value: Any) -> bool:
    """A model document under the exact pressure contract."""
    return isinstance(value, dict) and isinstance(value.get("pressure_contract"), dict) and isinstance(value.get("load_cases"), list)


def documents(container: Any, pointer: str = "") -> Iterator[tuple[str, dict]]:
    """Every pressure-contract model document inside ``container``, with its JSON pointer, in document order."""
    if is_document(container):
        yield pointer, container
        return
    if isinstance(container, dict):
        for key, value in container.items():
            yield from documents(value, f"{pointer}/{_escape(key)}")
    elif isinstance(container, list):
        for index, value in enumerate(container):
            yield from documents(value, f"{pointer}/{index}")


def _escape(key: str) -> str:
    return str(key).replace("~", "~0").replace("/", "~1")


def _nonzero(quantity: Any) -> bool:
    if not isinstance(quantity, dict):
        return False
    value = quantity.get("value")
    if isinstance(value, bool) or not isinstance(value, (int, float, str)):
        return False
    try:
        return float(value) != 0.0
    except ValueError:
        return False


def _condition_pressures(doc: dict, key: str, pipe_ids: set) -> list:
    sets = {s.get("id"): s for s in doc.get("condition_sets") or [] if isinstance(s, dict)}
    found = []
    for pipe in doc.get("pipe_segments") or []:
        if not isinstance(pipe, dict) or pipe.get("id") not in pipe_ids:
            continue
        condition_set = sets.get(pipe.get("condition_set_ref"))
        for condition in (condition_set or {}).get("conditions") or []:
            if isinstance(condition, dict) and condition.get("key") == key:
                found.append(condition.get("pressure"))
    return found


def pressurised_case_ids(doc: dict) -> list[str]:
    """The ids of the document's pressurised load cases, in document order."""
    if not is_document(doc):
        return []
    ids = []
    for case in doc["load_cases"]:
        if not isinstance(case, dict):
            continue
        regions = [r for r in case.get("pressure_regions") or [] if isinstance(r, dict)]
        pressurised = any(_nonzero(r.get("pressure")) for r in regions)
        key = (case.get("condition_use") or {}).get("pressure") if isinstance(case.get("condition_use"), dict) else None
        if not pressurised and key is not None and regions:
            members = {pipe for r in regions for pipe in r.get("member_pipe_ids") or []}
            pressurised = any(_nonzero(p) for p in _condition_pressures(doc, key, members))
        if pressurised and isinstance(case.get("id"), str):
            ids.append(case["id"])
    return ids


def apply(doc: dict, mode: str) -> dict:
    """Apply the switch to one model document in place and return it.

    ``all`` writes ``model_settings.pressure_elongation`` naming every
    pressurised case (other ``model_settings`` members are kept, the new member
    last); ``absent`` and ``none`` remove any such member, and remove a
    ``model_settings`` record that only held it. On a document with no
    pressurised case every value leaves the document as it is.
    """
    check_mode(mode)
    if not is_document(doc):
        return doc
    ids = pressurised_case_ids(doc)
    settings = doc.get(SETTINGS)
    if mode == "all":
        if ids:
            if not isinstance(settings, dict):
                settings = doc[SETTINGS] = {}
            settings.pop(MEMBER, None)
            settings[MEMBER] = {CASES: ids}
        return doc
    if isinstance(settings, dict) and MEMBER in settings:
        del settings[MEMBER]
        if not settings:
            del doc[SETTINGS]
    return doc


def apply_to_container(container: Any, mode: str) -> Any:
    """Apply the switch to every pressure document inside ``container`` (in place) and return it."""
    check_mode(mode)
    for _, doc in list(documents(container)):
        apply(doc, mode)
    return container
