"""Resolve original Chirality revisions through the explicit extraction table.

Original identities remain documentary facts. These helpers select equivalent
local Git inputs; they never fetch objects or substitute another revision. The
lookup of a revision that may be an original identity runs with
--no-lazy-fetch, so a partial clone does not ask its promisor remote for it.
"""
import re
import subprocess

# The original Chirality identities that maintained tests and retained pricing still cite: each with its extracted
# commit in this repository and the project tree recorded for the original. An original identity outside this table
# is refused; cite the extracted commit instead of adding a row for new work.
EXTRACTED = {
    '116149085a523678cc8247b81e9c367bd08d4dcc': ('b5c073cb0caef1bdaff6371a6de3db75abf58d3c', '644a83d05a6ccbed54a3b1e13ae261fa85700bd3'),
    '2243be380ba6f9b45ef62ea25d2ee0dd968b2a25': ('51eb3bcb31279d1e4e99dcd08312e9455773665e', '97eb89f0dca06a1a5f23432c0d6c2bc399f3ea8f'),
    '2653f8739262251d9fa2cc339f03422f8fea9052': ('93dd6b75b437e36bc23e1765ef06aa7ba268b7ed', 'b974801d40e829c6a8ee43a910dd12c84e46298e'),
    '2844f2b32d4b9f30effc41b4b387a302417697dc': ('330a0edefc886e07b6d8c2c85817e3fffe7d0f30', 'a3ef33b49fd0e1afea590ced195bb8de600fa6c2'),
    '3027cf2f356adf85ad3499052dea7ca3074b4a57': ('76e8f6f4b248743f2d494f507bdb30a841c74138', '451f666dc746989345de561c995c6e26daaf62cc'),
    '3826a6288173928ed32a554552fec72eb8ff3dd2': ('a1df578af695db308aef529c4ce8dc225595d325', '9f2e03ab3ba3542f0da3036bc8fa138349c998b4'),
    '471ea86a7ff849baed50a0032202ff4708829df1': ('f746133f864352ebc6f401c80e1b209ea85cc617', '1f537d1a422cf288c3a119b223f2c8d106f1ea82'),
    '71300752ab86b86c568c24e308eb5a45761c924a': ('a90b44f29992c844638737ae391e65749a8201d1', 'c391d501793ef03133c7c650d2b6b7be3ef2f206'),
    '7746774e81114e8fdd998ccf0bed12c46091ce45': ('015678cb847b3dcf97c813e8cf82eafcb9401e79', 'c465dffa53fec802b3f7d89824ce82fac6a8b44b'),
    '80cf05a8575dd53dca4f058d90d835d310cc2228': ('ab238ca79c725bc8d858fe99ffb0867349202610', 'b23d81d0923b0b0728c29c49542f358aa4fa22a1'),
    '9bfd04c25550c8600a5ae3fec55e008337e97b3a': ('a5ef9fd7b7bccac02ee6362f7f8aeff1d114d08a', '49956ebe98c958da33518aa08cd50136d315442d'),
    '9e03991d392f07b578f9fac4e5e39d8d289580ea': ('d93ada7f4b15bde60430399eca70a2f7f0233e90', '436d29674ff50474c7e50b32eb3357c69dcce72f'),
    'a467a1f03799f12ebd02df82a0145cfc958f9dcb': ('0d68ad23150485d558ed485753119501caed5874', '1c15efa9eb8919e57cef2d62bf0d302f2493331d'),
    'a9524745a3227ff229e22bf6aba9b307e0382701': ('71e7ebb33f7da2fb742d19ed4f78026c9894da49', 'cadd90a57a02c0b559606da1d2e13d7e6227ea6e'),
    'ac07a34ccd45a83425a077f64564e92155db943f': ('2d90e18bb240baccda850fdcba0aa906841a3864', '4a70b2d723c966a913800e7bfddb105b7bdc5fad'),
    'b137b9c46a0a848a9284e5147391641845c83815': ('42996137fca4ea4798735edae7e447d65bfab486', '6d1eab7e679f809d0e5d8dbe90a30fb6616710b3'),
    'b4b26cdd82a195342ee4dea2be0d4a99b8ffab67': ('82296430153438988feda15f7fc8cb073d367922', 'f52e06de2b36b8ee73000df82bd21d75268000d8'),
    'be7d4749583b48aa7e2b430b771ac1baf19bc37f': ('7aac5ca364d8f67c40820a2359fc72bd0878b19e', 'bbeb5057107fe16d59110ad60bbcc4cba2f44fb0'),
    'c986759b21605293e29383dd2b660ab5acb7c3fb': ('014e3558abcf9329dd694a3b7d2d5face2540aae', '039abc143bea3874f264bcadcc310515b9e04c33'),
    'e2055cb35fe71af03caae2c386bd8055fb0afd81': ('c7a1d716ca3cc6aebd89abef8f2297b9ff3b2670', '5de7ef40c72df4084e310ef8294d56a5cf640f89'),
    'f1da661c6d4f681df2dabed06a65f16fc50ff623': ('b5dcc3b6891e55f80d41c79feeed7b72a62ed52f', '119188e67f11869e72800e3cf1779ddb8301e943'),
}


def resolve_revision(repo, revision, *, env=None, git_options=()):
    """`env` and `git_options` (global options placed before -C) default to the caller's git environment; the retained
    pricing tool passes its sanitized ones (tools/retained_pricing/lib/pinned_git.py)."""
    explicit = revision.startswith('source:')
    source = revision.removeprefix('source:')
    if not explicit:
        # An original identity is usually absent here. In a partial (promisor) clone git would try to fetch the
        # missing object from the promisor remote on every call (hosted CI checks out with filter: blob:none);
        # --no-lazy-fetch (Git 2.45 or later) keeps the lookup local. Exit 128 (git's fatal exit: a revision not
        # found here, but also, for example, a directory that is not a repository) keeps the existing fall-through to
        # the extraction table, whose tree check still guards success. Any other failure, including a git that
        # rejects the option (129), is refused, not mapped.
        result = subprocess.run(['git', *git_options, '--no-lazy-fetch', '-C', str(repo), 'rev-parse', '--verify',
                                 revision + '^{commit}'], capture_output=True, text=True, env=env)
        if result.returncode == 0:
            return result.stdout.strip()
        if result.returncode != 128:
            raise ValueError('Offline revision lookup failed (git exit %d; --no-lazy-fetch needs Git 2.45 or later): %s'
                             % (result.returncode, result.stderr.strip()))
    if not re.fullmatch('[0-9a-fA-F]{7,40}', source):
        raise ValueError('Unknown source revision: ' + source)
    matches = [(old, new) for old, (new, _) in EXTRACTED.items() if old.startswith(source.lower())]
    if len(matches) != 1:
        raise ValueError('Source revision missing or ambiguous: ' + source)
    result = subprocess.run(['git', *git_options, '-C', str(repo), 'rev-parse', '--verify',
                             matches[0][1] + '^{commit}'], capture_output=True, text=True, env=env)
    if result.returncode:
        raise ValueError('Mapped source commit unavailable: ' + source)
    expected = EXTRACTED[matches[0][0]][1]
    tree = subprocess.run(['git', *git_options, '-C', str(repo), 'rev-parse', '--verify',
                           result.stdout.strip() + '^{tree}'], capture_output=True, text=True, env=env)
    if tree.returncode or tree.stdout.strip() != expected:
        raise ValueError('Mapped source tree differs from recorded original: ' + source)
    return result.stdout.strip()


def project_prefix(repo, revision, *, env=None, git_options=()):
    for prefix in ('', 'projects/chirality-piping/'):
        result = subprocess.run(['git', *git_options, '-C', str(repo), 'cat-file', '-t',
                                 revision + ':' + prefix + 'core'], capture_output=True, text=True, env=env)
        if result.returncode == 0 and result.stdout.strip() == 'tree':
            return prefix
    raise ValueError('Revision has no SWBPIPE source tree: ' + revision)


def source_spec(repo, revision, path):
    """Translate a historical Git address while retaining its original citation."""
    resolved = resolve_revision(repo, revision)
    relative = path.removeprefix('projects/chirality-piping/').removeprefix('./')
    return resolved + ':' + project_prefix(repo, resolved) + relative
