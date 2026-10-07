import json, sys
def tests(p):
    j = json.load(open(p)); out = {}
    for f in j["testResults"]:
        name = f["name"].split("/apps/desktop/")[-1]
        for t in f["assertionResults"]:
            out[f"{name} :: {t['fullName']}"] = t["status"]
    return j, out
jb, b = tests(sys.argv[1]); jc, c = tests(sys.argv[2])
print("base: files", jb["numTotalTestSuites"], "tests", jb["numTotalTests"], "passed", jb["numPassedTests"], "failed", jb["numFailedTests"], "| head: files", jc["numTotalTestSuites"], "tests", jc["numTotalTests"], "passed", jc["numPassedTests"], "failed", jc["numFailedTests"])
print("test files base/head:", len(jb["testResults"]), len(jc["testResults"]))
rem = sorted(set(b) - set(c)); add = sorted(set(c) - set(b))
print("removed (%d):" % len(rem)); [print("  -", x) for x in rem]
print("added (%d):" % len(add)); [print("  +", x) for x in add]
print("changed outcome:", [k for k in set(b) & set(c) if b[k] != c[k]])
