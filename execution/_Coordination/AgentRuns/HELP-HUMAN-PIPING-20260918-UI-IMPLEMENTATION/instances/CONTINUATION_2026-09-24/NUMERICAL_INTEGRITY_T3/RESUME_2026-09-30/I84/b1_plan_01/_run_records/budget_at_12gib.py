"""I84 B1-P: I82's priced points read against M = 12 GiB and a 5 % text-error budget.

Read-only arithmetic on I82's sealed `report.json` (R/I82/b1_cap_study_01/_run_records/).
Usage: python3 budget_at_12gib.py <report.json>
These are I82's emulated prices, not B1's in-build G5; B1's G5 replaces them.
"""
import json
import sys

points = json.load(open(sys.argv[1]))["points"]
M12 = 12 * 2**30  # 12,884,901,888 B (owner, 2026-10-07)
print("M = 12 GiB =", M12, "B; 0.9 M =", int(0.9 * M12))
for key in ["d1_c1", "d1_c2", "d1_c3", "d1_c4", "t_c4_k20_l128", "t_c4_k16_l128", "t_c4_m8", "t_c3_k16_l64"]:
    for mode in ("dense", "sparse"):
        p = points[key][mode]
        e, t = p["E_mov_plus_R"], p["TAV_W"]
        margin = 0.9 * M12 - e
        m5 = (e + 0.05 * t) / 0.9
        print(f"{key:15s} {mode:6s} E+R {e:>14,d}  TAV_W {t:>14,d}  margin@12GiB {int(margin):>15,d}"
              f"  budget@12GiB {margin / t:7.3f}  M for 5% {int(m5):>14,d} ({m5 / 2**30:.2f} GiB)")
