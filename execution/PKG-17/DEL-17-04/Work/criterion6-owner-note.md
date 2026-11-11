# Criterion 6 and the reference system: a note for the owner

**Status: owner decision recorded, 2026-11-08.** The owner chose option 3, "Family list
amended", of `caepipe-mbf-import.md` §20.5.13 ("the Design"). A-3 in
`desktop-mbf-export-import.md` §8.1 holds the amended definition; release target exit
criterion 6 holds its governing statement. The earlier option-1 recommendation under
Agent 0's decision (d), 2026-11-02 (paraphrase), is superseded. No code or accepted
limitation follows from this record.

**The criterion** (`docs/RELEASE_TARGET_v1.0.md`, exit criterion 6, **MBF round trip**):

> - The desktop saves the reference system as MBF with no Must family omitted
>   or unsupported in the loss report.
> - The desktop imports it back with the same entity identities.

A-3's owner amendment excludes combinations, which combine solved results, from criterion 6's
families; RT-A4 coverage is judged elsewhere. It does not change the writer's omission or
refusal accounting. The other refusals in Design §20.5.10 still stand, so criterion 6 remains
unmet. No reference-system round-trip claim is made. Any remaining family with no MBF record
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

## Consequence of the owner decision

The owner chose option 3 on 2026-11-08, rather than accepting the combinations as release
limitations under option 2. Excluding combinations from criterion 6 does not resolve the other
blocking families above. No route in the Design meets the amended criterion for the reference
system; no full reference-system claim can be made.

## Earlier recommendation, superseded

**Agent 0's 2026-11-02 basis recommended option 1 while missing evidence and coverage were
pursued. The owner's 2026-11-08 choice of option 3 supersedes that recommendation.** The
supplier questions remain in DEL-17-01 `Work/owner-supplier-questions-operating-OA.md`: three for
O-A, then G-7, G-1, G-4, G-2 and G-5, with G-10 held. The coverage tranches of Design
§20.5.14, item 7, follow. Sidecar ids support identity only and never carry mechanics. The
historical source question for RS-EXP remains separate: admitted evidence establishing G-5,
G-2 and G-10 could support the target's own case formation, but no design proposes that today.

## Reserved decisions

No further scope choice is requested by this note. Acceptance of any release limitation
remains the owner's act; this amendment accepts none and creates or proposes no release
limitation.
