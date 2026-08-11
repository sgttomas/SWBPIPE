# Deterministic archive operation evidence — TM-PIP-037

Status: `PASS — EXACTLY ONE ROW RELOCATED`

## Pre-archive mutation validation

After the owner-ruled eight-field mutation, the live register validator
returned:

```text
taskmgmt validate PASS: projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv — 33 row(s), schema columns and referential rules conform. Form only; content judgment stays human (PRD §9.3).
```

Independent comparison against the frozen 33-row basis found exactly one
changed row, `TM-PIP-037`, and exactly the eight authorized fields.

## Registered helper dry-run

Command:

```text
python3 -B tools/taskmgmt/taskmgmt.py archive --register projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv --dry-run
```

Exact result:

```text
taskmgmt archive DRY-RUN: 1 CLOSED row(s) would move projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv -> projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv
  live after: OPEN=9 DEFERRED=23 ELEVATED=0 CLOSED=0 (32 row(s)); archive total: 8
  mechanical relocation only; closure itself remains the owner's recorded act (K-TM-3), and archived rows stay part of the register's federated identity
```

The prediction was exactly one move, satisfying the owner ruling's archive
condition.

## Registered helper execution

Command:

```text
python3 -B tools/taskmgmt/taskmgmt.py archive --register projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv
```

Exact result:

```text
taskmgmt archive COMPLETE: 1 CLOSED row(s) moved projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv -> projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv
  live after: OPEN=9 DEFERRED=23 ELEVATED=0 CLOSED=0 (32 row(s)); archive total: 8
  mechanical relocation only; closure itself remains the owner's recorded act (K-TM-3), and archived rows stay part of the register's federated identity
```

Both resulting registers then validated `PASS`. No helper was directed at any
foreign register, receipt, evidence, decision, lifecycle, deliverable, or
other project surface.
