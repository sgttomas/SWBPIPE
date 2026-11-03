# Criterion 6 and the reference system: a note for the owner

**Status: a decision note for the owner. No choice is requested now.** It applies Agent 0's
decision (d), 2026-11-02 (paraphrase), from `caepipe-mbf-import.md` §20.5.14, referred to below
as "the Design". It changes no code, criterion, family list or limitation.

**The criterion** (`docs/RELEASE_TARGET_v1.0.md` §4, exit criterion 6, **MBF round trip**):

> - The desktop saves the reference system as MBF with no Must family omitted
>   or unsupported in the loss report.
> - The desktop imports it back with the same entity identities.

A-3 (`desktop-mbf-export-import.md` §8.1) reads "Must family" as every family the reference
system holds under criterion 1, plus its cases and combinations. A family with no MBF record
goes to criterion 3's limitations list, which only the owner accepts.

## Combinations that cannot be written today

The fixture (`fixtures/reference_system/reference_system.json` at `2bf0575b1`) holds two
combinations, both `result_state_subtraction` with no `difference_basis`:

- **RS-EXP**, `load:RS-OPE` − `load:RS-SUS`: the expansion term.
- **RS-dSAM**, `load:RS-SAM` − `load:RS-SUS`: the anchor-movement term. Its minuend RS-SAM is
  itself refused (below).

The MBF has no combination record, so both are **omitted** (`MBF-SI-COMBINATION-NOT-EXPORTED`).
With a recorded `difference_basis` (DEL-09-01 §8.2), RS-EXP would instead hit the blocking
`MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED`. No correspondence with the target's thermal case is
claimed: that would need G-5, G-2 and G-10 (Design §20.5.4 d). Sidecar ids would carry identity
only, not the combinations' mechanics or operand definitions (§20.5.13). DEL-09-01 §8.2's other
combinations (RS-dWIN, RS-dEQX/Y/Z, RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS) are pending,
not in the fixture, and would be omitted the same way (DEL-09-01 §11.1).

## Other blocking families

Details are in Design §20.5.10, desktop §1.4 and DEL-09-01 §11.1.

- **RS-P180:** `MBF-SI-MODULUS-SELECTION-MISMATCH` (§20.4.8). It waits on RT-K4.
- **RS-SAM:** a second sustained-type case (`MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED`), with
  anchor motion (`MBF-SI-BOUNDARY-MOTION-UNSUPPORTED`).
- **RS-SUS beside RS-M1's varying modulus:** `MBF-SI-SUSTAINED-MODULUS-UNPROVED` (G-2).
- **RS-OPE:** tier O-B, proposed only. Refused with
  `MBF-SI-ROUTE1-MODULUS-SELECTION-UNESTABLISHED` (G-1, G-4); it also waits on G-7, G-3 and
  G-8. The varying table itself is refused with
  `MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED` (Route 1 is not implemented).
- **Weight form:** `distributed_force` along `global_z`, and component weights at nodes
  (`MBF-SI-LOAD-UNSUPPORTED`).
- **Tee and connector:** RS-T1 (`MBF-SI-COMPONENT-UNSUPPORTED`, RT-K2); RS-EJ1 (RT-K4) and its
  region (`MBF-SI-PRESSURE-UNSUPPORTED`). The valve and flanges are refused too (RT-K3).
- **Hanger:** RS-VS1 is refused as a `hanger`. The families still pending in the fixture are
  refused once present.

## Consequence and recommendation

Under option 1 of §20.5.13 (criterion 6 as written, with A-3's family list), **no full
reference-system claim can be made today**. No route in the Design meets the criterion.

**Recommended: option 1, while the missing evidence and coverage are pursued.** First come the
supplier questions in DEL-17-01 `Work/owner-supplier-questions-operating-OA.md`: three for
O-A, then G-7, G-1, G-4, G-2 and G-5, with G-10 held. The coverage tranches of Design
§20.5.14, item 7, follow. This is not acceptance of the omissions, and it is not a reduced
family list. Sidecar ids support identity only and never carry mechanics. Source clarification
could still open a path for RS-EXP: if admitted evidence established G-5, G-2 and G-10, the
target's own case formation might carry its mechanics. No design proposes that today.

## Not requested now

**No scope choice is requested of the owner now.** Accepting a limitation (criterion 3; option 2)
and amending the criterion or A-3's family list (option 3) are the owner's decisions alone. No
agent makes or assumes either, and this note creates or proposes no release limitation.
