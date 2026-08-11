# Deterministic archive operation evidence — TM-PIP-040

Status: `PASS — EXACTLY ONE ROW RELOCATED`

## Pre-archive validation

After the exact eight-field owner-ruled mutation, the live register validated:

```text
taskmgmt validate PASS: projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv — 34 row(s), schema columns and referential rules conform. Form only; content judgment stays human (PRD §9.3).
```

An independent row comparison against `HEAD` found exactly one semantic row
delta, `TM-PIP-040`, and exactly the eight fields named by the proposal.

## Registered helper dry-run

Command:

```text
python3 tools/taskmgmt/taskmgmt.py archive --register projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv --dry-run
```

Exact result:

```text
taskmgmt archive DRY-RUN: 1 CLOSED row(s) would move projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv -> projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv
  live after: OPEN=9 DEFERRED=24 ELEVATED=0 CLOSED=0 (33 row(s)); archive total: 7
  mechanical relocation only; closure itself remains the owner's recorded act (K-TM-3), and archived rows stay part of the register's federated identity
```

The prediction was exactly one move, so execution was permitted by the owner
ruling and proposal manifest.

## Registered helper execution

Command:

```text
python3 tools/taskmgmt/taskmgmt.py archive --register projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv
```

Exact result:

```text
taskmgmt archive COMPLETE: 1 CLOSED row(s) moved projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv -> projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv
  live after: OPEN=9 DEFERRED=24 ELEVATED=0 CLOSED=0 (33 row(s)); archive total: 7
  mechanical relocation only; closure itself remains the owner's recorded act (K-TM-3), and archived rows stay part of the register's federated identity
```

Both resulting registers then validated `PASS`. No helper was directed at any
foreign register, receipt, evidence, decision, lifecycle, or deliverable
surface.
