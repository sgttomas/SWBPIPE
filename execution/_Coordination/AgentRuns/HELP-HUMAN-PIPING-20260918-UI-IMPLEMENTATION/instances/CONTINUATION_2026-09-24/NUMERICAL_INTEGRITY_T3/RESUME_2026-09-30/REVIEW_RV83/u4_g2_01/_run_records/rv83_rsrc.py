"""RV83 helper: print numbered lines of an installed rustdoc HTML source page.
Usage: RUSTDOC_SRC=<toolchain html/src root> python3 rsrc.py <path under src, without .html> <start> <end>
stdlib only; read-only."""
import html, os, re, sys
p = os.path.join(os.environ["RUSTDOC_SRC"], sys.argv[1] + ".html")
t = open(p, encoding="utf-8").read()
# isolate the code block, drop line-number anchors, strip tags
m = re.search(r'<code>(.*)</code>', t, re.S)
body = m.group(1)
body = re.sub(r'<a [^>]*?data-nosnippet[^>]*>\s*\d+\s*</a>', '', body)
body = re.sub(r'<span [^>]*?class="[^"]*line-number[^"]*"[^>]*>.*?</span>', '', body, flags=re.S)
body = re.sub(r'<[^>]+>', '', body)
lines = html.unescape(body).split("\n")
a, b = int(sys.argv[2]), int(sys.argv[3])
for i in range(a, min(b, len(lines)) + 1):
    print(f"{i}: {lines[i-1]}")
