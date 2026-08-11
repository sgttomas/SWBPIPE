# Application Path Manifest

Status: `FINAL MANAGER FAN-IN`

Final candidate inventory is 153 paths: 68 tracked modifications and 85
nonignored untracked files. The latter comprise one C-B record, ten V-D
evidence files, one DEL-09-04 run record, and 73 governed run-root records.

Specialist manifests/hashes:

- C-B policy: `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`;
- V-D manifest: `a08a738634155b01be83a04f2777bfcbbb131246b934ebda24eaf9ad4860f385`;
- MR-A changed-path manifest:
  `15e1426e837b05314ece932d4b72d3952b446d11f8e538a2672cd3f631364919`;
- V2 terminal return:
  `d4c791c3c667b23d7ecd0a7628c2383593fe3f191426760db9a08d2846b11ec4`.

Final repository-relative NUL-delimited sorted path-name SHA-256:
`bcece3d33d15e7632bc3fd8dd050fc869cc50628ac56a7a270ac82fcdcc2756b`.

Final sorted `path + NUL + file SHA-256 + LF` content-manifest SHA-256,
excluding this self-referential manifest only:
`a0293ac20bcd9a8c354281048cbbed9e8c68288f138c5ecf8d8d39acc544ce78`.

Terminal checks: 153/153 contained, candidate whitespace PASS,
`git diff --check` PASS, first Remaining bullet byte-identical, ignored paths
zero, staged paths zero. Path-anchor scan checked 1,293 live surfaces and
found exactly the two preserved V1 brief findings; no V2 or manager-closeout
finding was introduced.
