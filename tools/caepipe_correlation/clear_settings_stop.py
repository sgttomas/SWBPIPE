"""Record a clearance for a CAEPIPE harness run that stops later live steps as `settings_not_restored` (DEL-17-05).

From the project root:

    python3 -m tools.caepipe_correlation.clear_settings_stop --case ID --run runs/NNNNNN --by NAME --reason TEXT

A live step that reached the program and its saved settings writes a stage record in its run folder first, and a
check record once the settings check has finished. When the newest holder of the run lock has the first and not the
second (or a check that is not clean), every later live step is refused as `settings_not_restored` until a clearance
for that run exists. This command is the explicit act that records one, after the person running it has seen to the
guest's saved settings: a new file `settings_clearance.json` in that run's folder, created exclusively (never
rewritten), naming the run, who cleared it and why. The harness never records a clearance by itself.

It needs only the exchange folder (SWBPIPE_CAEPIPE_EXCHANGE, with its data-boundary checks), not the guest.
"""

from __future__ import annotations

import argparse
import os
import sys
from typing import Mapping

from core.handoff.caepipe_external.harness import (
    ContainmentError,
    HarnessSkip,
    record_settings_clearance,
    resolve_harness_config,
)


def main(argv: list[str] | None = None, environ: Mapping[str, str] = os.environ) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--case", required=True, metavar="ID", help="the case whose run is cleared")
    parser.add_argument("--run", required=True, metavar="runs/NNNNNN", help="the run folder that stops later steps")
    parser.add_argument("--by", required=True, metavar="NAME", help="who cleared it")
    parser.add_argument("--reason", required=True, metavar="TEXT", help="why, and what was done about the settings")
    args = parser.parse_args(argv)
    config = resolve_harness_config(environ, require_vm_tools=False)
    if isinstance(config, HarnessSkip):
        print(f"error: the exchange folder is not available: {config.reason}", file=sys.stderr)
        return 2
    try:
        path = record_settings_clearance(config, args.case, args.run, cleared_by=args.by, reason=args.reason)
    except (ContainmentError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(f"clearance recorded: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
