"""I104 SQ G5 (diagnostic only): make text_budget_looplog.py, a copy of a chain's text_budget.py that also
writes, to $TB_LOOPLOG, every loop header stack_product evaluates, with its rule and value and the context
(caller -> callee edge, or text site). The copy's printed output is unchanged.
Usage: mk_looplog.py <chain text_budget.py> <out py>"""
import sys
src, dst = sys.argv[1:3]
t = open(src).read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old
    t = t.replace(old, new)
rep("def stack_product(stack, note):\n    hs = list(stack)",
    "_LOG = []\n_CTX = [None]\ndef stack_product(stack, note):\n    hs = list(stack)\n    for _h in hs:\n        _b, _r = loop_bound(_h); _LOG.append((_CTX[0], _h, _r, _b))")
rep("    for v, w in cedges[c]:\n        if w not in anc:", "    for v, w in cedges[c]:\n        _CTX[0] = 'call ' + short(v) + ' -> ' + short(w)\n        if w not in anc:")
rep("def site_mult(f, line, fk, loops, key):\n", "def site_mult(f, line, fk, loops, key):\n    _CTX[0] = 'site ' + short(fk or '') + ' @' + f.split('/src/')[-1] + ':' + str(line)\n")
rep("print(json.dumps(out, indent=1))", "print(json.dumps(out, indent=1))\nif os.environ.get('TB_LOOPLOG'):\n    json.dump(_LOG, open(os.environ['TB_LOOPLOG'], 'w'))")
open(dst, "w").write(t)
