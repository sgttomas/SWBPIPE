# SCA-013 — Modal analysis

Status: **DECIDED** and executed in place. Agent 0 (HELP_HUMAN) decided it on 2026-10-27 under the owner's delegation. Its commitment and acceptance text is pending the independent review that `loop/LOOP_INIT.md` requires for changed commitment text. The change re-enters the v0.1 FR-024 dynamics scope, which DEC-078 (D-12, owner 2026-07-15) explicitly deferred, for **modal analysis only**: natural frequencies and mode shapes. DEC-078 required that "any future dynamics scope re-enters only through a new governed scope change with its own human decision packet"; this is that change.

## Authority

The owner's delegation, recorded in `loop/LOOP_INIT.md` (#1662):

> "At this point we're hill-climbing towards an app that would be on par with or considered superior to CAEPIPE and other industry standard applications in this domain, in terms of the modelling characteristics and the user interface, so if there's a change or expansion in scope that would clearly benefit that kind of hill climb then we want to do it and you can decide under the extension of the same standing delegation authorization. If it fixes a mistake, if it plugs an obvious gap, if it's about making something more accurate, truthful, or capable without considerable risks of any sort, you can take initiative and drive scope towards a successful completion of the first publications ready version of SWBPIPE." (owner, 2026-10-27)

> "Yes, decide them too" (owner, 2026-10-27, on reversing earlier deferrals)

Agent 0 (HELP_HUMAN) decided this change under that delegation on 2026-10-27. Agent 0's reason: "The owner's 2026-10-27 delegation (#1662, LOOP_INIT) explicitly covers reversing earlier deferrals ("Yes, decide them too") and scope that clearly serves parity with industry tools. Modal analysis is a baseline industry capability, additive, and refused by name outside its domain. That is the owner's human decision, delegated."

## The amendment

| Surface | Before | After |
|---|---|---|
| `docs/PRD.md` §11.4 | FR-SOL-001 to FR-SOL-015; no dynamics requirement | FR-SOL-016 "Support modal analysis" (Should) |
| `docs/PRD.md` §12.1 | Later modules may add geometric nonlinearity, dynamics, or specialized component behavior. | Modal analysis of the same linear model is specified in §12.9. Later modules may add geometric nonlinearity, further dynamics, or specialized component behavior. |
| `docs/PRD.md` §12.9 | — | New: Modal Analysis |
| `docs/PRD.md` §23, MVP exclusions | advanced nonlinear/dynamic modules unless otherwise prioritized. | advanced nonlinear/dynamic modules unless otherwise prioritized (modal analysis, §12.9, is prioritized by SCA-013; response-spectrum and time-history modules are not). |
| `docs/PRD.md` header and §2 amendment note | — | amended 2026-10-27 by SCA-013 |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | revision 0.15 | revision 0.16: scope item SOW-081 (OBJ-003, PKG-04) and Deliverable DEL-04-09 "Modal analysis"; OI-009 narrowed to the modules still deferred; DEC-078 amended in place to record the re-entry |
| `execution/PKG-04/DEL-04-09/` | — | New: `ScopeOfWork.md`, `deliverable.yaml`, `Design/modal-analysis.md` |
| `docs/COMPLETENESS_MAP.md` | no modal row | FR-SOL-016 row (P, not built); F1 on the remaining units, with the mass-basis finding beside the deferred PF seismic-mass alignment |

`execution/_Coordination/_DECISIONS/D-12_fr024_fr025_disposition.md` is historical and unchanged. FR-025 (local FEA export) is untouched.

## What enters scope, and what does not

- **In scope:** modal analysis of the linear model: the generalized eigenproblem of the restraint-reduced stiffness and a mass matrix on the self-weight basis. A mode count, a frequency cut-off or both is required as user input, with no default. Each mode's frequency, period, mass-normalized shape, translational participation factors and effective masses are published. Each extracted set gets integrity checks, a Sturm count for completeness among them, and the result refuses by name what it does not admit. The domain, decisions and sequence are in DEL-04-09's ScopeOfWork and Design.
- **Not in this amendment:** response spectrum (F2), time history (F3), harmonic analysis and every other dynamic module. They stay deferred under DEC-078. Rotational participation, nonlinear supports linearized at a static state, Timoshenko mass (M4) and a concentrated-mass model field are later slices or later changes.
- **Unchanged:** every other scope item, Deliverable mapping, dependency meaning and lifecycle state. The kernel, element, support, diagnostics, self-weight, component and solve-integration Deliverables keep their scope and are DEL-04-09's declared inputs. No lifecycle, release-readiness, professional, certification or code-compliance claim follows.

## Consumer

DEL-04-09's sequence (Agent 0, 2026-10-27) is:
1. M1a, mass;
2. M1b, the dense path;
3. M2, the sparse path, with Sturm and inertia certification;
4. M3, UI.

The build starts after PF Timoshenko (#1663) is in review, and D3 runs join the queue on Agent 0's go. The governing design is [DEL-04-09 Design, modal-analysis.md](../../PKG-04/DEL-04-09/Design/modal-analysis.md).

## Method

As with SCA-012, the `scope-change` workflow's grouped checkpoints are not used. The change is one decided scope statement and its governed text, edited in place under the repository's current record rule (AGENTS.md: a record needs a concrete consumer; decisions edit the text they govern). Unlike SCA-012, it has a decomposition effect: one scope item and one Deliverable, added as revision 0.15 added DEL-04-08. `_ScopeChange/_LATEST.md` now selects this amendment.

Standard claim fence applies (claims taxonomy per DEC-081).
