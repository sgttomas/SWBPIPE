"""Read-only, hash-pinned import of the closed_form generator modules.

The benchmark pack reuses the closed_form package's numerics, table rules,
continuum engine, finite elements and beam helpers. Each module file is
hashed before import and must equal the pinned SHA-256; otherwise the run
stops. Nothing in closed_form is written. The sibling discrete package's
c8_pipe_x.json is read only for one cross-check and is pinned the same way.
"""

import hashlib
import sys
from pathlib import Path

PACK = Path(__file__).resolve().parent.parent
CF_GEN = PACK.parent / "closed_form" / "generators"
C8 = PACK.parent / "discrete" / "c8_pipe_x.json"

PINNED = {
    "numerics.py": "06a466936f7320ef44ea532f81ef09e2703bac3a9293a4dc854e8dc90de53ec2",
    "rules.py": "067995d7d3f19a0e485a4c347fab3cc52da54acf8f04c98bb2b1f414d114520d",
    "continuum.py": "5c1acfe10bbd565c8db368ac3499d5839d2e1af7baa98213acdb7756a4ea0e7d",
    "fe.py": "23baf489faeeda083a4000319da79310ade1eefd5e8ab5e744f76c9ba107737b",
    "beams.py": "362f29b7fca32c9b1f8dafb100ab107b36fcef76856a485b43ef04e690ff39d8",
}
C8_SHA256 = "61a57007422d3419ca80444ca5d7a156d1211e057a968e321ca74eda37b77e79"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load():
    for name, h in PINNED.items():
        got = sha(CF_GEN / name)
        if got != h:
            raise SystemExit(f"pinned module changed: closed_form/generators/{name} {got} != {h}")
    if str(CF_GEN) not in sys.path:
        sys.path.append(str(CF_GEN))   # appended: the pack's own modules take precedence by name
    import numerics, rules, continuum, fe, beams  # noqa: E401,F401
    return numerics, rules, continuum, fe, beams


def c8_bytes():
    b = C8.read_bytes()
    if hashlib.sha256(b).hexdigest() != C8_SHA256:
        raise SystemExit("pinned sibling file changed: discrete/c8_pipe_x.json")
    return b
