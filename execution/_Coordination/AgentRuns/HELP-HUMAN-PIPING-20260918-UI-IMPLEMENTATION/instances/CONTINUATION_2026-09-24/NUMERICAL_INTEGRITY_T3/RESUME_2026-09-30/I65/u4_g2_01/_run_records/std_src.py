"""Print numbered Rust source lines from the installed rustdoc HTML source pages.
Read-only; stdlib only. Usage: python3 std_src.py <relative path under html/src> <start> <end>
The HTML root is taken from the TOOLCHAIN_DOC environment variable (a placeholder in records)."""
import html, os, re, sys
root = os.environ["TOOLCHAIN_DOC"]
path = os.path.join(root, sys.argv[1] + ".html")
text = open(path, encoding="utf-8").read()
m = re.search(r'<pre class="rust"><code>(.*?)</code></pre>', text, re.S)
body = m.group(1) if m else text
body = re.sub(r'<a [^>]*class="src-line-numbers"[^>]*>.*?</a>', '', body, flags=re.S)
body = re.sub(r'<[^>]+>', '', body)
lines = html.unescape(body).split("\n")
start, end = int(sys.argv[2]), int(sys.argv[3])
for i in range(start, min(end, len(lines)) + 1):
    print(f"{i}: {lines[i-1]}")
