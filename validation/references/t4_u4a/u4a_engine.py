"""T4-U4a independent arc-frame engine (standard-library Python only).

Unit-load (flexibility) method over a line of straight members and circular
arcs, cantilevered from its first node, in 90-digit decimal arithmetic. It
imports no product code and reads no product file. Section actions are
closed-form statics (the resultant of every load beyond the cut, including the
redundant reaction of an anchored last node); displacements and the redundants
come from the virtual-work integrals, evaluated by 48-point Gauss-Legendre
quadrature on each member (exact on straights; checked against 64 points on
arcs, see `precision_check`).

Formulation (invented models; small strain, linear elastic, Euler-Bernoulli,
no shear deformation):

- strain energy density N^2/(2EA) + T^2/(2GJ) + k |M_perp|^2/(2EI), with the
  user's flexibility factor k on arcs (one k for both bending planes) and
  k = 1 on straights; uniform axial free strain eps0 per member;
- pressure (wall formulation): the wetted-wall load (p Ai / R) along the outward
  normal per unit arc length on every arc, terminal caps -p Ai t_A at the first
  node and +p Ai t_D at the last node where the terminal transfers to the wall,
  the kink force p Ai (t_in - t_out) at interior nodes, and the Poisson strain
  -2 nu p Ai / (E As) added to eps0; the section's wall force is N_w and its
  elastic (effective) force is S = N_w - p Ai; shear and moments are common.

D-5 section bases (T4-U6; model key section_bases = "d5", used for pressure-1): EA, EI, GJ and the
Poisson strain's As use the nominal wall; p Ai uses the pressure bore OD/2 - ((t - c_m) - c_a); the
generator recovers stresses on the nominal and on the reduced section. Without the key (preview-physics-1)
one section, wall - mill tolerance, serves everything, as before.

Arc geometry is the product's documented rule (bend_plane_orientation): from
the authored chord d = x_j - x_i, the authored radius R and the pipe
y_reference projected normal to the chord (n_hat), the arc bows toward
+n_hat; its centre is c = (x_i + x_j)/2 - sqrt(R^2 - (L/2)^2) n_hat.

Frames (authored direction i -> j):

- chord frame: x = d/L, y = n_hat, z = x cross y (the straight element frame
  of the replaced span);
- tangent frame at arc angle psi: x = arc tangent toward j, y toward the arc
  centre, z = the bend-plane normal (x_i - c) cross (x_j - c), normalised;
- straight frame: x = d/L, y = y_reference projected normal to x, z = x cross y.

Sign conventions: cut rows are the j-side section action (the action of the
authored-j side on the authored-i side; positive N is tension); end rows are
node-on-element actions, end_i = -(cut at fraction 0) and end_j = +(cut at
fraction 1), each in the frame at that end.
"""

from __future__ import annotations

from decimal import Decimal, getcontext
from fractions import Fraction

getcontext().prec = 90
D = Decimal
ZERO = D(0)
ONE = D(1)
TWO = D(2)


# ----------------------------------------------------------------- numbers

def exact(value) -> Decimal:
    """The exact value of a binary64 input (a float or its round-trip string)."""
    return D(float(value))


def dec(text) -> Decimal:
    """An exact decimal input (an invented rational target such as 0.3)."""
    return D(str(text))


def _atan_small(x: Decimal) -> Decimal:
    total, power, n, x2 = ZERO, x, 1, x * x
    eps = D(10) ** (-(getcontext().prec + 5))
    while True:
        term = power / n
        if abs(term) < eps:
            return total
        total += term if (n // 2) % 2 == 0 else -term
        power *= x2
        n += 2


def atan(x: Decimal) -> Decimal:
    if x < 0:
        return -atan(-x)
    if x > 1:
        return PI / 2 - atan(ONE / x)
    halvings = 0
    while x > D("0.05"):
        x = x / (ONE + (ONE + x * x).sqrt())
        halvings += 1
    return _atan_small(x) * (2 ** halvings)


def _pi() -> Decimal:
    return 16 * _atan_small(ONE / 5) - 4 * _atan_small(ONE / 239)


PI = _pi()


def sin_cos(x: Decimal) -> tuple[Decimal, Decimal]:
    two_pi = 2 * PI
    k = (x / two_pi).to_integral_value()
    r = x - k * two_pi
    eps = D(10) ** (-(getcontext().prec + 5))
    s, c, term_s, term_c, n, r2 = r, ONE, r, ONE, 1, r * r
    while True:
        term_s = -term_s * r2 / ((2 * n) * (2 * n + 1))
        term_c = -term_c * r2 / ((2 * n - 1) * (2 * n))
        s += term_s
        c += term_c
        if abs(term_s) < eps and abs(term_c) < eps:
            return s, c
        n += 1


# ----------------------------------------------------------------- vectors

def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def scale(a, s):
    return (a[0] * s, a[1] * s, a[2] * s)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    return dot(a, a).sqrt()


def unit(a):
    return scale(a, ONE / norm(a))


VZERO = (ZERO, ZERO, ZERO)


def annulus(od: Decimal, wall: Decimal) -> dict:
    """One section basis: OD and its own wall (the same formulas and order as the single section)."""
    idia = od - 2 * wall
    I = PI * (od ** 4 - idia ** 4) / 64
    return {"wall": wall, "ri": od / 2 - wall, "As": PI * (od * od - idia * idia) / 4, "Ai": PI * idia * idia / 4,
            "I": I, "J": 2 * I, "Z": I / (od / 2)}


# -------------------------------------------------------------- quadrature

def gauss_legendre(n: int) -> list[tuple[Decimal, Decimal]]:
    """Nodes and weights on [0, 1] by Newton iteration on P_n."""
    out = []
    eps = D(10) ** (-(getcontext().prec - 5))
    for i in range(1, n + 1):
        _, c = sin_cos(PI * (D(i) - D("0.25")) / (D(n) + D("0.5")))
        x = c
        for _ in range(100):
            p0, p1 = ONE, x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < eps:
                break
        p0, p1 = ONE, x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1)
        w = 2 / ((1 - x * x) * dp * dp)
        out.append(((1 - x) / 2, w / 2))
    return out


_GL_CACHE: dict[int, list] = {}


def gl(n: int):
    if n not in _GL_CACHE:
        _GL_CACHE[n] = gauss_legendre(n)
    return _GL_CACHE[n]


# ---------------------------------------------------------------- geometry

class Member:
    """One straight member or circular arc, with authored and traversal sense."""

    def __init__(self, spec, positions, forward: bool):
        self.id = spec["pipe"]
        self.kind = spec["kind"]
        self.authored_from = spec["authored_from"]
        self.authored_to = spec["authored_to"]
        self.forward = forward  # traversal runs authored i -> j
        self.xi = positions[self.authored_from]
        self.xj = positions[self.authored_to]
        self.y_reference = tuple(exact(v) for v in spec["y_reference"])
        self.k = dec(spec.get("k", "1"))
        d = sub(self.xj, self.xi)
        self.chord_length = norm(d)
        dh = scale(d, ONE / self.chord_length)
        yp = sub(self.y_reference, scale(dh, dot(self.y_reference, dh)))
        nh = unit(yp)
        self.chord_axes = (dh, nh, cross(dh, nh))
        if self.kind == "arc":
            self.radius = dec(spec["R"])
            s = self.chord_length / (2 * self.radius)
            c = (ONE - s * s).sqrt()
            self.half_sin, self.half_cos = s, c
            self.phi = 2 * atan(s / c)
            mid = scale(add(self.xi, self.xj), D("0.5"))
            self.centre = sub(mid, scale(nh, self.radius * c))
            self.ex = scale(sub(self.xi, self.centre), ONE / self.radius)
            self.normal = unit(cross(sub(self.xi, self.centre), sub(self.xj, self.centre)))
            self.ey = cross(self.normal, self.ex)
            self.length = self.radius * self.phi
            self.sin_phi, self.cos_phi = sin_cos(self.phi)
        else:
            self.length = self.chord_length
            self.straight_axes = self.chord_axes

    # authored parameter u in [0, 1]
    def angle(self, u: Decimal) -> Decimal:
        return self.phi * u

    def point(self, u: Decimal):
        if self.kind == "arc":
            s, c = sin_cos(self.angle(u))
            return add(self.centre, scale(add(scale(self.ex, c), scale(self.ey, s)), self.radius))
        return add(self.xi, scale(sub(self.xj, self.xi), u))

    def axes(self, u: Decimal):
        """Authored local frame (x, y, z) at parameter u."""
        if self.kind == "arc":
            s, c = sin_cos(self.angle(u))
            x = add(scale(self.ex, -s), scale(self.ey, c))
            y = scale(add(scale(self.ex, c), scale(self.ey, s)), -ONE)
            return (x, y, self.normal)
        return self.straight_axes

    def traversal_tangent(self, u: Decimal):
        x = self.axes(u)[0]
        return x if self.forward else scale(x, -ONE)

    # loads over the authored parameter interval [ua, ub] (ua <= ub)
    def uniform_resultant(self, w, ua: Decimal, ub: Decimal, about):
        """Force and moment about `about` of a uniform load w per unit length."""
        if self.kind == "arc":
            pa, pb = self.angle(ua), self.angle(ub)
            sa, ca = sin_cos(pa)
            sb, cb = sin_cos(pb)
            span = pb - pa
            force = scale(w, self.radius * span)
            integral = add(scale(self.centre, span),
                           scale(add(scale(self.ex, sb - sa), scale(self.ey, ca - cb)), self.radius))
            lever = scale(sub(integral, scale(about, span)), self.radius)
            return force, cross(lever, w)
        qa, qb = self.point(ua), self.point(ub)
        length = norm(sub(qb, qa))
        force = scale(w, length)
        centroid = scale(add(qa, qb), D("0.5"))
        return force, cross(sub(centroid, about), force)

    def wall_pressure_resultant(self, p_ai: Decimal, ua: Decimal, ub: Decimal, about):
        """The wetted-wall load (p Ai / R) outward per unit arc length."""
        if self.kind != "arc":
            return VZERO, VZERO
        sa, ca = sin_cos(self.angle(ua))
        sb, cb = sin_cos(self.angle(ub))
        force = scale(add(scale(self.ex, sb - sa), scale(self.ey, ca - cb)), p_ai)
        return force, cross(sub(self.centre, about), force)


class Line:
    """A traversal-ordered line, anchored at its first node (and optionally its last)."""

    def __init__(self, model):
        self.model = model
        self.positions = {n: tuple(exact(v) for v in xyz) for n, xyz in model["nodes"].items()}
        self.order = model["traversal_order"]
        self.members = []
        for k, spec in enumerate(model["members"]):
            a, b = self.order[k], self.order[k + 1]
            if (spec["authored_from"], spec["authored_to"]) == (a, b):
                forward = True
            elif (spec["authored_from"], spec["authored_to"]) == (b, a):
                forward = False
            else:
                raise ValueError(f"member {spec['pipe']} does not join {a} and {b}")
            self.members.append(Member(spec, self.positions, forward))
        sec = model["section"]
        od = dec(sec["outside_diameter"])
        self.bases = None
        if model.get("section_bases") == "d5":
            # D-5 (DEL-05-03 Design section 1): stiffness (and the As of the Poisson strain) on the nominal
            # wall t; Ai on the pressure bore OD/2 - t_r with t_r = (t - c_m) - c_a; stresses on both sections.
            t = dec(sec["wall_thickness"])
            t_r = (t - dec(sec.get("mill_tolerance", "0"))) - dec(sec.get("corrosion_allowance", "0"))
            self.bases = {"nominal_section": annulus(od, t), "reduced_section": annulus(od, t_r)}
            nominal, reduced = self.bases["nominal_section"], self.bases["reduced_section"]
            self.ro = od / 2
            self.As, self.I, self.J, self.Z = nominal["As"], nominal["I"], nominal["J"], nominal["Z"]
            self.Ai, self.ri_bore = reduced["Ai"], reduced["ri"]
        else:
            wall = dec(sec["wall_thickness"]) - dec(sec.get("mill_tolerance", "0"))
            idia = od - 2 * wall
            self.ro = od / 2
            self.As = PI * (od * od - idia * idia) / 4
            self.Ai = PI * idia * idia / 4
            self.I = PI * (od ** 4 - idia ** 4) / 64
            self.J = 2 * self.I
            self.Z = self.I / self.ro
        mat = model["material"]
        self.E = dec(mat["E"])
        self.G = dec(mat["G"]) if "G" in mat else self.E / (2 * (ONE + dec(mat["nu"])))
        self.nu = dec(mat["nu"]) if "nu" in mat else None
        self.alpha = dec(mat.get("alpha", "0"))
        loads = model.get("loads", {})
        self.p = dec(loads.get("p", "0"))
        self.p_ai = self.p * self.Ai
        self.terminal_a = loads.get("terminal_A", "transfers_to_wall")
        self.terminal_d = loads.get("terminal_D", "transfers_to_wall")
        self.delta_t = {m.id: dec(loads.get("delta_T", {}).get(m.id, loads.get("delta_T_all", "0"))) for m in self.members}
        self.uniform = {m.id: [tuple(dec(v) for v in w) for w in loads.get("uniform", {}).get(m.id, [])] for m in self.members}
        for m in self.members:
            self.uniform[m.id] += [tuple(dec(v) for v in w) for w in loads.get("uniform_all", [])]
        self.nodal = {n: (tuple(dec(v) for v in fm[0]), tuple(dec(v) for v in fm[1])) for n, fm in loads.get("nodal", {}).items()}
        self.anchor_d = model.get("anchor_last", False)
        # Restrained DOFs of the last node (0-2 translations, 3-5 rotations) and their prescribed values:
        # a full anchor, or an active one-way contact closed at its gap (u = -gap along its DOF).
        restraint = model.get("last_restraint")
        if restraint is None and self.anchor_d:
            restraint = {"dofs": [0, 1, 2, 3, 4, 5], "prescribed": ["0"] * 6}
        self.last_dofs = list(restraint["dofs"]) if restraint else []
        self.last_prescribed = [dec(v) for v in restraint["prescribed"]] if restraint else []
        self.redundant = (VZERO, VZERO)
        self._cache = {}
        self.eps0 = {}
        for m in self.members:
            e = self.alpha * self.delta_t[m.id]
            if self.p != 0:
                e += -2 * self.nu * self.p_ai / (self.E * self.As)
            self.eps0[m.id] = e

    # --- loads beyond a cut, in traversal order
    def traversal_interval(self, m: Member, u: Decimal):
        """Authored parameter interval of the part of m beyond the cut at u."""
        return (u, ONE) if m.forward else (ZERO, u)

    def node_loads(self, index: int, about):
        """Point loads at traversal node `index` (its kink, caps and nodal loads)."""
        node = self.order[index]
        force, moment = VZERO, VZERO
        x = self.positions[node]
        if self.p != 0:
            if index == 0 and self.terminal_a == "transfers_to_wall":
                t = self.members[0].traversal_tangent(ZERO if self.members[0].forward else ONE)
                force = add(force, scale(t, -self.p_ai))
            elif index == len(self.order) - 1 and self.terminal_d == "transfers_to_wall":
                m = self.members[-1]
                t = m.traversal_tangent(ONE if m.forward else ZERO)
                force = add(force, scale(t, self.p_ai))
            elif 0 < index < len(self.order) - 1:
                m_in, m_out = self.members[index - 1], self.members[index]
                t_in = m_in.traversal_tangent(ONE if m_in.forward else ZERO)
                t_out = m_out.traversal_tangent(ZERO if m_out.forward else ONE)
                force = add(force, scale(sub(t_in, t_out), self.p_ai))
        if node in self.nodal:
            f, mm = self.nodal[node]
            force = add(force, f)
            moment = add(moment, mm)
        if index == len(self.order) - 1:
            force = add(force, self.redundant[0])
            moment = add(moment, self.redundant[1])
        return force, add(moment, cross(sub(x, about), force))

    def member_loads(self, m: Member, ua: Decimal, ub: Decimal, about):
        force, moment = VZERO, VZERO
        for w in self.uniform[m.id]:
            f, mm = m.uniform_resultant(w, ua, ub, about)
            force, moment = add(force, f), add(moment, mm)
        if self.p != 0:
            f, mm = m.wall_pressure_resultant(self.p_ai, ua, ub, about)
            force, moment = add(force, f), add(moment, mm)
        return force, moment

    def beyond(self, k: int, u: Decimal):
        """Resultant (about the cut point) of every load beyond the cut at u on member k."""
        key = (k, u, self.redundant)
        if key not in self._cache:
            self._cache[key] = self._beyond(k, u)
        return self._cache[key]

    def _beyond(self, k: int, u: Decimal):
        m = self.members[k]
        about = m.point(u)
        ua, ub = self.traversal_interval(m, u)
        force, moment = self.member_loads(m, ua, ub, about)
        for j in range(k + 1, len(self.members)):
            f, mm = self.member_loads(self.members[j], ZERO, ONE, about)
            force, moment = add(force, f), add(moment, mm)
        for index in range(k + 1, len(self.order)):
            f, mm = self.node_loads(index, about)
            force, moment = add(force, f), add(moment, mm)
        return force, moment

    # --- virtual work
    def _integrand(self, m: Member, u, actual, unit_load, with_strain: bool):
        t = m.traversal_tangent(u)
        (F, M), (f, mu) = actual, unit_load
        N, n = dot(F, t), dot(f, t)
        T, tu = dot(M, t), dot(mu, t)
        bend = dot(M, mu) - T * tu
        value = N * n / (self.E * self.As) + T * tu / (self.G * self.J) + m.k * bend / (self.E * self.I)
        if with_strain:
            value += n * self.eps0[m.id]
        return value

    def _unit_actions(self, node_index: int, dof: int, point):
        """Section action (beyond) at `point` of a unit load at a node."""
        x = self.positions[self.order[node_index]]
        vec = tuple(ONE if i == dof % 3 else ZERO for i in range(3))
        if dof < 3:
            return vec, cross(sub(x, point), vec)
        return VZERO, vec

    def virtual_work(self, node_index: int, n_points: int = 48, with_strain=True, actual_fn=None):
        """Six displacement components (translations, rotations) of a node."""
        out = [ZERO] * 6
        for k, m in enumerate(self.members[:node_index]):
            for (x, w) in gl(n_points):
                u = x
                actual = actual_fn(k, u) if actual_fn else self.beyond(k, u)
                point = m.point(u)
                for dof in range(6):
                    out[dof] += w * m.length * self._integrand(m, u, actual, self._unit_actions(node_index, dof, point), with_strain)
        return out

    def solve(self, n_points: int = 48):
        if self.last_dofs:
            last = len(self.order) - 1
            self.redundant = (VZERO, VZERO)
            delta0 = self.virtual_work(last, n_points)
            dofs = self.last_dofs
            flex = [[ZERO] * len(dofs) for _ in dofs]
            for j, col in enumerate(dofs):
                def unit_actual(k, u, col=col):
                    return self._unit_actions(last, col, self.members[k].point(u))
                column = self.virtual_work(last, n_points, with_strain=False, actual_fn=unit_actual)
                for i, row in enumerate(dofs):
                    flex[i][j] = column[row]
            r = solve_linear(flex, [self.last_prescribed[i] - delta0[row] for i, row in enumerate(dofs)])
            full = [ZERO] * 6
            for i, dof in enumerate(dofs):
                full[dof] = r[i]
            self.redundant = (tuple(full[:3]), tuple(full[3:]))
        return self

    # --- outputs
    def cut(self, k: int, u: Decimal):
        """Authored j-side section action at u: global (F, M) and local rows."""
        m = self.members[k]
        F, M = self.beyond(k, u)
        if not m.forward:
            F, M = scale(F, -ONE), scale(M, -ONE)
        return F, M

    def support_a(self):
        """Support-on-pipe action at the first node (global; moment about that node)."""
        F, M = self.beyond(0, ZERO if self.members[0].forward else ONE)
        f0, m0 = self.node_loads(0, self.positions[self.order[0]])
        return scale(add(F, f0), -ONE), scale(add(M, m0), -ONE)


def solve_linear(a, b):
    """Gaussian elimination with partial pivoting (Decimal)."""
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(m[r][col]))
        m[col], m[piv] = m[piv], m[col]
        for r in range(n):
            if r != col and m[r][col] != 0:
                f = m[r][col] / m[col][col]
                for c in range(col, n + 1):
                    m[r][c] -= f * m[col][c]
    return [m[i][n] / m[i][i] for i in range(n)]


def components(F, M, axes):
    x, y, z = axes
    return [dot(F, x), dot(F, y), dot(F, z), dot(M, x), dot(M, y), dot(M, z)]


FRACTIONS = [("end_i", D(0)), ("quarter_1", D("0.25")), ("midspan", D("0.5")), ("quarter_3", D("0.75")), ("end_j", D(1))]
