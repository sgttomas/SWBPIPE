# Preparation tooling and harness validation

The original parent attempt returned operational exit 2 because PyYAML was not importable. No governed validation failure was inferred from that missing dependency.

A fresh temporary virtual environment was created from the available Python 3.13 interpreter, using all exact pins in `projects/chirality-piping/requirements-dev.txt` plus the unpinned numpy dependency declared by `.github/workflows/governance-harness.yml`. CI currently selects Python 3.12; this local execution used **Python 3.13.14**, explicitly recorded rather than presented as identical to CI. No global package installation or dependency-file edit occurred. The exact installation commands, discovery failures, environment location and resolved versions are preserved in `ENVIRONMENT.json`, `environment-install.log` and `environment-versions.log`.

With the environment bin directory prepended to PATH and `PYTHONDONTWRITEBYTECODE=1`, the registered commands ran unchanged from repository root:

- `python3 tools/practitioner_harness/harness.py self-check`: exit **0**; INFO=14, NOT_APPLICABLE=1, REVIEW=4, WARN=55.
- `python3 -m pytest -q tools/practitioner_harness`: exit **0**; **379 passed in 35.09s**.

These are preparation-time checks against the lane at base `35249accf139f52478d029458946e50ed25ee5dc` plus in-flight untracked preparation evidence. They do not bind a later completed/frozen tranche or constitute product numerical verification. Rerun final required checks after the preparation writers freeze. Full logs and UTC execution times are in `HARNESS_RESULTS.json` and its referenced logs. The temporary environment may be reused in this session; it is not durable project infrastructure.
