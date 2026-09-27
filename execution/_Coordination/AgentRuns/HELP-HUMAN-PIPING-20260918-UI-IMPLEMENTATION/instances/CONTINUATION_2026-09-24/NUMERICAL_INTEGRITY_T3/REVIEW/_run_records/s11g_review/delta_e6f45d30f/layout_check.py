"""RV4 independent layout check of PP lib.rs: b62e40d4d against the repair head (standard library).
Usage: layout_check.py <old lib.rs> <new lib.rs>. Walks both whitespace-stripped texts; the only
permitted difference is a comma added immediately before ) ] or }."""
import re, sys
a = re.sub(r'\s+', '', open(sys.argv[1]).read()); b = re.sub(r'\s+', '', open(sys.argv[2]).read())
i = j = 0; added = 0; other = []
while i < len(a) and j < len(b):
    if a[i] == b[j]: i += 1; j += 1; continue
    if b[j] == ',' and j + 1 < len(b) and b[j + 1] in ')]}': added += 1; j += 1; continue
    other.append((i, a[i-30:i+30], b[j-30:j+30])); break
print('whitespace-stripped equal:', a == b)
print('trailing commas added before ) ] }:', added)
print('other differences:', other, '| both fully consumed:', i == len(a) and j == len(b))
