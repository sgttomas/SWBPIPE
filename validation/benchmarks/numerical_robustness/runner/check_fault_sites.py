#!/usr/bin/env python3
"""V-K A2: FK's fault sites are compiled out of every product build.

Standard library and the git CLI only. For every FK file V-K changed against a
base revision, removes each item or statement gated by
`#[cfg(any(test, feature = "mutation-controls"))]` (up to its `;` at depth 0,
or the `}` that closes an `if` block), then compares the result with the base
file. The only differences allowed are added comment lines. So, with the
feature off and outside `cfg(test)`, FK's code is the base code, token for
token apart from comments (plus the new file `retained/seeded.rs`, itself
gated by its `mod` line, and the `[features]` table of FK's Cargo.toml).

With `--allow-commit <rev>`, the lines that commit's own patch removed and
added (for example K6b's A0 export, on the V-K branch but not yet on main) are
also allowed, each at most as many times as the patch has it: the base can
then be main after a merge, and every other difference is still reported.

Usage: python3 check_fault_sites.py <base revision> [--allow-commit <rev>]   (from the repository)
"""
import collections
import difflib
import subprocess
import sys
from pathlib import Path

ATTR = '#[cfg(any(test, feature = "mutation-controls"))]'
FK = 'projects/chirality-piping/core/solver/frame_kernel'


def strip_gated(text):
    out, i = [], 0
    while True:
        j = text.find(ATTR, i)
        if j < 0:
            out.append(text[i:])
            return ''.join(out)
        # Drop the attribute's own indentation on its line.
        line_start = text.rfind('\n', 0, j) + 1
        out.append(text[i:line_start])
        k = j + len(ATTR)
        depth, seen_block, is_if = 0, False, None
        while k < len(text):
            ch = text[k]
            if is_if is None and not ch.isspace():
                is_if = text.startswith('if ', k)
            if ch in '([{':
                depth += 1
                if ch == '{' and depth == 1:
                    seen_block = True
            elif ch in ')]}':
                depth -= 1
                if ch == '}' and depth == 0 and is_if and seen_block:
                    k += 1
                    break
            elif ch == ';' and depth == 0:
                k += 1
                break
            k += 1
        # Consume the rest of the line (a newline).
        if text.startswith('\n', k):
            k += 1
        i = k


def patch_lines(root, rev, path):
    """(removed, added) line multisets of `rev`'s own patch to `path`."""
    diff = subprocess.check_output(['git', 'diff', '-U0', rev + '^', rev, '--', path], cwd=root, text=True)
    removed, added = collections.Counter(), collections.Counter()
    for line in diff.splitlines():
        if line.startswith('---') or line.startswith('+++'):
            continue
        if line.startswith('-'):
            removed[line[1:]] += 1
        elif line.startswith('+'):
            added[line[1:]] += 1
    return removed, added


def main():
    base = sys.argv[1]
    allow = sys.argv[sys.argv.index('--allow-commit') + 1] if '--allow-commit' in sys.argv else None
    root = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
    changed = subprocess.check_output(['git', 'diff', '--name-only', base, '--', FK + '/src'],
                                      cwd=root, text=True).split()
    problems = 0
    for path in changed:
        new = (root / path).read_text()
        if path.endswith('retained/seeded.rs'):
            continue
        old = subprocess.check_output(['git', 'show', '%s:%s' % (base, path)], cwd=root, text=True)
        stripped = strip_gated(new)
        extra = [line for line in difflib.ndiff(old.splitlines(), stripped.splitlines())
                 if line[:2] in ('+ ', '- ')]
        bad = [line for line in extra if not (line.startswith('+ ') and line[2:].strip().startswith('//'))]
        comments = len(extra) - len(bad)
        allowed = 0
        if allow:
            removed, added = patch_lines(root, allow, path)
            rest = []
            for line in bad:
                pool = removed if line.startswith('- ') else added
                if pool[line[2:]] > 0:
                    pool[line[2:]] -= 1
                    allowed += 1
                else:
                    rest.append(line)
            bad = rest
        gated = new.count(ATTR)
        print('%s: %d gated item(s); %d added comment line(s); %d line(s) of %s; %d other difference(s)'
              % (path, gated, comments, allowed, allow or '-', len(bad)))
        for line in bad:
            print('   ', line)
        problems += len(bad)
    untracked = subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard', FK + '/src'],
                                        cwd=root, text=True).split()
    print('new files: %s' % untracked)
    ok = problems == 0 and all(p.endswith('retained/seeded.rs') for p in untracked)
    print('OK' if ok else 'DIFFERENCES')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
