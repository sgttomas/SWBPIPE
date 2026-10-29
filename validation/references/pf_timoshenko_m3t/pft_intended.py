"""The census's second reference: the exact solution of the product's intended system (Agent 0, the census ruling).

K-D5's bound B certifies the product's solve against its intended system: DEL-04-01 Design
kd5-member-relative-bound.md, '"Certified" means against K-D5's intended system', whose §1 intended system is the
re-formation at p = 128 "of the stiffness and of every formed load term from its binary64 primitives". This module
solves that system exactly (pft_engine's 110-digit arithmetic) from the binary64 values the solver used.

Only the section and material operands change from pft_engine: each straight member's E, G, A, Iy, Iz, J and (on a
Timoshenko member) A_s are the resolved binary64 values of the census probe's `frames` (README "Census probe"),
taken exactly, in place of the engine's exact annulus of OD and wall. The element (kd5t_engine.local_stiffness with
EA, GJ, EIy, EIz and phi = 12 E I/(G A_s L^2)), the loads (thermal E A alpha dT from the same binary64 E and A), the
recovery and the stresses (sigma = N/A, M/Z with Z = I/r_o, T r_o/J; r_o = OD/2 of the request) are pft_engine's.
The geometry, supports and loads are the request's binary64 values, as in pft_engine.

The physical reference (pft_engine, the set's rows) differs from this one only by the section-property rounding
of the binary64 A, I, J and A_s against the annulus of the authored OD and wall; M3 judges that gap (1e-9).
"""

from __future__ import annotations

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
import pft_engine as E  # noqa: E402

K = E.K
OPERANDS = ("E", "G", "A", "Iy", "Iz", "J")


def member_id(label):
    """The published entity_ref of a probe frame's `member` label, `member:{len}:{id}:`."""
    head, _, rest = label.partition(":")
    n, _, rest = rest.partition(":")
    if head != "member" or not n.isdigit() or len(rest) != int(n) + 1 or not rest.endswith(":"):
        raise ValueError(f"not a probe member label: {label!r}")
    return rest[:-1]


class IntendedMember(E.Member):
    """pft_engine's member with the solver's binary64 section and material operands."""

    def __init__(self, spec, nodes, materials, theory, frame):
        super().__init__(spec, nodes, materials, theory)
        x = {k: frame[k] for k in OPERANDS}
        if any(type(v) is not float for v in x.values()):
            raise ValueError(f"{self.id}: a frame operand is not a binary64 number")
        if x["E"] != self.Ef or x["G"] != self.Gf:
            raise ValueError(f"{self.id}: the frame's E/G are not the request's binary64 E/G")
        Ex, Gx = K.exact(x["E"]), K.exact(x["G"])
        A, Iy, Iz, J = (K.exact(x[k]) for k in ("A", "Iy", "Iz", "J"))
        ro = self.sec["ro"]
        self.sec = dict(self.sec, A=A, I=Iz, Iy=Iy, Iz=Iz, J=J, Z=Iz / ro)
        if Iy != Iz:
            raise ValueError(f"{self.id}: Iy != Iz; the engine's single Z needs an annulus")
        self.EA, self.EI, self.GJ = Ex * A, Ex * Iz, Gx * J
        self.EIy = Ex * Iy
        if theory == E.TIMOSHENKO:
            if type(frame.get("A_s")) is not float:
                raise ValueError(f"{self.id}: a Timoshenko frame without A_s")
            As = K.exact(frame["A_s"])
            self.As_shear = As
            phi_y, phi_z = 12 * self.EIy / (Gx * As * self.L * self.L), 12 * self.EI / (Gx * As * self.L * self.L)
        else:
            if frame.get("A_s") is not None:
                raise ValueError(f"{self.id}: an Euler-Bernoulli frame with A_s")
            phi_y = phi_z = E.ZERO
        self.phi = phi_z
        self.k_local = K.local_stiffness(self.EA, self.GJ, self.EIy, self.EI, self.L, phi_y, phi_z)


def intended_reference(request, frames):
    """pft_engine.Reference of `request` (Pa moduli) with every member built from its probe frame {member id: frame}."""
    orig = E.Member
    E.Member = lambda spec, nodes, mats, theory: IntendedMember(spec, nodes, mats, theory, frames[spec["id"]])
    try:
        return E.Reference(request)
    finally:
        E.Member = orig
