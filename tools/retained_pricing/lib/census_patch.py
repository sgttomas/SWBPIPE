"""SQ2: I95's census substitution (R/I95/b3_s_01/_run_records/b3s_exact_chain.py section 10), unchanged in content:
each per-case preview-tree Value form becomes the componentwise maximum of the preview tree's facts and the exact
evidence's facts at (n, m, g). The census is I95's b3s_census.py re-run on the c3cf970ad3 snapshot (identical output)."""
def apply(dst, census, patch):
    parts = census["exact_evidence_case_parts"]
    FIELDS = ("arr", "obj", "ent", "strb", "keyb", "nums")
    def coef(k):
        return (parts["rest"][k], parts["per_pipe"][k] + parts["per_unavailable_pipe"][k], 6 * parts["per_dof"][k] + parts["per_node"][k])
    helper = ("\n# I95 B3-S census: per-case evidence facts = max(preview tree, exact evidence) at (n, m, g)\n"
              "_EXC = " + repr({k: coef(k) for k in FIELDS}) + "\n"
              "def _exmax(n, m, g):\n"
              "    pv = dict(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,\n"
              "              strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),\n"
              "              keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m)\n"
              "    return {k: max(pv[k], _EXC[k][0] + _EXC[k][1] * m + _EXC[k][2] * n) for k in pv}\n")
    patch("ordinary_caps.py", '''PREVIEW = value_tree(3 * m + 2 * g, 3 + m + g, 9 + 15 * m + 2 * g,
                     m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
                     (9 + 15 * m + 2 * g) * 40)''', helper + '''_PX = _exmax(n, m, g)
PREVIEW = value_tree(_PX["arr"], _PX["obj"], _PX["ent"], _PX["strb"], _PX["keyb"])   # I95: max(preview, exact)''')
    patch("g4_caps.py", '''       + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
            strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64), keyb=(9 + 15 * m + 2 * g) * 40,
            nums=10 * m, depth=6).scale(CC)''', '''       + VF(depth=7, **_exmax(n, m, g)).scale(CC)                      # I95: max(preview, exact)''')
    patch("g4_caps.py", '''rows = {}
# =========================================================================== envelope facts''', helper + '''rows = {}
# =========================================================================== envelope facts''')
    patch("g4_caps.py", '''PREVIEW_T = (s("Value") * (3 * m + 2 * g) + X({"Node(String,Value)": 3 + m + g + (9 + 15 * m + 2 * g) // 5})
             + B(m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64) + (9 + 15 * m + 2 * g) * 40))''',
          '''_PX = _exmax(n, m, g)                                                                # I95: max(preview, exact)
PREVIEW_T = (s("Value") * _PX["arr"] + X({"Node(String,Value)": _PX["obj"] + _PX["ent"] // 5}) + B(_PX["strb"] + _PX["keyb"]))''')
    patch("t25_g4.py", '''          + VF(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
               strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
               keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m, depth=6).scale(CC)''', '''          + VF(depth=7, **_exmax(n, m, g)).scale(CC)                 # I95: max(preview, exact)''')
    patch("t25_g4.py", '''# ---------------------------------------------------------------- per-invocation finalize''',
          helper + '''# ---------------------------------------------------------------- per-invocation finalize''')
    # U6-RP (T4-U6, D-5): a census taken under D-5 states the route's rows per member, measured on every committed
    # empty-region exact case (census_exact.py); the route's P_final and R0 take it in place of 51 at the caps.
    rpm = census.get("rows_per_member", 51)
    # W7-RP (W-7, retained-pricing-rules.md §8.20): the twelve global end rows per member the ordinary pressure-2
    # envelope carries and the census's pressure-source-1 fixtures omit (`global_end_rows_published`).
    w7 = census.get("global_end_rows_per_member", 0)
    rpm += w7
    if rpm != 51:
        why = (f"   # U6-RP: route E's census, {rpm} rows per member" if not w7 else
               f"   # U6-RP/W7-RP: route E's census, {rpm - w7} rows per member, + {w7} global end rows")
        patch("g3lib.py", "P=7 * n + 51 * m + 8 * g + 3, R0=7 * n + 51 * m + g + 3,",
              f"P=7 * n + {rpm} * m + 8 * g + 3, R0=7 * n + {rpm} * m + g + 3,")
        patch("g4_caps.py", "P = 7 * n + 51 * m + 8 * g + 3                     # P_final",
              f"P = 7 * n + {rpm} * m + 8 * g + 3                     # P_final" + why)
        patch("ordinary_caps.py", "    mats = 8; pts = 16                                # model + request materials; points each\n"
              "    P = 7 * n + 51 * m + 8 * g + 3; R0 = 7 * n + 51 * m + g + 3",
              "    mats = 8; pts = 16                                # model + request materials; points each\n"
              f"    P = 7 * n + {rpm} * m + 8 * g + 3; R0 = 7 * n + {rpm} * m + g + 3" + why)
        patch("producer_caps.py", "P_final = 7 * n + 51 * m + 8 * g + 3",
              f"P_final = 7 * n + ({rpm} if which == \"caps\" else 51) * m + 8 * g + 3" + why)
        patch("t25_g4.py", "P = 7 * n + 51 * m + 8 * g + 3\nR0 = 7 * n + 51 * m + g + 3",
              f"P = 7 * n + {rpm} * m + 8 * g + 3" + why + f"\nR0 = 7 * n + {rpm} * m + g + 3")
    return dict({"per_case_evidence_coefficients": {k: coef(k) for k in FIELDS}, "at_caps": census["componentwise_max"]},
                **({"rows_per_member": rpm} if rpm != 51 else {}))
