"""Check the rendered Ubuntu source contract without APT or host writes."""
import os
from pathlib import Path
import re
import subprocess
import tempfile
import textwrap
import unittest


ACTION = Path(__file__).resolve().parents[3] / '.github/actions/setup-piping-e2e/action.yml'


def run_blocks(action):
    """Read this action's indented bash run scalars using only the stdlib."""
    return [textwrap.dedent(match.group(1)) for match in re.finditer(
        r'^      run: \|\n((?:        .*\n|\n)+)', action, re.MULTILINE)]


class UbuntuSourceTests(unittest.TestCase):
    def setUp(self):
        self.action = ACTION.read_text()
        self.blocks = run_blocks(self.action)
        self.configure = next(block for block in self.blocks if 'ubuntu.sources' in block)

    def test_rendered_sources_preserve_trust_and_request_by_hash(self):
        # Execute the actual expandable heredoc with a fixture codename. Only
        # replace its privileged output command; do not execute the host setup.
        match = re.search(r'^sudo tee /etc/apt/sources.list.d/ubuntu.sources >/dev/null <<EOF\n'
                          r'(.*?)^EOF$', self.configure, re.MULTILINE | re.DOTALL)
        self.assertIsNotNone(match)
        with tempfile.TemporaryDirectory() as directory:
            result = subprocess.run(
                ['bash', '--noprofile', '--norc'],
                input='set -euo pipefail\ncat <<EOF\n' + match.group(1) + 'EOF\n',
                text=True, capture_output=True, check=True, cwd=directory,
                env={**os.environ, 'VERSION_CODENAME': 'noble'})
        stanzas = [dict(line.split(': ', 1) for line in stanza.splitlines())
                   for stanza in result.stdout.strip().split('\n\n')]
        common = {
            'Types': 'deb', 'Components': 'main restricted universe multiverse',
            'By-Hash': 'force',
            'Signed-By': '/usr/share/keyrings/ubuntu-archive-keyring.gpg',
        }
        # Exact allowed fields also reject trust/expiry/insecure overrides.
        self.assertEqual(stanzas, [
            {**common, 'URIs': 'https://archive.ubuntu.com/ubuntu/',
             'Suites': 'noble noble-updates noble-backports'},
            {**common, 'URIs': 'https://security.ubuntu.com/ubuntu/',
             'Suites': 'noble-security'},
        ])

    def test_bash_syntax_and_bounded_diagnostics(self):
        for block in self.blocks:
            subprocess.run(['bash', '-n'], input=block, text=True, check=True,
                           capture_output=True)
        diagnostic = next(line for line in self.configure.splitlines()
                          if line.startswith('apt-config '))
        self.assertEqual(diagnostic.split(), [
            'apt-config', 'shell', 'BY_HASH', 'Acquire::By-Hash',
            'PACKAGES_BY_HASH', 'Acquire::IndexTargets::deb::Packages::By-Hash',
            'DEP11_BY_HASH', 'Acquire::IndexTargets::deb::DEP-11::By-Hash',
            'HTTP_NO_CACHE', 'Acquire::http::No-Cache',
            'HTTPS_NO_CACHE', 'Acquire::https::No-Cache',
        ])

    def test_both_playwright_cache_paths_remain_available(self):
        steps = self.action.split('    - name: ')
        paths = {}
        for step in steps:
            condition = re.search(r"^      if: steps.playwright-cache.outputs.cache-hit (==|!=) 'true'$",
                                  step, re.MULTILINE)
            if condition:
                command = re.search(r'^      run: (.+)$', step, re.MULTILINE)
                self.assertIsNotNone(command)
                paths[condition.group(1)] = command.group(1)
                self.assertIn('      working-directory: projects/chirality-piping', step)
        self.assertEqual(paths, {'==': 'npx playwright install-deps chromium',
                                 '!=': 'npx playwright install --with-deps chromium'})


if __name__ == '__main__':
    unittest.main()
