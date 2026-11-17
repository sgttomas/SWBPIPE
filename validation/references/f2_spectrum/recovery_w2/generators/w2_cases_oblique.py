"""Additive W2 case V-ARC-OBL: a realized 90-degree arc in an oblique plane (arc frame-order control).

Every number is invented. Every coordinate is dyadic (exact binary64); the derived directions are
exact rationals. Section, material, spectra, directions, damping and base rules are those of
w2_cases.py.

  t1 = (3, 4, 0)/5            the traversal tangent at B (direction of P1)
  t2 = (-12, 9, 20)/25        the traversal tangent at C (direction of P2); t1 . t2 = 0, |t2| = 1
  R  = 25/32 m = 0.78125 m    so that R t1 and R t2 are dyadic

  A (0, 0, 0)                       anchor
  B = A + (5/4) t1                  = (0.75, 1, 0)                straight P1, 1.25 m
  centre = B + R t2                 = (0.375, 1.28125, 0.625)
  C = B + R (t1 + t2)               = (0.84375, 1.90625, 0.625)   realized arc BEND1, 90 degrees
  D = C + (25/16) t2                = (0.09375, 2.46875, 1.875)   straight P2, 1.5625 m; anchor

The straights are tangent to the arc at both ends. The arc bows toward t1 - t2, which is normal
to the chord t1 + t2; y_reference is its integer multiple (27, 11, -20). The end-tangent frames
are x = t (traversal tangent), y toward the centre, z = x cross y:
  end B: x = t1, y = t2, z = (16, -12, 15)/25;   end C: x = t2, y = -t1, z = (16, -12, 15)/25.
None is a signed permutation of the global axes.
"""

NODES = {
    "A": ["0", "0", "0"],
    "B": ["0.75", "1", "0"],
    "C": ["0.84375", "1.90625", "0.625"],
    "D": ["0.09375", "2.46875", "1.875"],
}

PIPE_MEMBERS = {
    "P1": {"kind": "straight", "i": "A", "j": "B", "y_reference": ["0", "0", "1"]},
    "BEND1": {"kind": "arc", "i": "B", "j": "C", "radius_m": "0.78125", "flexibility_factor": "4.25",
              "y_reference": ["27", "11", "-20"]},
    "P2": {"kind": "straight", "i": "C", "j": "D", "y_reference": ["0", "0", "1"]},
}

VARIANTS = {
    "V-ARC-OBL": {
        "nodes": ["A", "B", "C", "D"],
        "members": ["P1", "BEND1", "P2"],
        "connector": False,
        "rods": False,
        "anchors": ["A", "D"],
        "purpose": ("arc frame-order control: a realized arc in an oblique plane whose end-tangent frames are "
                    "not signed permutations of the global axes, anchored at A and D"),
    },
}

REQUESTS = {
    "V-ARC-OBL": {"cutoff_Hz": "320", "rigid_frequency_Hz": "250"},
}

# Exact derived directions (rationals, as numerator lists over a common denominator).
T1 = ([3, 4, 0], 5)
T2 = ([-12, 9, 20], 25)
